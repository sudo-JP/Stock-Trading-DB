use crate::networking::protocols::CppTask;
use std::{collections::VecDeque, sync::{Arc, Mutex}}; 

pub struct CppWorkQueue {
    queue: Mutex<VecDeque<CppTask>>,
}

impl CppWorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(CppWorkQueue {
            queue: Mutex::new(VecDeque::new()),
        })
    }

    pub fn push(&self, task: CppTask) {
        self.queue.lock().unwrap().push_back(task);
    }

    pub fn pop(&self) -> Option<CppTask> {
        self.queue.lock().unwrap().pop_front()
    }
}
