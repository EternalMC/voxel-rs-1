use crossbeam_channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;
use std::{collections::BTreeMap, sync::Arc, sync::RwLock};
lazy_static! {
    static ref DEBUG_INFO: Arc<RwLock<Option<Sender<DebugInfoUnit>>>> = Arc::new(RwLock::new(None));
}

#[derive(Debug, Clone)]
struct DebugInfoUnit {
    pub section: String,
    pub id: String,
    pub message: String,
}

/// Helper struct allowing multiple threads to easily show debug info.
/// There can only be one active `DebugInfo` at any time.
pub struct DebugInfo {
    receiver: Receiver<DebugInfoUnit>,
    sections: BTreeMap<String, BTreeMap<String, String>>,
}

impl DebugInfo {
    /// Create a new `DebugInfo` struct and make it the current one.
    pub fn new_current() -> Self {
        let (sender, receiver) = unbounded();
        *DEBUG_INFO.write().unwrap() = Some(sender);
        Self {
            receiver,
            sections: BTreeMap::new(),
        }
    }

    /// Get the debug info
    pub fn get_debug_info(&mut self) -> BTreeMap<String, BTreeMap<String, String>> {
        while let Ok(diu) = self.receiver.try_recv() {
            self.sections
                .entry(diu.section)
                .or_insert(BTreeMap::new())
                .insert(diu.id, diu.message);
        }
        self.sections.clone()
    }
}

/// Send debug info to the current `DebugInfo` if there is one
pub fn send_debug_info(section: impl ToString, id: impl ToString, message: impl ToString) {
    DEBUG_INFO.read().unwrap().as_ref().map(|sender| {
        sender
            .send(DebugInfoUnit {
                section: section.to_string(),
                id: id.to_string(),
                message: message.to_string(),
            })
            .unwrap()
    });
}
