// build.rs
extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};

fn main() {
    //Copy images to debug for testing
    let co = CopyOptions {
        skip_exist: true,
        ..Default::default()
    };
    copy("./media", "./target/debug", &co).unwrap_or_default();

    //pkg_config::Config::new().probe("opencv4").unwrap();
    cc::Build::new()
        .cpp(true)
        .file("src/csrc/circles.cpp")
        .include("/usr/include/opencv4")
        .compile("circles.a");
    println!("cargo:rerun-if-changed=src/csrc/circles.c");
}
