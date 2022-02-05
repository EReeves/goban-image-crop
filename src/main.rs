#![feature(vec_into_raw_parts)]
#![feature(test)]

extern crate num;

mod circles;
mod cl_args;

use circles::get_circles;
use cl_args::Args;
use clap::Parser;
use glob::{glob, Paths};
use image::{imageops, GenericImageView};
use num::clamp;
use std::path::PathBuf;

fn main() {
    let args = Args::parse();

    let paths = get_files(&args);
    for ext in paths {
        for path in ext {
            match path {
                Ok(p) => {
                    let p2 = p.clone();
                    println!("{}", p.display());
                    let circles = get_circles(p.into_os_string().to_str().unwrap());
                    let minmax = find_min_max(circles);
                    crop(p2, minmax, &args.padding);
                }
                Err(_) => todo!(),
            }
        }
    }
}

fn crop(path: PathBuf, minmax: (i32, i32, i32, i32), padding: &u32) {
    let mut img = image::open(&path).unwrap();
    let size = img.dimensions();

    //Still need to clamp to actual image dimensions.
    let x = minmax.1 - *padding as i32;
    let y = minmax.3 - *padding as i32;
    let w = minmax.0 as u32 + padding;
    let h = minmax.2 as u32 + padding;

    //Crop here, while clamping.
    let subimg = imageops::crop(
        &mut img,
        clamp(x, 1, i32::MAX) as u32,
        clamp(y, 1, i32::MAX) as u32,
        clamp(w, 1, size.0),
        clamp(h, 1, size.1),
    );

    subimg.to_image().save(path).unwrap();
}

/// Returns a tuple of min/max coordinates (max_x, min_x, max_y, min_y)
fn find_min_max(circles: Vec<i32>) -> (i32, i32, i32, i32) {
    assert!(circles.len() > 0);
    let (mut max_x, mut min_x, mut max_y, mut min_y) = (0, i32::MAX, 0, i32::MAX);

    let mut addt_radius = 0;

    for triplet in circles.chunks(3) {
        let (x, y) = (triplet[0], triplet[1]);

        if x > max_x {
            max_x = x
        } else if x < min_x {
            min_x = x
        };

        if y > max_y {
            max_y = y
        } else if y < min_y {
            min_y = y
        };

        addt_radius += triplet[2]
    }

    //Average radius
    let avg = addt_radius / (circles.len() as i32 / 3);

    (max_x + avg, min_x - avg, max_y + avg, min_y - avg)
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
