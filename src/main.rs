extern crate raster;
extern crate clap;
extern crate gif;
use std::env;
use raster::{transform, editor, BlendMode, PositionMode};
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::fs::File;
use std::thread;

struct FrameHolder {
    frame: Frame
}

fn main() {

    let frame_count = 12;

	let args: Vec<String> = env::args().collect();
    let _image_file_name = args[1].clone();
    let _src_image = raster::open(&_image_file_name).unwrap();

    let resize_pixels_per_frame = _src_image.height / frame_count;

    println!("will resize {} pixels per frame", resize_pixels_per_frame);

    let mut gif_output = File::create("output.gif").unwrap();
    let mut encoder = Encoder::new(&mut gif_output, _src_image.width as u16, _src_image.height as u16, &[]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    let mut handles = Vec::with_capacity(frame_count as usize);
    let mut frames: Vec<FrameHolder> = Vec::with_capacity(frame_count as usize);

    for (index, _chunk) in frames.chunks_mut(1).enumerate() {
        let i: i32 = index as i32;
        let mut _outer_image = _src_image.clone();
        let mut _inner_image = _src_image.clone();
        let mut _center_image = _src_image.clone();
        let width = _src_image.width;
        let height = _src_image.height;

        handles.push(thread::spawn(move || {
            transform::resize_exact_height(&mut _inner_image, i*resize_pixels_per_frame).unwrap();
            transform::resize_exact_height(&mut _center_image, height + i*resize_pixels_per_frame).unwrap();
            transform::resize_exact_height(&mut _outer_image, (height*2) + i*resize_pixels_per_frame).unwrap();
            let mut overlay = editor::blend(&_outer_image, &_center_image, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap();
            overlay = editor::blend(&overlay, &_inner_image, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap();
            editor::crop(&mut overlay, width, height, PositionMode::Center, 0, 0).unwrap();
            chunk.iter().next().frame = Frame::from_rgba(width as u16, height as u16, overlay.bytes.as_mut_slice());
        }));
    }
    for handle in handles {
        handle.join();
    }
}
