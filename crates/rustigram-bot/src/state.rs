use dashmap::DashMap;
use std::any::{Any, TypeId};
use std::sync::Arc;

/// Thread-safe, type-erased key-value store for sharing state between handlers.
///
/// Backed by a `DashMap` so reads never block writes.
#[derive(Clone, Default)]
/// Thread-safe, type-keyed store for sharing data across handlers.
///
/// Values are stored by their [`TypeId`] so each type occupies exactly one
/// slot. Use this for things that are global to the bot — database pools,
/// configuration, shared counters.
///
/// `StateStorage` is cheap to clone (internally `Arc`-backed) and reads
/// never block writes.
///
/// # Example
///
/// ```rust,ignore
/// use rustigram_bot::state::StateStorage;
///
/// let store = StateStorage::new();
/// store.insert(42_u32);
/// store.insert(my_db_pool);
///
/// let n: u32 = store.get().unwrap(); // 42
/// ```
pub struct StateStorage {
    inner: Arc<DashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl StateStorage {
    /// Creates a new empty `StateStorage`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a value. Replaces any existing value of the same type.
    pub fn insert<T: Any + Send + Sync + 'static>(&self, value: T) {
        self.inner.insert(TypeId::of::<T>(), Arc::new(value));
    }

    /// Returns a clone of the stored value for type `T`, if present.
    pub fn get<T: Any + Send + Sync + Clone + 'static>(&self) -> Option<T> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<T>().cloned())
    }

    /// Returns an `Arc` reference to the stored value for type `T`.
    pub fn get_arc<T: Any + Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|v| Arc::clone(&*v).downcast::<T>().ok())
    }
}

/// Per-user or per-chat FSM state keyed by `(chat_id, user_id)`.
///
/// Stores the current dialogue state as a type-erased value so the state
/// machine type does not need to be known at the storage level.
#[derive(Clone, Default)]
/// Per-user conversation state store for finite state machines.
///
/// State is keyed by `(chat_id, user_id)` pairs. Any `'static` type can be
/// stored — no shared trait is required. Use this to track where a user is
/// in a multi-step conversation.
///
/// # Example
///
/// ```rust,ignore
/// use rustigram_bot::state::DialogueStorage;
///
/// #[derive(Clone)]
/// enum State { AwaitingName, AwaitingEmail { name: String } }
///
/// let storage = DialogueStorage::new();
/// storage.set(chat_id, user_id, State::AwaitingName);
///
/// match storage.get::<State>(chat_id, user_id) {
///     Some(State::AwaitingName) => { /* ask for name */ }
///     Some(State::AwaitingEmail { name }) => { /* ask for email */ }
///     None => { /* no active dialogue */ }
/// }
///
/// storage.remove(chat_id, user_id); // clear when done
/// ```
pub struct DialogueStorage {
    inner: Arc<DashMap<(i64, i64), Arc<dyn Any + Send + Sync>>>,
}

impl DialogueStorage {
    /// Creates a new empty `DialogueStorage`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the dialogue state for a `(chat_id, user_id)` pair.
    pub fn set<S: Any + Send + Sync + 'static>(&self, chat_id: i64, user_id: i64, state: S) {
        self.inner.insert((chat_id, user_id), Arc::new(state));
    }

    /// Returns the dialogue state for a `(chat_id, user_id)` pair.
    pub fn get<S: Any + Send + Sync + Clone + 'static>(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Option<S> {
        self.inner
            .get(&(chat_id, user_id))
            .and_then(|v| v.downcast_ref::<S>().cloned())
    }

    /// Removes the dialogue state for a `(chat_id, user_id)` pair.
    pub fn remove(&self, chat_id: i64, user_id: i64) {
        self.inner.remove(&(chat_id, user_id));
    }
}
