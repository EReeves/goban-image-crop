mod cl_args;
mod circles;

use circles::get_circles;
use cl_args::Args;
use clap::Parser;
use glob::{glob, Paths};


fn main() {
    let args = Args::parse();

    let paths = get_files(args);
    for ext in paths {
        for path in ext {
            match path {
                Ok(p) => {
                    println!("{}", p.display());
                    get_circles(p.into_os_string().to_str().unwrap());
                },
                Err(_) => todo!(),
            }
        }
    }
 

    //let circles = find_circles();
   // println!("{:?}", unsafe {(*circles).get(0)});
}

fn get_files(args: Args) -> Vec<Paths> {
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
            println!("{}", &pattern);
            glob(&pattern).expect("Error parsing extension. Check your arguments.")
        })
        .collect();
    paths
}
