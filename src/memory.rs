use std::sync::Arc;
use std::mem;
use super::gtop;
use super::GLibTopHandle;
use super::percent_usage::PercentUsage;

/// A handle to a `glibtop_mem` struct and an `Option<Arc<GLibTopHandle>>` to determine when `glibtop_init()`
/// and `glibtop_close()` should be called
pub struct Memory {
    gtop_memory: gtop::glibtop_mem,
    handle: Option<Arc<GLibTopHandle>>
}

impl Memory {
    /// Always returns true, since we (should) always have RAM
    pub fn is_available() -> bool {
        true // we always have RAM
    }

    /// Instantate a new Memory struct, call `glibtop_init()`. `glibtop_close()` will be called when this
    /// object is dropped
    pub fn new() -> Memory {
        let mut m = Memory {
            handle: None,
            gtop_memory: unsafe { mem::zeroed() }
        };
        unsafe { gtop::glibtop_init(); }
        m.measure();
        return m
    }

    /// Instantate a new Memory struct, does not call `glibtop_init()`. `glibtop_close()` will be called when all
    /// references to this handle are dropped.
    pub fn with_handle(h: Arc<GLibTopHandle>) -> Memory {
        let mut m = Memory {
            handle: Some(h),
            gtop_memory: unsafe { mem::zeroed() }
        };
        m.measure();
        return m
    }

    /// Update the values from glibtop
    pub fn measure(&mut self) {
        unsafe {
            gtop::glibtop_get_mem(&mut self.gtop_memory);
        }
    }

    /// Expose fields from the glibtop_mem struct. Not updated unless `Memory::measure()` is called
    pub fn flags(&self) -> u64 {
        // pub flags: guint64,
        self.gtop_memory.flags
    }
    pub fn total(&self) -> u64 {
        // pub total: guint64,
        self.gtop_memory.total
    }
    pub fn used(&self) -> u64 {
        // pub used: guint64,
        self.gtop_memory.used
    }
    pub fn free(&self) -> u64 {
        // pub free: guint64,
        self.gtop_memory.free
    }
    pub fn shared(&self) -> u64 {
        // pub shared: guint64,
        self.gtop_memory.shared
    }
    pub fn buffer(&self) -> u64 {
        // pub buffer: guint64,
        self.gtop_memory.buffer
    }
    pub fn cached(&self) -> u64 {
        // pub cached: guint64,
        self.gtop_memory.cached
    }
    pub fn user(&self) -> u64 {
        // pub user: guint64,
        self.gtop_memory.user
    }
    pub fn locked(&self) -> u64 {
        // pub locked: guint64,
        self.gtop_memory.locked
    }

}

impl PercentUsage for Memory {
    fn percent_usage(&mut self) -> f64 {
        self.measure();
        let m     = self.gtop_memory;
        let total = m.total as f64;
        let user  = m.user as f64;
        user/total // Same number that gnome-system-monitor reports. However, they round to the 0.001 place.
                   // I might be missing something somewhere
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        if self.handle.is_none() { // manual glibtop init and close
            unsafe { gtop::glibtop_close() }
        }
    }
}
