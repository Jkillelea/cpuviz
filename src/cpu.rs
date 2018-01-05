#![allow(warnings)]

use std::time::{Instant, Duration};
use std::mem;

/// Keep the state of the CPU: Holds a handle to a glibtop_cpu struct and a recording of the last time a measurement was taken
pub struct Cpu {
    last_time: Instant,
    cpu_handle: super::gtop::glibtop_cpu,
}

impl Cpu {
    /// Is this type of device available on this platform? Yes, a CPU is always available.
    pub fn is_available() -> bool {
        true // we always have a cpu
    }

    /// Instantiate a new Cpu. Right now, this calls glibtop_init, but that might change in the future.
    pub fn new() -> Cpu {
        unsafe {
            let mut cpu = Cpu {
                last_time: Instant::now(),
                cpu_handle: mem::zeroed(),
            };
            super::gtop::glibtop_init();
            super::gtop::glibtop_get_cpu(&mut cpu.cpu_handle);
            return cpu
        }
    }

    /// Return the number of jiffies and the amount of time elapsed since the last measurement.
    pub fn measure(&mut self) -> (u64, Duration) { // returns numer of jiffies, time since last measurement
        let now = Instant::now();
        let last_jiffies = self.cpu_handle.total;
        unsafe {
            super::gtop::glibtop_get_cpu(&mut self.cpu_handle); // update number of jiffies
        }
        let time_diff = now - self.last_time;
        let jiffy_diff = self.cpu_handle.total - last_jiffies;
        self.last_time = now; // update time. jiffies already updated by call to glibtop_get_cpu
        (jiffy_diff, time_diff)
    }

    pub fn frequency(&self) -> u64 {
        self.cpu_handle.frequency
    }

    // pub fn load_percent(&mut self) {
    //     // (number of processors) * (proc_times2 - proc_times1) * 100 / (float) (total_cpu_usage2 - total_cpu_usage1)
    //     let prev_total_usage = self.cpu_handle.total;
    //     self.measure();
    //     let now_total_usage = self.cpu_handle.total;
    //
    // }
}

impl Drop for Cpu {
    /// Calls glibtop_close at the moment, might want to move this to something reference counted.
    fn drop(&mut self) {
        unsafe {
            super::gtop::glibtop_close();
        }
    }
}
