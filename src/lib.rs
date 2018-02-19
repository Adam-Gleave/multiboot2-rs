pub mod basic_info;
pub mod memory_map;
pub mod tag;

extern crate core;

use core::fmt;

use self::basic_info::BasicMemoryInfoTag;
use self::memory_map::MemoryMapTag;
use self::tag::{ TagType, Tag, TagIterator };

//public structs and functions for reading multiboot2 information
pub unsafe fn load(start_address: usize) -> Multiboot2Info {
	let header = &*(start_address as *const Multiboot2Fixed);
	Multiboot2Info { header_ptr: header }
}

pub struct Multiboot2Info {
	header_ptr: *const Multiboot2Fixed,
}

struct Multiboot2Fixed {
	total_size: u32,
	reserved: u32,
}

impl Multiboot2Info {
	pub fn start_address(&self) -> usize {
		self.header_ptr as usize
	}

	pub fn total_size(&self) -> usize {
		self.get_header().total_size as usize
	}

	pub fn end_address(&self) -> usize {
		self.start_address() + self.total_size()
	}

	pub fn get_basic_mem(&self) -> Option<&'static BasicMemoryInfoTag> {
		self.get_tag(TagType::BasicMemoryInfo as u32)
			.map(|tag| unsafe { &*(tag as *const Tag as *const BasicMemoryInfoTag) })
	}

	pub fn get_mem_map(&self) -> Option<&'static MemoryMapTag> {
		self.get_tag(TagType::MemoryMap as u32)
			.map(|tag| unsafe { &*(tag as *const Tag as *const MemoryMapTag) })
	}

	fn get_header(&self) -> &Multiboot2Fixed {
		unsafe { &*self.header_ptr }
	}

	fn get_tag(&self, search_type: u32) -> Option<&'static Tag> {
		self.get_tags().find(|tag| tag.tag_type == search_type)
	}

	fn get_tags(&self) -> TagIterator {
		TagIterator { current_tag: unsafe { self.header_ptr.offset(1) as *const _ } }
	}
}

impl fmt::Debug for Multiboot2Info {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "Multiboot2 Information:")?;
		writeln!(formatter, "Info Structure- Start: {:#X} End: {:#X}",
			self.start_address(), self.end_address())?;

		if let Some(basic_mem_tag) = self.get_basic_mem() {
			writeln!(formatter, "{:?}", basic_mem_tag)?;
		}

		if let Some(mem_map_tag) = self.get_mem_map() {
			writeln!(formatter, "{:?}", mem_map_tag)?;
		}

		Ok(())
	}
}
