// Conversion queue - очередь для фоновой обработки файлов
// (Упрощённая заглушка, можно расширить в будущем)

use crate::error::Result;

pub struct ConversionQueue;

impl ConversionQueue {
    pub fn new() -> Self {
        Self
    }

    pub async fn enqueue(&self, _job_id: i32) -> Result<()> {
        // TODO: Implement queue logic
        Ok(())
    }
}
