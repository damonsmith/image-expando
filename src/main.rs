extern crate raster;
extern crate clap;
extern crate gif;
use clap::{Arg, App};
use raster::{transform, editor, BlendMode, PositionMode};
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::fs::File;
fn main() {

    let _matches = App::new("Image to expander animation generator")
        .version("1.0")
        .author("Damon Smith <damon.default@gmail.com>")
        .about("Spins and expands an image to create an animated gif")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    
    let frame_count = 12;
    let image_file_name = _matches.value_of("INPUT").unwrap();
    let mut _src_image = raster::open(image_file_name).unwrap();

    let resize_pixels_per_frame = _src_image.height / frame_count;

    println!("will resize {} pixels per frame", resize_pixels_per_frame);

    let mut gif_output = File::create("output.gif").unwrap();
    let mut encoder = Encoder::new(&mut gif_output, _src_image.width as u16, _src_image.height as u16, &[]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    for i in 0..frame_count {
        
        let mut _inner_image = _src_image.clone();
        let mut _center_image = _src_image.clone();
        let mut _outer_image = _src_image.clone();
        
        transform::resize_exact_height(&mut _inner_image, i*resize_pixels_per_frame).unwrap();
        transform::resize_exact_height(&mut _center_image, _src_image.height + i*resize_pixels_per_frame).unwrap();
        transform::resize_exact_height(&mut _outer_image, (_src_image.height*2) + i*resize_pixels_per_frame).unwrap();
        
        let mut overlay = editor::blend(&_outer_image, &_center_image, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap();
        overlay = editor::blend(&overlay, &_inner_image, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap();
        
        editor::crop(&mut overlay, _src_image.width, _src_image.height, PositionMode::Center, 0, 0).unwrap();
        
        let _frame = Frame::from_rgba(_src_image.width as u16, _src_image.height as u16, overlay.bytes.as_mut_slice());
        encoder.write_frame(&_frame).unwrap();
    }
}
