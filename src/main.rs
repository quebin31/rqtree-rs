mod prim;
mod rqtree;

use image;

fn main() {
    let mut rqtree = rqtree::RQTree::with_umbral(10.0);
    let img = image::open("test/test.png").expect("Failed to open!");
    let img = img.to_rgb();

    let output = rqtree.segment_image(&img);
    output.save("test/output.png").expect("Failed to write!");
}
