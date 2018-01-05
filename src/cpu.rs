#![allow(warnings)]

use std::time::{Instant, Duration};
use std::mem;
use std::sync::Arc;
use super::GLibTopHandle;

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
        unsafe {
            let mut cpu = Cpu {
                last_time:      Instant::now(),
                cpu_handle:     mem::zeroed(),
                glibtop_handle: None,
            };
            super::gtop::glibtop_init();
            super::gtop::glibtop_get_cpu(&mut cpu.cpu_handle);
            return cpu
        }
    }

    /// Create a new object with a handle to a glibtop object.
    /// `glibtop_close` is called when all references to the handle are dropped accross the entire scope of the program.
    pub fn with_handle(h: Arc<GLibTopHandle>) -> Cpu {
        unsafe {
            let mut cpu = Cpu {
                last_time:      Instant::now(),
                cpu_handle:     mem::zeroed(),
                glibtop_handle: Some(h),
            };
            super::gtop::glibtop_get_cpu(&mut cpu.cpu_handle);
            return cpu
        }
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
        if self.glibtop_handle.is_none() { // if opening and closing glibtop manually
            unsafe {
                super::gtop::glibtop_close();
            }
        }
    }
}
