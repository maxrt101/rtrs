extern crate alloc;
use alloc::boxed::Box;

pub trait TickProvider {
    type Tick;

    fn get_tick(&mut self) -> Self::Tick;
}

#[derive(Copy, Clone)]
pub struct GlobalTickProvider;

impl TickProvider for GlobalTickProvider {
    type Tick = u32;

    fn get_tick(&mut self) -> Self::Tick {
        super::global_tick()
    }
}

pub fn global_tick_provider() -> Box<dyn TickProvider<Tick = u32>> {
    Box::new(GlobalTickProvider {})
}
