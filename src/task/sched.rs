use crate::task::{Task, TaskState};
use crate::task::tcb::TaskFlags;

extern crate alloc;

use core::task::Poll;

type SchedulerStorage<T> = alloc::vec::Vec<T>;

pub struct Scheduler {
    tasks: SchedulerStorage<Task<'static, ()>>,
    idle:  Option<Task<'static>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: SchedulerStorage::new(),
            idle:  None,
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

    pub fn attach(&mut self, task: Task<'static, ()>) {
        self.tasks.push(task);
        self.sort(true);
    }

    pub fn schedule(&mut self) {
        self.sort(self.sort_cond_check());
        
        let mut should_resort = false;
        let mut done = false;
        let mut all_blocked = true;

        for i in 0..self.tasks.len() {
            if self.tasks[i].is_state(TaskState::Ready) {
                all_blocked = false;
                
                unsafe { super::this::reset(); }

                match self.tasks[i].poll() {
                    Poll::Ready(_) => {
                        done = true;
                    }
                    Poll::Pending => {}
                }

                let mut global = super::this::GLOBAL.lock_mut();

                if (*global).should_cancel {
                    self.tasks[i].done();
                }

                while let Some(task) = (*global).new_tasks.pop_back() {
                    self.tasks.push(task);
                    should_resort = true;
                }
            }
        }

        unsafe { super::this::reset(); }

        if done {
            self.tasks.retain(|t| !t.is_state(TaskState::Done));
        }
        
        if should_resort {
            self.sort(true);
        }
        
        if all_blocked {
            if let Some(idle) = &mut self.idle {
                let _ = idle.poll();
            }
        }
    }

    pub fn run_to_completion(&mut self) {
        while !self.all_done() {
            self.schedule();
        }
    }
}

impl crate::object::Object for Scheduler {}
