//basic_info.rs
//contains structs and methods for the basic memory information tag (type 4)

use core::fmt;

#[repr(packed)]
pub struct BasicMemoryInfoTag {
	tag_type: u32,
	size: u32,
	mem_lower: u32,
	mem_upper: u32,
}

impl BasicMemoryInfoTag {
	pub fn get_mem_lower(&self) -> u32 {
		self.mem_lower
	}

	pub fn get_mem_upper(&self) -> u32 {
		self.mem_upper
	}
}

impl fmt::Debug for BasicMemoryInfoTag {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "Basic Memory Info- Lower: {:#X}\nUpper: {:#X}",
			self.mem_lower, self.mem_upper)?;

		Ok(())
	}
}
