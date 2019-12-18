mod prim;
mod rqtree;

use image;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 4 {
        println!("Uso: {} <imagen> <salida> <umbral>", args[0]);
        return;
    }

    let umbral: f64 = args[3].parse().expect("Expected a float!");

    let mut rqtree = rqtree::RQTree::with_umbral(umbral);
    let img = image::open(&args[1]).expect("Failed to open!");
    let img = img.to_rgb();

    let output = rqtree.segment_image(&img);
    output.save(&args[2]).expect("Failed to write!");
}
