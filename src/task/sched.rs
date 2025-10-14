use crate::task::{Task, TaskState};
use crate::task::tcb::TaskFlags;

extern crate alloc;

use core::task::Poll;

type SchedulerStorage<T> = alloc::vec::Vec<T>;

pub struct Scheduler<R = ()> {
    tasks: SchedulerStorage<Task<'static, R>>,
}

impl<R> Scheduler<R> {
    pub fn new() -> Self {
        Self {
            tasks: SchedulerStorage::new(),
        }
    }

    fn sort_cond_check(&self) -> bool {
        for task in self.tasks.iter() {
            if task.get_clear_flag(TaskFlags::PrioChanged) {
                return true;
            }
        }

        false
    }

    fn sort(&mut self, cond: bool) {
        if cond {
            self.tasks.sort_by(|a, b| {
                a.prio().cmp(&b.prio())
            });
        }
    }

    pub fn all_done(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn attach(&mut self, task: Task<'static, R>) {
        self.tasks.push(task);
        self.sort(true);
    }

    pub fn schedule(&mut self) {
        self.sort(self.sort_cond_check());

        let mut done = false;

        for task in self.tasks.iter_mut() {
            match task.poll() {
                Poll::Ready(_) => {
                    done = true;
                }
                Poll::Pending => {}
            }
        }

        if done {
            self.tasks.retain(|t| !t.is_state(TaskState::Done));
        }
    }

    pub fn run_to_completion(&mut self) {
        while !self.all_done() {
            self.schedule();
        }
    }
}

impl<R: 'static> crate::object::Object for Scheduler<R> {}
