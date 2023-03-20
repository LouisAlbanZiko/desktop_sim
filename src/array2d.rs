
pub struct Array2D<T: Sized> {
	w: usize,
	h: usize,
	data: Vec<T>
} 

impl<T> Array2D<T> where T: Sized, T: Clone {
	pub fn new(w: usize, h: usize, val: T) -> Self {
		Array2D {
			w: w,
			h: h,
			data: vec![val; w * h]
		}
	}

	pub fn as_ptr(&self) -> *const T {
		self.data.as_ptr()
	}

	#[allow(dead_code)]
	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.data.as_mut_ptr()
	}

	pub fn width(&self) -> usize {
		self.w
	}

	pub fn height(&self) -> usize {
		self.h
	}

	#[allow(dead_code)]
	pub fn as_vec(&self) -> &Vec<T> {
		&self.data
	}

	#[allow(dead_code)]
	pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
		&mut self.data
	}
}

impl<T> std::ops::Index<(usize, usize)> for Array2D<T> where T: Sized, T: Clone {
	type Output = T;
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.data[index.0 + index.1 * self.w]
	}
}

impl<T> std::ops::IndexMut<(usize, usize)> for Array2D<T> where T: Sized, T: Clone {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		self.data.get_mut(index.0 + index.1 * self.w).unwrap()
	}
}
