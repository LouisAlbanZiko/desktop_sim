extern crate gl;
extern crate gl_loader;
extern crate nalgebra_glm as glm;
extern crate rand;

mod graphics;
mod window;
mod monitor;
mod physics;
mod array2d;

use array2d::*;
use graphics::*;
use window::*;
use monitor::*;
use physics::*;

fn main()
{
	let monitors: Vec<Monitor> = monitors_list();
	println!("Found {} monitors.", monitors.len());
	for monitor in monitors.iter() {
		println!("pos: ({}, {}), size: ({}, {})", monitor.x, monitor.y, monitor.w, monitor.h);
	}

	let window: WindowInfo = window_background_create().unwrap();

	//let frame_buffer_default: FrameBuffer = FrameBuffer::default(window.width, window.height);
	let mut textures: Vec<Texture> = Vec::<Texture>::with_capacity(monitors.len());
	let mut textures_data: Vec<Array2D<Color>> = Vec::<Array2D<Color>>::with_capacity(monitors.len());
	let mut simulations: Vec<Physics> = Vec::<Physics>::with_capacity(monitors.len());
	let mut rectangles: Vec<Rect> = Vec::<Rect>::with_capacity(monitors.len());
	for monitor in monitors.iter() {
		textures.push(Texture::new(monitor.w / 4 / 4, monitor.h / 4 / 4));
		println!("num_rows: {}, num_collumns: {}", monitor.h as usize, monitor.w as usize);
		let default_color = Color { r: 0 as u8, g: 0 as u8, b: 0 as u8, a: 255 as u8 };
		let texture_data = Array2D::new(monitor.w as usize / 4 / 4, monitor.h as usize / 4 / 4, default_color);
		textures_data.push(texture_data);
		simulations.push(Physics::new(monitor.w as usize / 4 / 4, monitor.h as usize / 4 / 4));
		rectangles.push(Rect::new(monitor.x as f32, monitor.y as f32, monitor.w as f32, monitor.h as f32, window.x as f32, window.y as f32, window.width as f32, window.height as f32));
	}

	let shader: graphics::shader::Shader = graphics::shader::load("texture");
	shader.bind();

	//let shader_circle: graphics::shader::Shader = graphics::shader::load("circle");
	
	
	unsafe { gl::ClearColor(0.1, 0.1, 0.2, 1.0) };

	use rand::Rng;
	let mut color = Color { r: rand::thread_rng().gen_range(0..=255), g: rand::thread_rng().gen_range(0..=255), b: rand::thread_rng().gen_range(0..=255), a: 255 };
	let mut red_change: i32 = 1;
	let mut green_change: i32 = -1;
	let mut blue_change: i32 = 1;

	let mut total_duration: f32 = 0.0;
	let mut duration: f32 = 0.0;
	let mut last_time = std::time::Instant::now();
	let mut frames = 0;
	let mut updates = 0;

	let mut average_update_time: u128 = 0;
	let mut total_update_count: u32 = 0;

	while total_duration < 1000.0 {

		// timing
		let time = std::time::Instant::now();

		let dt = (time - last_time).as_secs_f32();
		duration += dt;
		total_duration += dt;
		
		last_time = time;

		frames += 1;
		if duration >= 1.0 {
			let current_average_update_time = (average_update_time / total_update_count as u128) as f64 / 1000000.0;
			println!("fps: {}, ups: {}, average_update_time: {:.3}ms, calc_ups: {}", frames, updates, current_average_update_time, (1000.0 / current_average_update_time) as u128);
			frames = 0;
			updates = 0;
			duration -= 1.0;
		}

		// update
		let update_start_time = std::time::Instant::now();
		{
			for i in 0..simulations.len() {
				simulations[i].update(1.0 / 30.0);
				simulations[i].render(textures_data.get_mut(i).unwrap());

				if updates % 10 == 0 {
					let x: f32 = simulations[i].size.x / 2.0;
					let y: f32 = simulations[i].size.y / 2.0;
					let index = simulations[i].add_object(x, y, color);
					simulations[i][index].vel.x -= 0.2;

					color.r = (color.r as i32 + red_change  ) as u8;
					color.g = (color.g as i32 + green_change) as u8;
					color.b = (color.b as i32 + blue_change ) as u8;

					if color.r == 255 || color.r == 0 { red_change = -red_change; }
					if color.g == 255 || color.g == 0 { green_change = -green_change; }
					if color.b == 255 || color.b == 0 { blue_change = -blue_change; }
				}
			}
		}
		average_update_time += (std::time::Instant::now() - update_start_time).as_nanos();
		total_update_count += 1;
		updates += 1;

		// render
		{
			unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
			for i in 0..simulations.len() {
				textures[i].set_data(&textures_data[i]);
				rectangles[i].draw();

				let c = Color { r: 0 as u8, g: 0 as u8, b: 0 as u8, a: 255 as u8 };
				for y in 0..textures_data[i].height() {
					for x in 0..textures_data[i].width() {
						textures_data[i][(x, y)] = c;
					}
				}
			}
			window_swap_buffers(&window);
		}
	}

	println!("total_update_time: {}, total_update_count {}", average_update_time, total_update_count);
	let current_average_update_time = (average_update_time / total_update_count as u128) as f64 / 1000000.0;
	println!("average_update_time: {:.3}ms, calc_ups: {}", current_average_update_time, (1000.0 / current_average_update_time) as u128);

	unsafe { gl::ClearColor(0.4, 0.1, 0.1, 0.0) };
	unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
	window_swap_buffers(&window);

	window_background_destroy(window);
}


/*
// Define the vertex data for a rectangle
	let vertices: [GLfloat; 5 * 4] = [
		-1.0, -1.0,		1.0, 0.1, 0.1,
		 1.0, -1.0,		0.1, 1.0, 0.1,
		 1.0,  1.0,		0.1, 0.1, 1.0,
		-1.0,  1.0,		1.0, 1.0, 1.0 
	];

	// Create and bind a VBO
	let mut vbo: GLuint = 0;
	unsafe {
		gl::GenBuffers(1, &mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * std::mem::size_of::<GLfloat>() * 2) as GLsizeiptr,
			vertices.as_ptr() as *const GLvoid,
			gl::STATIC_DRAW,
		);
	}

	// Set up the vertex attributes
	unsafe {
		let stride = 6 * std::mem::size_of::<GLfloat>() as GLsizei;
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
		gl::EnableVertexAttribArray(0);
		gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<GLfloat>() as GLsizei) as *const _);
		gl::EnableVertexAttribArray(1);
	}

	// Define the indices for the rectangle
	let indices: [GLuint; 6] = [
		0, 1, 2,
		2, 3, 0
	];

	// Create and bind an index buffer
	let mut ebo: GLuint = 0;
	unsafe {
		gl::GenBuffers(1, &mut ebo);
		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
		gl::BufferData(
			gl::ELEMENT_ARRAY_BUFFER,
			(indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
			indices.as_ptr() as *const GLvoid,
			gl::STATIC_DRAW,
		);
	}
 */