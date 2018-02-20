//elf_sections.rs
//contains structs and methods needed to read elf section tags

use core::{ str, slice, fmt };
use tag::Tag;

pub struct ElfSectionsTag {
	ptr: *const ElfSectionsInfo,
}

#[repr(C, packed)]
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

#[derive(PartialEq)]
#[repr(u32)]
pub enum SectionType {
	Unused = 0,
	Program = 1,
	SymbolTable = 2,
	StringTable = 3,
	RelaRelocation = 4,
	SymbolHashTable = 5,
	DynamicLinkTable = 6,
	Note = 7,
	Uninitialized = 8,
	RelRelocation = 9,
	Reserved = 10,
	EnvironmentSpecific = 0x6000_0000,
	ProcessorSpecific = 0x7000_000,
}

#[repr(C, packed)]
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

#[repr(C, packed)]
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

pub fn get_elf_sections_tag(tag: &Tag) -> ElfSectionsTag {
	let elf_tag = ElfSectionsTag {
		ptr: unsafe { (tag as *const Tag).offset(1) } as *const ElfSectionsInfo,
	};

	elf_tag
}

pub struct ElfSectionsIterator {
	current_section: *const u8,
	remaining: u32,
	entry_size: u32,
	string_table: *const u8,
}

impl Iterator for ElfSectionsIterator {
    type Item = Section;

    fn next(&mut self) -> Option<Section> {
        if self.remaining == 0 {
            return None;
        }

        loop {
            let section = Section {
                entry: self.current_section,
                string_table: self.string_table,
                size: self.entry_size,
            };

            self.current_section = unsafe { self.current_section.offset(self.entry_size as isize) };
            self.remaining -= 1;

            if section.get_type() != SectionType::Unused {
                return Some(section);
            }
        }
	}
}

impl ElfSectionsTag {
	fn first_section(&self) -> *const u8 {
		(unsafe { self.ptr.offset(1) }) as *const _
	}

	fn get_sections(&self) -> ElfSectionsIterator {
        let string_table_offset = (self.get().shndx * self.get().entry_size) as isize;

        let string_table_ptr = unsafe {
            self.first_section().offset(string_table_offset) as *const _
        };
        
        ElfSectionsIterator {
            current_section: self.first_section(),
            remaining: self.get().num - 1,
            entry_size: self.get().entry_size,
            string_table: string_table_ptr,
		}
	}

    fn get(&self) -> &ElfSectionsInfo {
        unsafe { &*self.ptr }
	}
}

impl Section {
	pub fn get_type(&self) -> SectionType {
		match self.get().get_type() {
			0 => SectionType::Unused,
			1 => SectionType::Program,
			2 => SectionType::SymbolTable,
			3 => SectionType::StringTable,
			4 => SectionType::RelaRelocation,
			5 => SectionType::SymbolHashTable,
			6 => SectionType::DynamicLinkTable,
			7 => SectionType::Note,
			8 => SectionType::Uninitialized,
			9 => SectionType::RelRelocation,
			10 => SectionType::Reserved,
			0x6000_0000...0x6FFF_FFFF => SectionType::EnvironmentSpecific,
			0x7000_0000...0x7FFF_FFFF => SectionType::ProcessorSpecific,
			_ => panic!(),		
		}
	}

	pub fn get_name(&self) -> &str {
		let ptr = unsafe {
			self.get_string_table().offset(self.get().get_name_index() as isize)
		};

		let len = {
			let mut _len = 0;

			unsafe {
				while *ptr.offset(_len) != 0 {
					_len += 1;
				}

				_len as usize
			}
		};

		str::from_utf8(unsafe { slice::from_raw_parts(ptr, len) }).unwrap()
	}

	pub fn get_start_addr(&self) -> u64 {
		self.get().get_address()
	}

	pub fn get_end_addr(&self) -> u64 {
		self.get().get_address() + self.get().get_size()
	}

	pub fn get_size(&self) -> u64 {
		self.get().get_size()
	}

    pub fn get_flags(&self) -> SectionFlags {
        SectionFlags::from_bits_truncate(self.get().get_flags())
	}

	pub fn get_section_type(&self) -> u32 {
		self.get().get_type()
	}

	fn get(&self) -> &Entry {
		match self.size {
			40 => unsafe { &*(self.entry as *const Entry32) },
			64 => unsafe { &*(self.entry as *const Entry64) },
			_ => panic!(),
		}
	}

	unsafe fn get_string_table(&self) -> *const u8 {
		match self.size {
			40 => (*(self.string_table as *const Entry32)).address as *const _,
			64 => (*(self.string_table as *const Entry64)).address as *const _,
			_ => panic!(),
		}
	}
}

trait Entry {
	fn get_name_index(&self) -> u32;
	fn get_type(&self) -> u32;
	fn get_flags(&self) -> u64;
	fn get_address(&self) -> u64;
	fn get_size(&self) -> u64;
}

impl Entry for Entry32 {
	fn get_name_index(&self) -> u32 {
		self.name_index
	}

	fn get_type(&self) -> u32 {
		self.entry_type
	}

	fn get_flags(&self) -> u64 {
		self.flags as u64
	}

	fn get_address(&self) -> u64 {
		self.address as u64
	}

	fn get_size(&self) -> u64 {
		self.size as u64
	}
}

impl Entry for Entry64 {
	fn get_name_index(&self) -> u32 {
		self.name_index
	}

	fn get_type(&self) -> u32 {
		self.entry_type
	}

	fn get_flags(&self) -> u64 {
		self.flags
	}

	fn get_address(&self) -> u64 {
		self.address
	}

	fn get_size(&self) -> u64 {
		self.size
	}
}

bitflags! {
    pub struct SectionFlags: u64 {
        const WRITABLE = 0x1;
        const ALLOCATED = 0x2;
        const EXECUTABLE = 0x4;
    }
}

impl fmt::Debug for ElfSectionsTag {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "Elf Sections:")?;

		for section in self.get_sections() {
			writeln!(formatter, "Name: {:15}, S: {:#08X}, E: {:#08X}, L: {:#08X}, F: {:#04X}",
				section.get_name(), section.get_start_addr(), section.get_end_addr(), 
				section.get_size(), section.get_flags().bits())?;
		}

		Ok(())
	}
}
