//! Event ids for the server: UUIDv7, time-ordered so the events index stays compact.

use uuid::Uuid;
use zidara_kernel::ids::{EventId, IdGen};

#[derive(Clone, Copy, Debug, Default)]
pub struct UuidV7;

impl IdGen for UuidV7 {
    fn next_id(&mut self) -> EventId {
        EventId(Uuid::now_v7())
    }
}
