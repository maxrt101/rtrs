extern crate alloc;

use super::types::ChannelVec;

pub type SubscriberId = usize;
type SubscriberBitmap = usize;

pub struct Message<T: Copy + Clone> {
    received: SubscriberBitmap,
    data: T,
}

pub struct Channel<T: Copy + Clone> {
    subscribers: SubscriberBitmap,
    queue: ChannelVec<Message<T>>,
}

impl<T: Copy + Clone> Channel<T> {
    pub fn new() -> Self {
        Self {
            subscribers: 0,
            queue: ChannelVec::new(),
        }
    }

    fn find_free_subscriber(&self) -> Option<SubscriberId> {
        let mut i = 0;
        let mut subscribers = self.subscribers;

        while subscribers != 0 {
            subscribers <<= 1;
            i += 1;
        }

        if i >= size_of::<SubscriberId>() * 8 {
            None
        } else {
            Some(i)
        }
    }

    pub fn subscribe(&mut self) -> Option<SubscriberId> {
        self.find_free_subscriber()
    }

    pub fn unsubscribe(&mut self, subscriber: SubscriberId) {
        self.subscribers &= !(1 << subscriber);
    }

    pub fn send(&mut self, data: T) {
        if self.subscribers != 0 {
            let _ = self.queue.push(Message { data, received: 0 });
        }
    }

    fn check_remove(&mut self) {
        if let Some(last) = self.queue.last() {
            if last.received == self.subscribers {
                self.queue.pop();
            }
        }
    }

    pub fn recv(&mut self, id: SubscriberId) -> Option<T> {
        self.check_remove();

        let res =
            if let Some(last) = self.queue.last_mut() {
                if last.received & (1 << id) == 0 {
                    last.received |= 1 << id;

                    Some(last.data)
                } else {
                    None
                }
            } else {
                None
            };

        self.check_remove();

        res
    }
}

