use std::time;

use units;


fn duration_in_game(duration: time::Duration) -> units::Duration {
    let seconds = duration.as_secs();
    let nanos = duration.subsec_nanos();
    let time_s = seconds as units::Time * units::SEC;
    // a billion times the actual time represented by the nanos
    let time_n_bi = nanos as units::Time * units::SEC;
    time_s + time_n_bi / 1_000_000_000
}

struct Simple {
    start_instant: Option<time::Instant>,
    last_time: units::Time,
}

impl Simple {
    fn new(start_time: units::Time) -> Simple {
        Simple {
            start_instant: None,
            last_time: start_time,
        }
    }

    fn elapsed_as_of(&self, now: time::Instant) -> time::Duration {
        if let Some(start) = self.start_instant {
            now.duration_since(start)
        } else {
            // time only passes if the clock has started
            time::Duration::new(0,0)
        }
    }

    fn time(&self, now: time::Instant) -> units::Time {
        let elapsed = self.elapsed_as_of(now);
        self.last_time + duration_in_game(elapsed)
    }

    fn stop(&mut self, now: time::Instant) {
        self.last_time = self.time(now);
        self.start_instant = None;
    }

    fn start(&mut self, now: time::Instant) {
        self.stop(now);
        self.start_instant = Some(now);
    }
}


pub struct Stuttering {
    // continuously reset this field to control the clock
    pub max_time: units::Time,
    // internal clock is always on, but will be reset as required
    clock: Simple,
}


impl Stuttering {
    pub fn new(igt: units::Time) -> Self {
        let max_time = igt;  // in this way the clock starts 'off'

        let mut clock = Simple::new(igt);
        clock.start(time::Instant::now());

        Stuttering { clock, max_time }
    }

    pub fn time(self: &mut Self) -> units::Time {
        // let Clock handle the time
        let now = time::Instant::now();
        let ig_now = self.clock.time(now);

        // meddle as required
        if ig_now > self.max_time {
            // make the clock stutter so that max_time is not exceeded
            self.clock = Simple::new(self.max_time);
            self.clock.start(now);

            // we must be at max_time
            self.max_time
        } else {
            ig_now
        }
    }
}
