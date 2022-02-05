// build.rs
extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    //Copy images to output for testing
    let co = CopyOptions {
        skip_exist: false,
        overwrite: true,
        ..Default::default()
    };
    copy("./media", &out_dir, &co).unwrap_or_default();

    pkg_config::Config::new().probe("opencv4").unwrap();

    cc::Build::new()
        .cpp(true)
        .file("src/csrc/circles.cpp")
        .compile("circles");

    println!("cargo:rerun-if-env-changed=REBUILD");  
    println!("cargo:rustc-link-lib=dylib=opencv_core");
    println!("cargo:rustc-link-lib=dylib=opencv_imgproc");
    println!("cargo:rustc-link-lib=dylib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=dylib=opencv_highgui");
}
