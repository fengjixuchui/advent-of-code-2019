//Stole this code from a cargo package, then modified it for my own purposes

#![allow(dead_code)]
pub mod stopwatch {
    use std::default::Default;
    use std::time::{Duration, Instant};

    #[derive(Clone, Copy)]
    pub struct Stopwatch {
        start_time: Option<Instant>,
        elapsed: Duration,
    }

    impl Default for Stopwatch {
        fn default() -> Stopwatch {
            Stopwatch {
                start_time: None,
                elapsed: Duration::from_secs(0),
            }
        }
    }

    impl Stopwatch {
        pub fn new() -> Stopwatch {
            let sw: Stopwatch = Default::default();
            return sw;
        }
        pub fn start_new() -> Stopwatch {
            let mut sw = Stopwatch::new();
            sw.start();
            return sw;
        }

        pub fn start(&mut self) {
            self.start_time = Some(Instant::now());
        }
        pub fn stop(&mut self) {
            self.elapsed = self.elapsed();
            self.start_time = None;
        }
        pub fn reset(&mut self) {
            self.elapsed = Duration::from_secs(0);
            self.start_time = None;
        }
        pub fn restart(&mut self) {
            self.reset();
            self.start();
        }

        pub fn is_running(&self) -> bool {
            return self.start_time.is_some();
        }

        pub fn elapsed(&self) -> Duration {
            match self.start_time {
                Some(t1) => {
                    return t1.elapsed() + self.elapsed;
                },
                None => {
                    return self.elapsed;
                },
            }
        }
        pub fn elapsed_time(&self) -> (u32, f64) {
            let dur = self.elapsed();
            return (dur.as_secs() as u32, dur.subsec_nanos() as f64 / 1000000 as f64);
        }
        pub fn print_elapsed(&self) {
            let (seconds, milliseconds) = self.elapsed_time();
            if seconds > 0 {
                println!("Took {}s {}ms", seconds, milliseconds);
            } else {
                println!("Took {}ms", milliseconds);
            }
        }
    }
}