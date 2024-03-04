
use std::fs::File;
use std::sync::Once;

static mut OUTFILE: *mut File = std::ptr::null_mut();
static INIT: Once = Once::new();

fn init_outfile() {
    unsafe {
        OUTFILE = Box::into_raw(Box::new(File::create("output.s").unwrap()));
    }
}

pub fn get_outfile() -> &'static mut File {
    INIT.call_once(|| init_outfile());
    unsafe { &mut *OUTFILE }
}


