mod args;
use args::ProgramArgs;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};
use std::{fs::File, io::BufReader};

// A numeration to holds Errors.
#[derive(Debug)]
enum ImageDataError {
    DifferentImageFormats,
    BufferTooSmall,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        // Create a buffer containing all pixels in four channels: RGBA.
        let buffer_capacity = height * width * 4;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity.try_into().unwrap());

        // Initilize a FloatingImage instance and return it.
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataError> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataError::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataError> {
    // Get the args using ProgramArgs.
    let args: ProgramArgs = ProgramArgs::new();

    // Return DynamicImage instances from the location given by args.
    let (first_image, first_image_format) = find_image_from_path(args.fir_img_loc);
    let (second_image, second_image_format) = find_image_from_path(args.sec_img_loc);

    // Check if the formats are equal, if not, return Error.
    if first_image_format != second_image_format {
        return Err(ImageDataError::DifferentImageFormats);
    }

    // Change images' sizes to smallest one.
    let (first_image, second_image) = standardize_size(first_image, second_image);

    // Alocate space for output and insert the combined data into.
    let mut output: FloatingImage =
        FloatingImage::new(first_image.width(), first_image.height(), args.out_img_loc);
    let combined_data: Vec<u8> = combine_images(first_image, second_image);
    output.set_data(combined_data)?;

    // Save to output file.
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        first_image_format,
    )
    .unwrap();

    // Return OK.
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    // This function reads the image and return its DynamicImage and format.
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

    (image, image_format)
}

fn get_smallest_dimensions(first_dim: (u32, u32), second_dim: (u32, u32)) -> (u32, u32) {
    // This function returns the smallest dimensions amongs the given ones.
    let first_pixel_count: u32 = first_dim.0 * first_dim.1;
    let second_pixel_count: u32 = second_dim.0 * second_dim.1;

    if first_pixel_count < second_pixel_count {
        first_dim
    } else {
        second_dim
    }
}

fn standardize_size(
    first_image: DynamicImage,
    second_image: DynamicImage,
) -> (DynamicImage, DynamicImage) {
    // This function resizes bigger image to smallest one's sizes and return both.
    let (width, height) =
        get_smallest_dimensions(first_image.dimensions(), second_image.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if second_image.dimensions() == (width, height) {
        (
            first_image.resize_exact(width, height, Triangle),
            second_image,
        )
    } else {
        (
            first_image,
            second_image.resize_exact(width, height, Triangle),
        )
    }
}

fn combine_images(first_image: DynamicImage, second_image: DynamicImage) -> Vec<u8> {
    let first_vec: Vec<u8> = first_image.to_rgba8().into_vec();
    let second_vec: Vec<u8> = second_image.to_rgba8().into_vec();

    alternate_pixels(first_vec, second_vec)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data: Vec<u8> = vec![0u8; vec_1.len()];

    let mut index = 0;
    while index < vec_1.len() {
        if index % 8 == 0 {
            combined_data.splice(index..=index + 3, set_rgba(&vec_1, index, index + 3));
        } else {
            combined_data.splice(index..=index + 3, set_rgba(&vec_2, index, index + 3));
        }
        index += 4;
    }

    combined_data
}

fn set_rgba(vector: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba: Vec<u8> = Vec::new();
    for index in start..=end {
        let val: u8 = match vector.get(index) {
            Some(d) => *d,
            None => panic!("Index is out of bounds."),
        };

        rgba.push(val);
    }

    rgba
}
