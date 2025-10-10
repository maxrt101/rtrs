mod provider;
mod timeout;

use crate::object_with;

pub use timeout::Timeout;
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

