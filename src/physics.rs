use crate::graphics::Color;
use crate::array2d::Array2D;

#[derive(Clone, Copy)]
pub struct VerletObject {
	pub pos: glm::Vec2,
	pub vel: glm::Vec2,
	pub acc: glm::Vec2,
	pub color: Color,
}

impl VerletObject {
	fn check_collision(&mut self, other: &mut Self) {
		let collision_axis = self.pos - other.pos;
		let distance2 = glm::length2(&collision_axis);
		if distance2 < 1.0 {
			let distance = distance2.sqrt();
			let normal = collision_axis / distance;
			let delta = 0.5 * (1.0 - distance);
			self.pos = self.pos + normal * delta;
			self.vel = self.vel + normal * delta  * 100.0;
			other.pos = other.pos - normal * delta;
			other.vel = other.vel - normal * delta * 100.0;
		}
	}

	fn movement(&mut self, dt: f32, acc: glm::Vec2) {
		let new_pos = self.pos + self.vel * dt + self.acc * (dt * dt * 0.5);
		let new_acc = acc;
		let new_vel = self.vel + (self.acc + new_acc) * (dt * 0.5);
		self.pos = new_pos;
		self.vel = new_vel;
		self.acc = new_acc;
	}
}

#[derive(Clone, Copy)]
struct Cell {
	count: u8,
	indices: [u8; 7]
}

pub struct Physics {
	pub size: glm::Vec2,
	objects: Vec<VerletObject>,
	grid: Array2D<Cell>
}

const STEP_COUNT: usize = 8;
const GRAVITY: f32 = -98.0;

impl Physics {
	pub fn new(w: usize, h: usize) -> Physics {
		Physics {
			size: glm::vec2(w as f32, h as f32),
			objects: Vec::with_capacity(w * h),
			grid: Array2D::new(w, h, Cell { count: 0, indices: [0; 7] })
		}
	}

	pub fn add_object(&mut self, x: f32, y: f32, color: Color) -> usize {
		let index = self.objects.len();
		self.objects.push(VerletObject {
			pos: glm::vec2(x, y),
			vel: glm::vec2(0.0, 0.0),
			acc: glm::vec2(0.0, 0.0),
			color: color
		});
		index
	}

	pub fn update(&mut self, dt: f32) {
		let step_dt = dt / (STEP_COUNT as f32);
		for _ in [0..STEP_COUNT] {
			self.apply_collisions();
			self.movement(step_dt);
		}
	}

	pub fn apply_collisions(&mut self) {
		for (index, object) in self.objects.iter().enumerate() {
			let cell: &mut Cell = &mut self.grid[(object.pos.x as usize, object.pos.y as usize)];
			cell.indices[cell.count as usize] = index as u8;
		}

		for v_i1 in 0..self.objects.len() {
			for v_i2 in (v_i1 + 1)..self.objects.len() {
				let (first_slice, second_slice) = self.objects.split_at_mut(v_i2);
				second_slice[0].check_collision(&mut first_slice[v_i1]);
			}
		}
	}

	pub fn movement(&mut self, dt: f32) {
		for object in self.objects.iter_mut() {
			
			// movement
			object.movement(dt, glm::vec2(0.0, GRAVITY));

			// constraint
			let margin: f32 = 0.0;
			if object.pos.x < margin {
				object.pos.x = margin;
				object.vel.x = 0.0;
			} else if object.pos.x > self.size.x - margin {
				object.pos.x = self.size.x - margin;
				object.vel.x = 0.0;
			}
			if object.pos.y < margin {
				object.pos.y = margin;
				object.vel.y = 0.0;
			} else if object.pos.y > self.size.y - margin {
				object.pos.y = self.size.y - margin;
				object.vel.y = 0.0;
			}
		}
	}

	pub fn render(&self, buffer: &mut Array2D<Color>) {
		for object in self.objects.iter() {
			buffer[(object.pos.x as usize, object.pos.y as usize)] = object.color;
		}
	}

}

impl std::ops::Index<usize> for Physics {
	type Output = VerletObject;
	fn index(&self, index: usize) -> &Self::Output {
		&self.objects[index]
	}
}

impl std::ops::IndexMut<usize> for Physics {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.objects[index]
	}
}
