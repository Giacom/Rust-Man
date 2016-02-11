use sfml::system as sf;
use units;

pub struct GameTime {
    clock: sf::Clock,

    pub start_time: units::MS,
    pub elapsed_time: units::MS,

    pub start_frame_time: units::MS,
    pub previous_frame_time: units::MS,

    pub delta_time: units::DT,
    pub fixed_time: units::DT,

    pub ticks: units::MS,
    pub fixed_ticks: units::MS,
    pub fps: i32,
}

impl GameTime {
    pub fn new() -> GameTime {
        let clock: sf::Clock = sf::Clock::new();

        GameTime {
            start_time: clock.get_elapsed_time().as_milliseconds(),
            elapsed_time: 0,

            start_frame_time: 0,
            previous_frame_time: clock.get_elapsed_time().as_milliseconds(),

            delta_time: 0.0,
            fixed_time: 0.0,

            ticks: 0,
            fixed_ticks: 0,
            fps: 0,

            clock: clock,
        }
    }

    pub fn get_time_in_ms(&self) -> units::MS {
        self.clock.get_elapsed_time().as_milliseconds() as units::MS
    }
}