use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Data submitted by a user via Telegram Passport.
pub struct PassportData {
    /// Array of encrypted passport elements.
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data.
    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An encrypted element containing Telegram Passport data.
pub struct EncryptedPassportElement {
    /// Element type (e.g. `"passport"`, `"email"`, `"phone_number"`).
    #[serde(rename = "type")]
    pub kind: String,
    /// Base64-encoded encrypted Telegram Passport element data.
    pub data: Option<String>,
    /// User's verified phone number.
    pub phone_number: Option<String>,
    /// User's verified email address.
    pub email: Option<String>,
    /// Array of encrypted files with documents.
    pub files: Option<Vec<crate::file::PassportFile>>,
    /// Encrypted file with the front side of the document.
    pub front_side: Option<crate::file::PassportFile>,
    /// Encrypted file with the reverse side of the document.
    pub reverse_side: Option<crate::file::PassportFile>,
    /// Encrypted file with a selfie of the user holding the document.
    pub selfie: Option<crate::file::PassportFile>,
    /// Array of encrypted files with translated versions of the document.
    pub translation: Option<Vec<crate::file::PassportFile>>,
    /// Base64-encoded element hash for error validation.
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Credentials required to decrypt and authenticate Telegram Passport data.
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON credentials.
    pub data: String,
    /// Base64-encoded data hash for authentication.
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key.
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source")]
/// An error in one of the fields provided by the user via Telegram Passport.
pub enum PassportElementError {
    /// Error in one of the data fields.
    #[serde(rename = "data")]
    DataField {
        /// Element type that has the error.
        r#type: String,
        /// Name of the data field with the error.
        field_name: String,
        /// Base64-encoded data hash.
        data_hash: String,
        /// Error message.
        message: String,
    },
    /// Error in the front side of the document.
    #[serde(rename = "front_side")]
    FrontSide {
        /// Element type that has the error.
        r#type: String,
        /// Base64-encoded file hash.
        file_hash: String,
        /// Error message.
        message: String,
    },
    /// Error in the reverse side of the document.
    #[serde(rename = "reverse_side")]
    ReverseSide {
        /// Element type that has the error.
        r#type: String,
        /// Base64-encoded file hash.
        file_hash: String,
        /// Error message.
        message: String,
    },
    /// Error in the selfie photo.
    #[serde(rename = "selfie")]
    Selfie {
        /// Element type that has the error.
        r#type: String,
        /// Base64-encoded file hash.
        file_hash: String,
        /// Error message.
        message: String,
    },
    /// Error in one of the uploaded files.
    #[serde(rename = "file")]
    File {
        /// Element type that has the error.
        r#type: String,
        /// Base64-encoded file hash.
        file_hash: String,
        /// Error message.
        message: String,
    },
    /// Error in a list of uploaded files.
    #[serde(rename = "files")]
    Files {
        /// Element type that has the error.
        r#type: String,
        /// Base64-encoded file hashes.
        file_hashes: Vec<String>,
        /// Error message.
        message: String,
    },
    /// Error in one of the translation files.
    #[serde(rename = "translation_file")]
    TranslationFile {
        /// Element type that has the error.
        r#type: String,
        /// Base64-encoded file hash.
        file_hash: String,
        /// Error message.
        message: String,
    },
    /// Error in a list of translation files.
    #[serde(rename = "translation_files")]
    TranslationFiles {
        /// Element type that has the error.
        r#type: String,
        /// Base64-encoded file hashes.
        file_hashes: Vec<String>,
        /// Error message.
        message: String,
    },
    /// Error in an unspecified place.
    #[serde(rename = "unspecified")]
    Unspecified {
        /// Element type that has the error.
        r#type: String,
        /// Base64-encoded element hash.
        element_hash: String,
        /// Error message.
        message: String,
    },
}
