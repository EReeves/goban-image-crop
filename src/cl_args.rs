use clap::{Parser};

#[derive(Parser)]
#[clap(author = "Evan Reeves", version = "0.1", about = "Batch crop images of gobans.", long_about = None)]
pub struct Args {

    //Whether to search the folder recursively.
    #[clap(short, long)]
    pub recursive: bool,

    //Padding in pixels around where stones are detected
    #[clap(short, long, default_value = "10")]
    pub padding: u32,

    //Comma delimited string of file extensions. For supported extensions see https://crates.io/crates/image.
    #[clap(short, long, default_value = "png,jpg,bmp")]
    pub extensions: String,
    
    //Folder of images
    #[clap(short, long, default_value = ".")] //remove after
    pub directory: String,
}
