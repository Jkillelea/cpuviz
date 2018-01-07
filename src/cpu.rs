#![allow(warnings)]

use std::time::{Instant, Duration};
use std::mem;
use std::sync::Arc;
use super::GLibTopHandle;
use super::percent_usage::PercentUsage; // trait

const N_CPUS: u32 = 4;

/// Keep the state of the CPU: Holds a handle to a glibtop_cpu struct and a recording of the last time a measurement was taken
pub struct Cpu {
    last_time:      Instant,
    cpu_handle:     super::gtop::glibtop_cpu,
    glibtop_handle: Option<Arc<GLibTopHandle>> // if None, glibtop will be opened and closed manually in new() and drop()
}

impl Cpu {
    /// Is this type of device available on this platform? Yes, a CPU is always available.
    pub fn is_available() -> bool {
        true // we always have a cpu
    }

    /// Instantiate a new Cpu. Calls glibtop_init and glibtop_close when dropped
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            last_time:      Instant::now(),
            cpu_handle:     unsafe {mem::zeroed()},
            glibtop_handle: None,
        };
        unsafe {
            super::gtop::glibtop_init();
            super::gtop::glibtop_get_cpu(&mut cpu.cpu_handle);
        }
        cpu.measure(); // inital measurement
        return cpu
    }

    /// Create a new object with a handle to a glibtop object.
    /// `glibtop_close` is called when all references to the handle are dropped accross the entire scope of the program.
    pub fn with_handle(h: Arc<GLibTopHandle>) -> Cpu {
        let mut cpu = Cpu {
            last_time:      Instant::now(),
            cpu_handle:     unsafe {mem::zeroed()},
            glibtop_handle: Some(h),
        };
        unsafe {
            super::gtop::glibtop_get_cpu(&mut cpu.cpu_handle);
        }
        cpu.measure(); // inital measurement
        return cpu
    }

    /// Return the number of jiffies and the amount of time elapsed since the last measurement.
    pub fn measure(&mut self) -> (u64, Duration) { // returns numer of jiffies, time since last measurement
        let now = Instant::now();
        let last_jiffies = self.total() - self.idle();
        unsafe {
            super::gtop::glibtop_get_cpu(&mut self.cpu_handle); // update number of jiffies
        }
        let time_diff = now - self.last_time;
        let jiffy_diff = self.total() - self.idle() - last_jiffies;
        self.last_time = now; // update time. jiffies already updated by call to glibtop_get_cpu

        (jiffy_diff, time_diff)
    }

    /// Returns the last reported value from the glibtop_cpu struct. Only updates when Cpu::measure() is called.
    pub fn total(&self) -> u64 {
        self.cpu_handle.total
    }
    pub fn flags(&self) -> u64 {
        self.cpu_handle.flags
    }
    pub fn user(&self) -> u64 {
        self.cpu_handle.user
    }
    pub fn nice(&self) -> u64 {
        self.cpu_handle.nice
    }
    pub fn sys(&self) -> u64 {
        self.cpu_handle.sys
    }
    pub fn idle(&self) -> u64 {
        self.cpu_handle.idle
    }
    pub fn iowait(&self) -> u64 {
        self.cpu_handle.iowait
    }
    pub fn irq(&self) -> u64 {
        self.cpu_handle.irq
    }
    pub fn frequency(&self) -> u64 {
        self.cpu_handle.frequency
    }
}

impl PercentUsage for Cpu {
    fn percent_usage(&mut self) -> f64 {
        let (j, d) = self.measure(); // elapsed jiffies and elapsed time
        let j = j as f64;
        let d = float_seconds(d);
        j/(100.0 * (N_CPUS as f64) * d) // dividing by d gives an answer between 0 and 400. Divide by 100*N_CPUS to bring it between 0 and 1.0
    }
}

impl Drop for Cpu {
    /// Calls glibtop_close at the moment, might want to move this to something reference counted.
    fn drop(&mut self) {
        if self.glibtop_handle.is_none() { // if opening and closing glibtop manually
            unsafe {
                super::gtop::glibtop_close();
            }
        }
    }
}

// convert std::time::Duration to a floating point number of seconds
fn float_seconds(t: Duration) -> f64 {
    let s = t.as_secs();      // number of seconds
    let n = t.subsec_nanos(); // nanoseconds
    (s as f64) + ((n as f64) * 1e-9)
}
