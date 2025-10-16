// Dialog states for FSM (Finite State Machine)
use serde::{Deserialize, Serialize};

/// Bot states for dialogs
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum State {
    #[default]
    Start,
    ReceivingTodoTitle,
    ReceivingTodoDescription { title: String },
    ReceivingReminderTime,
    ReceivingReminderText { time: String },
    ConvertingFile { file_id: String, file_type: String },
}
