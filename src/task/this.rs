use crate::sync::RwLock;
use heapless::Deque;

pub(crate) struct GlobalContext {
    pub should_cancel: bool,
    pub new_tasks: Deque<super::Task<'static, ()>, 4>,

}

impl GlobalContext {
    const fn new() -> Self {
        Self {
            should_cancel: false,
            new_tasks: Deque::new(),
        }
    }
}

pub(crate) static GLOBAL: RwLock<GlobalContext> = RwLock::new(GlobalContext::new());

pub fn cancel() {
    let mut global = GLOBAL.lock_mut();

    (*global).should_cancel = true;
}

pub fn spawn(task: super::Task<'static, ()>) {
    let mut global = GLOBAL.lock_mut();

    let _ = (*global).new_tasks.push_back(task);
}

pub unsafe fn reset() {
    let mut global = GLOBAL.lock_mut();

    (*global).should_cancel = false;
    (*global).new_tasks.clear();
}
