use std::collections::VecDeque;

pub trait ReadBytes {
	fn pop_u8(&mut self) -> u8;
	fn pop_u16(&mut self) -> u16;
	fn pop_u32(&mut self) -> u32;
	fn pop_u64(&mut self) -> u64;
	fn pop_vec(&mut self, length: usize) -> Vec<u8>;

	fn peek_u8(&mut self) -> u8;
	fn peek_u16(&mut self) -> u16;
}

impl ReadBytes for VecDeque<u8> {
	fn pop_u8(&mut self) -> u8 { self.pop_front().unwrap() }

	fn pop_u16(&mut self) -> u16 {
		let mut result: u16 = 0;
		result += self.pop_front().unwrap() as u16;
		result = self.pop_front().unwrap() as u16 + (result << 8);
		result
	}

	fn pop_u32(&mut self) -> u32 {
		let mut result: u32 = 0;
		result += self.pop_front().unwrap() as u32;
		for _i in 0..3 {
			result = self.pop_front().unwrap() as u32 + (result << 8);
		}

		result
	}

	fn pop_u64(&mut self) -> u64 {
		let mut result: u64;
		result = self.pop_front().unwrap() as u64;
		for _i in 0..7 {
			result = self.pop_front().unwrap() as u64 + (result << 8);
		}

		result
	}

	fn pop_vec(&mut self, length: usize) -> Vec<u8> {
		let mut result: Vec<u8> = Vec::new();
		for _i in 0..length {
			result.push(self.pop_front().unwrap());
		}

		result
	}

	fn peek_u8(&mut self) -> u8 { self[0] }

	fn peek_u16(&mut self) -> u16 {
		let mut result: u16 = self[0] as u16;
		result = self[1] as u16 + (result << 8);
		result
	}
}
