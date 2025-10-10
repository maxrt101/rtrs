use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TaskState {
    Ready,
    Pending,
    Paused,
    Done,
}

impl TryFrom<u8> for TaskState {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == TaskState::Ready as u8   => Ok(TaskState::Ready),
            x if x == TaskState::Pending as u8 => Ok(TaskState::Pending),
            x if x == TaskState::Paused as u8  => Ok(TaskState::Paused),
            x if x == TaskState::Done as u8    => Ok(TaskState::Done),
            _ => Err(()),
        }
    }
}

impl Into<u8> for TaskState {
    fn into(self) -> u8 {
        self as u8
    }
}

impl core::fmt::Display for TaskState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TaskState::Ready    => f.write_str("Ready"),
            TaskState::Pending  => f.write_str("Pending"),
            TaskState::Paused   => f.write_str("Paused"),
            TaskState::Done     => f.write_str("Done"),
        }
    }
}

pub struct TaskControlBlock {
    pub state: AtomicU8,
}

impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(TaskState::Ready.into()),
        }
    }

    pub fn pend(&self) {
        // TODO: Can't pend if paused
        self.state.store(TaskState::Pending.into(), Ordering::SeqCst);
    }

    pub fn ready(&self) {
        // TODO: Can't run if paused
        self.state.store(TaskState::Ready.into(), Ordering::SeqCst);
    }

    pub fn done(&self) {
        self.state.store(TaskState::Done.into(), Ordering::SeqCst);
    }

    pub fn pause(&self) {
        // TODO: Can't pause if done
        self.state.store(TaskState::Paused.into(), Ordering::SeqCst);
    }

    pub fn resume(&self) {
        self.state.store(TaskState::Ready.into(), Ordering::SeqCst);
    }

    pub fn get_state(&self) -> TaskState {
        TaskState::try_from(self.state.load(Ordering::Acquire)).unwrap()
    }

    pub fn is_state(&self, s: TaskState) -> bool {
        s == self.get_state()
    }
}
