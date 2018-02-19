//tag.rs
//contains structs and methods used to read multiboot2 tags

pub enum TagType {
	BasicMemoryInfo = 4,
	MemoryMap = 6,
}

#[repr(C, packed)]
pub struct Tag {
	pub tag_type: u32,
	size: u32,
}

pub struct TagIterator {
	pub current_tag: *const Tag,
}

impl Iterator for TagIterator {
	type Item = &'static Tag;

	fn next(&mut self) -> Option<&'static Tag> {
		match unsafe { &*self.current_tag } {
			//if current tag is an end tag, return None
			&Tag { tag_type: 0, size: 0x08 } => None,

			//else find the next tag
			next_tag => {
				//get address to current tag
				let mut current_address = self.current_tag as usize;

				//get next tag address, align at 8 bytes
				current_address += ((next_tag.size + 0x07) & !0x07) as usize;
				//get tag from address
				self.current_tag = current_address as *const _;

				Some(next_tag)
			},
		}
	}
}
