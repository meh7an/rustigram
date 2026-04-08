use serde::{Deserialize, Serialize};

use crate::chat::Chat;
use crate::message::{MessageEntity, ParseMode};
use crate::user::User;

/// A single task in a checklist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task.
    pub id: i64,
    /// Text of the task.
    pub text: String,
    /// Special entities that appear in the task text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// User that completed the task; omitted if not completed by a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_user: Option<User>,
    /// Chat that completed the task; omitted if not completed by a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_chat: Option<Chat>,
    /// Unix timestamp when the task was completed; `0` if not completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<i64>,
}

/// A checklist message content type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checklist {
    /// Title of the checklist.
    pub title: String,
    /// Special entities in the checklist title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_entities: Option<Vec<MessageEntity>>,
    /// List of tasks in the checklist.
    pub tasks: Vec<ChecklistTask>,
    /// `true` if users other than the creator can add tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,
    /// `true` if users other than the creator can mark tasks as done or not done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}

/// A task to add when creating or editing a checklist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive and unique within the checklist.
    pub id: i64,
    /// Text of the task (1–100 characters after entities parsing).
    pub text: String,
    /// Parse mode for entities in the task text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Special entities in the task text; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

/// A checklist to send or create.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputChecklist {
    /// Title of the checklist (1–255 characters after entities parsing).
    pub title: String,
    /// Parse mode for entities in the title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Special entities in the title; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_entities: Option<Vec<MessageEntity>>,
    /// List of 1–30 tasks in the checklist.
    pub tasks: Vec<InputChecklistTask>,
    /// Pass `true` if other users can add tasks to the checklist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,
    /// Pass `true` if other users can mark tasks as done or not done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}

/// Service message: tasks in a checklist were marked as done or not done.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistTasksDone {
    /// Message containing the checklist whose tasks were updated.
    ///
    /// Boxed to break the recursive `Message → ChecklistTasksDone → Message` cycle.
    /// Will not contain `reply_to_message` even if the message itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<crate::message::Message>>,
    /// Identifiers of the tasks that were marked as done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_as_done_task_ids: Option<Vec<i64>>,
    /// Identifiers of the tasks that were marked as not done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_as_not_done_task_ids: Option<Vec<i64>>,
}

/// Service message: new tasks were added to a checklist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistTasksAdded {
    /// Message containing the checklist to which tasks were added.
    ///
    /// Boxed to break the recursive cycle. Will not contain `reply_to_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<crate::message::Message>>,
    /// The tasks that were added.
    pub tasks: Vec<ChecklistTask>,
}
