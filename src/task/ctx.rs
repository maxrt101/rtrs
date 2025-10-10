use core::sync::atomic::{AtomicBool, Ordering};

pub struct ExecutionContext {
    should_run: AtomicBool,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            should_run: AtomicBool::new(true),
        }
    }
    
    pub fn should_run(&self) -> bool {
        self.should_run.load(Ordering::Relaxed)
    }
    
    pub fn set_should_run(&self, should_run: bool) {
        self.should_run.store(should_run, Ordering::Relaxed);
    }
}
