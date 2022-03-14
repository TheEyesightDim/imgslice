use clap::Parser;
use image::{self, GenericImageView};
use std::{env, path::Path};

fn main() {
    let opts = CLI::parse();
    let input: &str = &opts.input;
    let img = match image::open(&input) {
        Ok(i) => i,
        _ => {
            println!("Error: could not open source image");
            return;
        }
    };

    let out = env::current_dir().unwrap();
    let filename_base = match &opts.output {
        Some(s) => Path::new(s).file_stem().unwrap(),
        None => Path::new(&input).file_stem().unwrap(),
    };

    let extension = Path::new(&input).extension().unwrap();

    println!("Slicing input {}...", input);
    let mut count: u32 = 0;
    let (orig_w, orig_h) = img.dimensions();
    for i in 0..orig_w / opts.width {
        for j in 0..orig_h / opts.height {
            let x = i * opts.width;
            let y = j * opts.height;
            let tile = img.crop_imm(x, y, opts.width, opts.height);

            let mut o = out.clone();
            o.push(format!("{}_{}", filename_base.to_str().unwrap(), count));
            let o = o.with_extension(extension);
            count += 1;
            match tile.save(&o) {
                Ok(_) => println!("Saving {}...", &o.display()),
                Err(e) => println!("Error: unable to save output ({}) - {}", &o.display(), e),
            }
        }
    }
}

/// Slice an image into tiles, and save them to files.
#[derive(Parser)]
struct CLI {
    /// The path to the input file.
    #[clap(short, long)]
    input: String,

    /// The width of a tile.
    #[clap(short, long)]
    width: u32,

    /// The height of a tile.
    #[clap(short, long)]
    height: u32,

    /// The name of the output file.
    #[clap(short, long)]
    output: Option<String>,
}
