extern crate test;

use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_char;

use test::Bencher;

extern "C" {
    fn get_circles_from_img(
        path: *const c_char,
        circles: *mut i32,
        length: *mut u32,
        buffer_size: u32,
    ) -> i32;
}

///Returns vector of triplet chunks representing the x, y, and radius of circles found from image at path str.
pub fn get_circles(str: &str) -> Vec<i32> {
    let cstr = CString::new(str).expect("Null termination exists in filename.");

    //Use a buffer to save worrying about deallocation.
    const BUFFER_SIZE: usize = (361 * 3) + 500; //361 triplets per board, plus a buffer.

    let mut circles = Vec::<i32>::with_capacity(BUFFER_SIZE);
    let (ptr, _, _) = circles.into_raw_parts();

    let mut length: MaybeUninit<u32> = MaybeUninit::uninit();

    unsafe {
        assert!(
            get_circles_from_img(cstr.as_ptr(), ptr, length.as_mut_ptr(), BUFFER_SIZE as u32) < 1,
            "Too many circles detected. Adjust parameters to be less forgiving."
        );

        let len = length.assume_init() as usize;
        circles = Vec::from_raw_parts(ptr, len * 3, len * 3);
    };

    circles
}

#[bench]
fn bench_one_image(b: &mut Bencher) {
    b.iter(|| {
        get_circles("./media/go.png");
    });
}
