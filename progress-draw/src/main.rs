use image::{Rgb, RgbImage, ImageBuffer};

const WIDTH: u32 = 5794;
const HEIGHT: u32 = 5794;

fn set_pixel(img: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
    let pixel = img.get_pixel_mut(x, y);
    *pixel = color;
}

fn set_done_pixel(img: &mut RgbImage, current: u32) {
    let start: u32 = 0x2000000;
    let pixel_number = current - start;

    let x = pixel_number % WIDTH;
    let y = pixel_number / WIDTH;

    set_pixel(img, x, y, Rgb([255, 0, 0]));
}

fn set_progress_pixel(img: &mut RgbImage, current: u32) {
    let start: u32 = 0x2000000;
    let pixel_number = current - start;

    let x = pixel_number % WIDTH;
    let y = pixel_number / WIDTH;

    set_pixel(img, x, y, Rgb([255, 255, 0]));
}

fn create_image() {
    // Create a new RgbImage with specified width and height
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    // Iterate through all pixels and set them as black
    // for x in 0..width {
    //     for y in 0..height {
    //         let pixel = img.get_pixel_mut(x, y);
    //         *pixel = Rgb([0, 0, 0]); // Set pixel color to black
    //     }
    // }

    for i in (0x3ffffff-0x2000000)..(WIDTH*HEIGHT) {
        let x = i % WIDTH;
        let y = i / WIDTH;
        
        set_pixel(&mut img, x, y, Rgb([0, 0, 255]));
    }

    set_done_pixel(&mut img, 0x2000000);
    set_done_pixel(&mut img, 0x2000001);
    set_done_pixel(&mut img, 0x2ffffff);
    set_done_pixel(&mut img, 0x3000000);
    set_progress_pixel(&mut img, 0x3fffffe);
    set_done_pixel(&mut img, 0x3ffffff);

    // Save the image to a file
    img.save("progress.png").expect("Failed to save image.");
}

fn main() {
    // println!("{}", 0x3ffffff-0x2000000);
    // println!("{}", 5794*5794);

    create_image();

    println!("Image created.");
}
