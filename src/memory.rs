use std::sync::Arc;
use std::mem;
use super::gtop;
use super::GLibTopHandle;
use super::percent_usage::PercentUsage;

pub struct Memory {
    gtop_memory: gtop::glibtop_mem,
    handle: Option<Arc<GLibTopHandle>>
}

impl Memory {
    /// Always returns true, since we always have RAM
    pub fn is_available() -> bool {
        true // we always have RAM
    }

    /// Instantate a new Memory struct, call `glibtop_init()`. `glibtop_close()` will be called when this
    /// object is dropped
    pub fn new() -> Memory {
        let mut m = Memory {
            handle: None,
            gtop_memory: unsafe {mem::zeroed()}
        };
        unsafe {
            gtop::glibtop_init();
            gtop::glibtop_get_mem(&mut m.gtop_memory);
        }
        return m
    }

    /// Instantate a new Memory struct, does not call `glibtop_init()`. `glibtop_close()` will be called when all
    /// references to this handle are dropped.
    pub fn with_handle(h: Arc<GLibTopHandle>) -> Memory {
        let mut m = Memory {
            handle: Some(h),
            gtop_memory: unsafe {mem::zeroed()}
        };
        unsafe {
            gtop::glibtop_get_mem(&mut m.gtop_memory);
        }
        return m
    }

    /// Update the values from glibtop
    pub fn measure(&mut self) {
        unsafe {
            gtop::glibtop_get_mem(&mut self.gtop_memory);
        }
    }

    /// Expose fields from the glibtop_mem struct. Not updated unless `Memory::measure()` is called
    // pub flags: guint64,
    pub fn flags(&self) -> u64 {
        self.gtop_memory.flags
    }
    // pub total: guint64,
    pub fn total(&self) -> u64 {
        self.gtop_memory.total
    }
    // pub used: guint64,
    pub fn used(&self) -> u64 {
        self.gtop_memory.used
    }
    // pub free: guint64,
    pub fn free(&self) -> u64 {
        self.gtop_memory.free
    }
    // pub shared: guint64,
    pub fn shared(&self) -> u64 {
        self.gtop_memory.shared
    }
    // pub buffer: guint64,
    pub fn buffer(&self) -> u64 {
        self.gtop_memory.buffer
    }
    // pub cached: guint64,
    pub fn cached(&self) -> u64 {
        self.gtop_memory.cached
    }
    // pub user: guint64,
    pub fn user(&self) -> u64 {
        self.gtop_memory.user
    }
    // pub locked: guint64,
    pub fn locked(&self) -> u64 {
        self.gtop_memory.locked
    }

}

impl PercentUsage for Memory {
    fn percent_usage(&mut self) -> f64 {
        self.measure();
        let m     = self.gtop_memory;
        let total = m.total as f64;
        let user  = m.user as f64;
        user/total // same number that gnome-system-monitor reports
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        if self.handle.is_none() { // manual glibtop init and close
            unsafe { gtop::glibtop_close() }
        }
    }
}
