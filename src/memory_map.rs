//memory_map.rs
//contains structs and methods for reading memory map tags (type 6)

use core::fmt;

#[repr(C)]
pub struct MemoryMapTag {
	tag_type: u32,
	size: u32,
	entry_size: u32,
	entry_version: u32,
	starting_entry: MemoryMap,
}

#[repr(C, packed)]
pub struct MemoryMap {
	base_address: u64,
	length: u64,
	entry_type: u32,
	reserved: u32,
}

impl MemoryMap {
	pub fn base_address(&self) -> usize {
		self.base_address as usize
	}

	pub fn length(&self) -> usize {
		self.length as usize
	}

	pub fn end_address(&self) -> usize {
		(self.base_address + self.length) as usize
	}

	pub fn entry_type(&self) -> MemoryMapType {
		match self.entry_type {
			1 => MemoryMapType::Available,
			3 => MemoryMapType::AvailableWithACPI,
			_ => MemoryMapType::Reserved
		}
	}

	pub fn is_availabe(&self) -> bool {
		self.entry_type() == MemoryMapType::Available
	}
}

#[derive(PartialEq)]
pub enum MemoryMapType {
	Available = 1,
	AvailableWithACPI = 3,
	Reserved = 4,
}

impl MemoryMapTag {
	pub fn memory_areas(&self) -> MemoryMapIterator {
		let start_ptr = self as *const MemoryMapTag;
		let starting_entry = &self.starting_entry as *const MemoryMap;

		MemoryMapIterator {
			current_area: starting_entry as u64,
			last_area: (start_ptr as u64 + (self.size - self.entry_size) as u64),
			entry_size: self.entry_size,
		}
	}
}

pub struct MemoryMapIterator {
	current_area: u64,
	last_area: u64,
	entry_size: u32,
}

impl Iterator for MemoryMapIterator {
	type Item = &'static MemoryMap;

	fn next(&mut self) -> Option<&'static MemoryMap> {
		//is a valid address
		if self.current_area <= self.last_area {
			//dereference
			let area = unsafe { &*(self.current_area as *const MemoryMap) };
			//iterate to next area
			self.current_area = self.current_area + (self.entry_size as u64);

			//return area if available
			if area.entry_type == MemoryMapType::Available as u32 {
				Some(area)
			}
			//area is not available for some reason, read next area
			else {
				self.next()
			}
		} 
		//if address of current area exceeds last area, return None
		else {
			None
		}
	}
}

impl fmt::Debug for MemoryMapTag {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "Memory Areas:")?;

		for mem_area in self.memory_areas() {
			writeln!(formatter, "Start: {:#X}, End: {:#X}, Length: {:#X}",
				mem_area.base_address(), mem_area.end_address(), mem_area.length())?;
		}

		Ok(())
	}
}
