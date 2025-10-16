// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.
use std::time::Instant;

#[derive(Clone)]
pub struct Timer {
    check_point: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            check_point: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.check_point = Instant::now();
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.check_point.elapsed()
    }

    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }

    pub fn elapsed_seconds_for_step(&self, step: &str) -> String {
        format!("Time for {}: {:.3} seconds", step, self.elapsed_seconds())
    }
}

#[cfg(test)]
mod timer_tests {
    use std::{thread, time};

    use super::*;

    #[test]
    fn test_new() {
        let timer = Timer::new();
        assert!(timer.check_point.elapsed().as_secs() < 1);
    }

    #[test]
    fn test_reset() {
        let mut timer = Timer::new();
        thread::sleep(time::Duration::from_millis(100));
        timer.reset();
        assert!(timer.check_point.elapsed().as_millis() < 10);
    }

    #[test]
    #[ignore]
    fn test_elapsed() {
        let timer = Timer::new();
        thread::sleep(time::Duration::from_millis(100));
        assert!(timer.elapsed().as_millis() > 100);
        assert!(timer.elapsed_seconds() > 0.1);
    }

    #[test]
    fn test_elapsed_seconds_for_step() {
        let timer = Timer::new();
        let output = timer.elapsed_seconds_for_step("test step");
        assert!(output.contains("test step"));
        assert!(output.contains("seconds"));
    }
}
