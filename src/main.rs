#![feature(vec_into_raw_parts)]

extern crate num;

mod circles;
mod cl_args;

use circles::Border;
use cl_args::Args;
use clap::Parser;
use glob::{glob, Paths};
use num::clamp;
use std::path::PathBuf;
use image::{imageops, GenericImageView, Rgba, ImageBuffer};


fn main() {
    let args = Args::parse();

    let paths = get_files(&args);
    for ext in paths {
        for path in ext {
            match path {
                Ok(p) => {
                    let p2 = p.clone(); //Since it gets consumed by into_os_string.
                    println!("{}", p.display());
                    let circles = circles::from_img_path(p.into_os_string().to_str().unwrap());
                    let border = circles::find_border(circles);
                    let cropped_img = crop_image_with_padding(&p2, border, &args.padding);
                    cropped_img.save(p2).unwrap();
                }
                Err(_) => todo!(),
            }
        }
    }
}

fn get_files(args: &Args) -> Vec<Paths> {
    //Recursive messages and pattern for glob
    let (recurs_msg, recurse_pattern) = match args.recursive {
        true => ("recursively", "/**/*."),
        false => ("non-recursively", "/*."),
    };

    //Extensions
    let extensions = args.extensions.split(',');

    println!(
        "Scraping {} for {:?} (see -e) images {} (see -r).",
        args.directory, args.extensions, recurs_msg
    );

    //Create a glob pattern for each extension.
    let image_patterns = extensions.into_iter().map(|ext| {
        format!(
            "{dir}{rec}{ext}",
            dir = args.directory,
            rec = recurse_pattern,
            ext = ext
        )
    });

    //Probably better not to return lazy iterator hell. Find the images now.
    let paths: Vec<_> = image_patterns
        .into_iter()
        .map(|pattern| -> Paths {
            glob(&pattern).expect("Error parsing extension. Check your arguments.")
        })
        .collect();
    paths
}

fn crop_image_with_padding<'a>(path: &'a PathBuf, border: Border, padding: & i32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = image::open(&path).unwrap();
    let size = img.dimensions();

    let crop_x = border.x - padding; 
    let crop_y = border.y - padding;
    let crop_w = border.w + padding;
    let crop_h = border.h + padding;

    //Crop here, while clamping to the image size.
    let subimg = imageops::crop(
        &mut img,
        clamp(crop_x, 1, i32::MAX) as u32,
        clamp(crop_y, 1, i32::MAX) as u32,
        clamp(crop_w, 1, size.0 as i32) as u32,
        clamp(crop_h, 1, size.1 as i32) as u32,
    );

    subimg.to_image()
}
