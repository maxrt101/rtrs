
pub struct Timeout {
    duration: u32,
    start: u32,
}

impl Timeout {
    pub fn new(duration: u32) -> Self {
        Self { duration, start: crate::time::global_tick() }
    }

    pub fn infinite() -> Self {
        Self { duration: u32::MAX, start: 0 }
    }

    pub fn reset(&mut self) {
        self.start = crate::time::global_tick();
    }

    pub fn extend(&mut self, duration: u32) {
        self.duration += duration;
    }

    pub fn expire(&mut self) {
        self.start = 0
    }

    pub fn expired(&self) -> bool {
        crate::time::global_tick() - self.start >= self.duration
    }

    pub fn elapsed(&self) -> u32 {
        crate::time::global_tick() - self.start
    }

    pub fn left(&self) -> u32 {
        if self.start + self.duration < crate::time::global_tick() {
            0
        } else {
            self.start + self.duration - crate::time::global_tick()
        }
    }
}
