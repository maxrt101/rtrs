mod provider;
mod timeout;
mod tick;

use crate::object_with;

pub use timeout::Timeout;
pub use tick::TickProvider;
pub use tick::GlobalTickProvider;
pub use tick::global_tick_provider;
pub use provider::TimeProvider;
pub use provider::TIME_OBJECT_NAME;

pub fn global_tick() -> u32 {
    object_with!(TIME_OBJECT_NAME, TimeProvider, time, {
        time.now()
    })
}

pub fn delay_ms(ms: u32) {
    let now = global_tick();

    while global_tick() - now < ms {
        // TODO: Atomic barrier?
    }
}

