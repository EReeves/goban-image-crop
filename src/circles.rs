use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_char;

///Each triplet [0, 1, 2, ...] represents [x, y, radius]
pub type Circles = Vec<i32>;

extern "C" {
    fn get_circles_from_img(
        path: *const c_char,
        circles: *mut i32,
        length: *mut u32,
        buffer_size: u32,
    ) -> i32;
}

///Returns vector of triplet chunks representing the x, y, and radius of circles found from image at path str.
pub fn from_img_path(str: &str) -> Circles {
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

pub struct Border {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

/// Returns the border rect of the circles.
pub fn find_border(circles: Circles) -> Border {
    let mut border = Border {x: i32::MAX, y: i32::MAX, w: 0, h: 0};

    //For calculating avg radius.
    let mut radius_add = 0;

    for triplet in circles.chunks(3) {
        let (cx, cy) = (triplet[0], triplet[1]);

        if cx > border.w {
            border.w = cx
        } else if cx < border.x {
            border.x = cx
        };

        if cy > border.h {
            border.h = cy
        } else if cy < border.y {
            border.y = cy
        };

        radius_add += triplet[2]
    }

    //Average radius
    let avg = radius_add / (circles.len() as i32 / 3);

    border.w += avg;
    border.h += avg;
    border.x -= avg;
    border.y -= avg;

    border
}