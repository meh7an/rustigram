//! A mock Bot API server, so tests can assert what rustigram actually sends.
//!
//! Until now nothing exercised the transport at all: `post_json`,
//! `post_multipart`, the 429 retry, error mapping, and `download_file` had never
//! run under test. The multipart-drop bug — five settable options that never
//! reached Telegram on byte uploads — lived precisely there, and no amount of
//! type-level checking could have caught it. Only inspecting the bytes on the
//! wire can.
//!
//! [`spawn`] uses [`ClientConfig::api_base_url`], the same mechanism that points
//! rustigram at a local Bot API server, so nothing in the production code is
//! aware of the tests.
//!
//! Every server binds to an ephemeral local port. No test reaches the network.

#![allow(dead_code)] // each test binary uses a different subset

pub mod fixtures;

use rustigram_api::{BotClient, ClientConfig};
use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

/// The token every mock test uses. `BotClient` validates the format, and the
/// numeric prefix is what appears in request paths.
pub const TOKEN: &str = "123456:test-token-for-mock-tests";

/// Starts a mock server and a client pointed at it.
pub async fn spawn() -> (MockServer, BotClient) {
    let server = MockServer::start().await;
    let config = ClientConfig::new(TOKEN)
        .expect("the test token is well-formed")
        .api_base_url(server.uri());
    let client = BotClient::new(config).expect("client builds");
    (server, client)
}

/// The path a Bot API call lands on, e.g. `/bot123456:.../sendMessage`.
pub fn api_path(api_method: &str) -> String {
    format!("/bot{TOKEN}/{api_method}")
}

/// Mounts a successful `ok: true` response carrying `result`.
pub async fn mount_ok(server: &MockServer, api_method: &str, result: Value) {
    Mock::given(method("POST"))
        .and(path(api_path(api_method)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "ok": true,
            "result": result,
        })))
        .mount(server)
        .await;
}

/// Mounts a success response on every path, whatever the method.
///
/// For sweeps that call every builder in turn and care about the request rather
/// than the reply. Most calls will fail to decode the empty result, which is
/// fine and expected — the request is still recorded, and that is the subject.
pub async fn mount_catch_all(server: &MockServer) {
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "ok": true,
            "result": {},
        })))
        .mount(server)
        .await;
}

/// The Bot API method name a recorded request was sent to.
pub fn api_method_of(request: &Request) -> String {
    request
        .url
        .path()
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .to_owned()
}

/// Mounts an `ok: false` API error.
pub async fn mount_api_error(
    server: &MockServer,
    api_method: &str,
    code: u16,
    description: &str,
    parameters: Option<Value>,
) {
    let mut body = serde_json::json!({
        "ok": false,
        "error_code": code,
        "description": description,
    });
    if let Some(p) = parameters {
        body["parameters"] = p;
    }
    Mock::given(method("POST"))
        .and(path(api_path(api_method)))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(server)
        .await;
}

/// Mounts a raw body on a method's path, for responses that are not valid
/// `ApiResponse` JSON at all.
pub async fn mount_raw(server: &MockServer, api_method: &str, status: u16, body: &str) {
    Mock::given(method("POST"))
        .and(path(api_path(api_method)))
        .respond_with(ResponseTemplate::new(status).set_body_string(body))
        .mount(server)
        .await;
}

/// Mounts a `GET` returning raw bytes, for the file-download endpoint.
pub async fn mount_file(server: &MockServer, file_path: &str, bytes: &'static [u8]) {
    Mock::given(method("GET"))
        .and(path(format!("/file/bot{TOKEN}/{file_path}")))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(bytes))
        .mount(server)
        .await;
}

/// Answers the first `n` calls to a method with one response and the rest with
/// another, so a retry can be observed rather than inferred.
///
/// Registration order matters here: wiremock prefers the mock mounted first
/// among those still able to respond, so the scripted response must be mounted
/// before the fallback.
pub async fn mount_then(
    server: &MockServer,
    api_method: &str,
    first: Value,
    times: u64,
    then: Value,
) {
    Mock::given(method("POST"))
        .and(path(api_path(api_method)))
        .respond_with(ResponseTemplate::new(200).set_body_json(first))
        .up_to_n_times(times)
        .mount(server)
        .await;
    Mock::given(method("POST"))
        .and(path(api_path(api_method)))
        .respond_with(ResponseTemplate::new(200).set_body_json(then))
        .mount(server)
        .await;
}

/// A flood-control body, with the wait Telegram asks for.
pub fn flood_control(retry_after: u32) -> Value {
    serde_json::json!({
        "ok": false,
        "error_code": 429,
        "description": "Too Many Requests: retry after 1",
        "parameters": { "retry_after": retry_after },
    })
}

/// Every request the server received, in order.
pub async fn requests(server: &MockServer) -> Vec<Request> {
    server
        .received_requests()
        .await
        .expect("the mock server records requests")
}

/// The single request the server received, failing loudly on any other count.
///
/// "Exactly one" is usually the property under test — a retry that fired twice,
/// or a call that never happened, should fail here rather than silently pass a
/// body assertion against the wrong request.
pub async fn only_request(server: &MockServer) -> Request {
    let mut all = requests(server).await;
    assert_eq!(
        all.len(),
        1,
        "expected exactly one request, the server saw {}",
        all.len()
    );
    all.remove(0)
}

/// Parses a captured request body as JSON.
pub fn json_body(request: &Request) -> Value {
    serde_json::from_slice(&request.body).unwrap_or_else(|e| {
        panic!(
            "request body is not JSON: {e}\nbody was: {}",
            String::from_utf8_lossy(&request.body)
        )
    })
}

/// The field names present in a captured `multipart/form-data` body.
///
/// Deliberately a coarse scan of the part headers rather than a full multipart
/// parse: the property under test is *which fields were sent*, and that is what
/// drifted between the two send paths.
pub fn multipart_field_names(request: &Request) -> Vec<String> {
    let body = String::from_utf8_lossy(&request.body);
    // Split on `; name="`, not `name="` — the latter also matches the
    // `filename="..."` of a file part, which would report the uploaded file's
    // name as though it were a form field.
    let mut names: Vec<String> = body
        .split("; name=\"")
        .skip(1)
        .filter_map(|rest| rest.split('"').next())
        .map(str::to_owned)
        .collect();
    names.sort();
    names.dedup();
    names
}

/// Asserts a multipart request carries exactly these field names.
pub fn assert_multipart_fields(request: &Request, expected: &[&str]) {
    let actual = multipart_field_names(request);
    let mut want: Vec<String> = expected.iter().map(|s| (*s).to_owned()).collect();
    want.sort();
    assert_eq!(
        actual, want,
        "multipart fields differ\n  sent:     {actual:?}\n  expected: {want:?}"
    );
}

/// Asserts the JSON body contains `key` with `value`, naming the key on failure.
pub fn assert_field(body: &Value, key: &str, value: Value) {
    assert_eq!(
        body.get(key),
        Some(&value),
        "field `{key}`: expected {value}, body was {body}"
    );
}

/// Asserts the JSON body has no `key` at all.
///
/// Unset optional parameters must be absent, not `null` — Telegram treats an
/// explicit null differently from an omitted field for several methods.
pub fn assert_absent(body: &Value, key: &str) {
    assert!(
        body.get(key).is_none(),
        "field `{key}` should be absent when unset, body was {body}"
    );
}
