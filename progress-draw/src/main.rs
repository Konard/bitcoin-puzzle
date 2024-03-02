use image::{Rgb, RgbImage, ImageBuffer};

fn create_image() {
    // Define the size of the image
    let width = 8064;
    let height = 8064;
    
    // Create a new RgbImage with specified width and height
    let mut img: RgbImage = ImageBuffer::new(width, height);

    // Iterate through all pixels and set them as black
    // for x in 0..width {
    //     for y in 0..height {
    //         let pixel = img.get_pixel_mut(x, y);
    //         *pixel = Rgb([0, 0, 0]); // Set pixel color to black
    //     }
    // }

    for i in 65011712..width*height {
        // Calculate the coordinates of the pixel to set as blue
        let row = i / width;
        let col = i % width;

        // Set the specified pixel as blue
        let blue_pixel = img.get_pixel_mut(col, row);
        *blue_pixel = Rgb([0, 0, 255]); // Set pixel color to blue
    }

    // Save the image to a file
    img.save("progress.png").expect("Failed to save image");
}

fn main() {
    create_image();

    println!("Image created.");
}
