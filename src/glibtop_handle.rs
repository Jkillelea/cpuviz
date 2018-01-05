use super::gtop;

pub struct GLibTopHandle();
impl GLibTopHandle {
    /// calls `glibtop_init`
    pub fn new() -> GLibTopHandle {
        // println!("glibtop_init");
        unsafe {
            gtop::glibtop_init();
        }
        GLibTopHandle {}
    }
}

impl Drop for GLibTopHandle {
    /// calls `glibtop_close`
    fn drop(&mut self) {
        // println!("glibtop_close");
        unsafe {
            gtop::glibtop_close();
        }
    }
}
