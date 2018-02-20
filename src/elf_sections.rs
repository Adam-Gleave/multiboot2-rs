//elf_sections.rs
//contains structs and methods needed to read elf section tags

pub struct ElfSectionsTag {
	ptr: *const ElfSectionsInfo,
}

pub struct ElfSectionsInfo {
	num: u32,
	entry_size: u32,
	shndx: u32,
}

pub struct Section {
	entry: *const u8,
	string_table: *const u8,
	size: u32,
}

pub struct Entry32 {
	name_index: u32,
    entry_type: u32,
    flags: u32,
    address: u32,
    offset: u32,
    size: u32,
    link: u32,
    info: u32,
    addr_align: u32,
	entry_size: u32,
}

pub struct Entry64 {
	name_index: u32,
    entry_type: u32,
    flags: u64,
    address: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    addr_align: u64,
	entry_size: u64,
}

pub fn get_elf_sections_tag(&tag: Tag) {
	let elf_tag = ElfSectionsTag {
		unsafe { 
			ptr: (tag as as *const Tag).offset(1) as *const ElfSectionsInfo,
		}
	};

	elf_tag
}

pub struct ElfSectionsIterator {
	current_section: *const u8,
	remaining: u32,
	entry_size: u32,
	string_table: *const u8,
}

impl ElfSectionsTag {
	fn first_section(&self) -> *const u8 {
		(unsafe { ptr.offset(1) }) as *const _
	}

	fn get_sections(&self) -> ElfSectionsIterator {
		//TODO
	}
}
