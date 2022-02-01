use std::os::raw::{c_int, c_char};
use std::ffi::CString;

extern "C" {
    fn get_circles_from_img(path: *const c_char) -> c_int;
}

pub fn get_circles(str: &str) {//TODO: -> Vec<[f64;4]>> {

    let cstr = CString::new(str)
    .expect("Null termination exists in filename");
    unsafe {get_circles_from_img(cstr.as_ptr());};
}

#[test]
fn test_circles() {
    let mut s = "./go.png";
    unsafe { 
        //TODO:test
    }
}