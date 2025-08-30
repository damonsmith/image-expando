extern crate clap;
extern crate gif;
extern crate raster;
use gif::{Encoder, Frame, Repeat};
use raster::{editor, transform, BlendMode, PositionMode};
use std::env;
use std::fs::File;
use std::thread;

const NUMTHREADS: usize = 4;
const FRAME_COUNT: usize = 12;

fn main() {
	let args: Vec<String> = env::args().collect();
	let _image_file_name = args[1].clone();
	let _src_image = raster::open(&_image_file_name).unwrap();

	let mut gif_output = File::create("output.gif").unwrap();
	let mut encoder = Encoder::new(
		&mut gif_output,
		_src_image.width as u16,
		_src_image.height as u16,
		&[],
	)
	.unwrap();
	encoder.set_repeat(Repeat::Infinite).unwrap();

	let mut frame_numbers: Vec<i32> = (0..FRAME_COUNT as i32).collect();
	let chunks = frame_numbers.chunks_mut(FRAME_COUNT / NUMTHREADS);

	let mut children = vec![];

	// Divide the set of frames up into a chunk of frames for each thread
	println!("frames: {}, threads: {}", FRAME_COUNT, NUMTHREADS);
	for chunk in chunks {
		let nums_chunk = chunk.to_owned();
		let src = _src_image.clone();
		children.push(thread::spawn(move || {
			let frames: Vec<Frame> = nums_chunk
				.iter()
				.map(|frame_number| {
					return generate_gif_frame(&src, frame_number);
				})
				.collect();
			frames
		}));
	}

	for handle in children {
		let frames = handle.join().unwrap();
		for frame in frames {
			encoder.write_frame(&frame).unwrap();
		}
	}
}

fn generate_gif_frame<'a>(src: &raster::Image, frame_number: &i32) -> Frame<'a> {
	let resize_amount = frame_number * (src.height / FRAME_COUNT as i32);
	let mut _outer_image = src.clone();
	let mut _inner_image = src.clone();
	let mut _center_image = src.clone();
	let width = src.width;
	let height = src.height;

	transform::resize_exact_height(&mut _inner_image, resize_amount).unwrap();
	transform::resize_exact_height(&mut _center_image, height + resize_amount).unwrap();
	transform::resize_exact_height(&mut _outer_image, (height * 2) + resize_amount).unwrap();
	let mut overlay = editor::blend(
		&_outer_image,
		&_center_image,
		BlendMode::Normal,
		1.0,
		PositionMode::Center,
		0,
		0,
	)
	.unwrap();
	overlay = editor::blend(
		&overlay,
		&_inner_image,
		BlendMode::Normal,
		1.0,
		PositionMode::Center,
		0,
		0,
	)
	.unwrap();
	editor::crop(&mut overlay, width, height, PositionMode::Center, 0, 0).unwrap();
	println!("generated gif frame: {}", frame_number);
	Frame::from_rgba(width as u16, height as u16, overlay.bytes.as_mut_slice())
}
