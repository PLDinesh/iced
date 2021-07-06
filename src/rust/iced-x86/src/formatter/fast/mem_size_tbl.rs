// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::formatter::fast::FastStringMemorySize;
use crate::iced_constants::IcedConstants;
use alloc::boxed::Box;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use static_assertions::const_assert;

// GENERATOR-BEGIN: MemorySizes
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
static MEM_SIZE_TBL_DATA: [u8; 160] = [
	0x00,
	0x01,
	0x0D,
	0x03,
	0x0B,
	0x0B,
	0x0E,
	0x0F,
	0x10,
	0x01,
	0x0D,
	0x03,
	0x0B,
	0x0E,
	0x0F,
	0x10,
	0x03,
	0x08,
	0x0C,
	0x0D,
	0x03,
	0x0B,
	0x03,
	0x0B,
	0x0B,
	0x09,
	0x08,
	0x08,
	0x0D,
	0x03,
	0x0B,
	0x0C,
	0x0E,
	0x0D,
	0x04,
	0x05,
	0x07,
	0x06,
	0x00,
	0x00,
	0x00,
	0x00,
	0x0C,
	0x10,
	0x00,
	0x0C,
	0x11,
	0x10,
	0x0D,
	0x0D,
	0x03,
	0x03,
	0x03,
	0x03,
	0x03,
	0x03,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x12,
	0x02,
	0x02,
	0x12,
	0x02,
	0x12,
	0x12,
	0x02,
	0x02,
	0x0A,
	0x0A,
	0x0A,
	0x12,
	0x02,
	0x0A,
	0x02,
	0x0A,
	0x0A,
	0x02,
	0x02,
	0x12,
	0x12,
	0x02,
	0x02,
	0x0A,
	0x0A,
	0x0A,
	0x12,
	0x02,
	0x0A,
	0x02,
	0x0A,
	0x0A,
	0x02,
	0x02,
	0x12,
	0x12,
	0x02,
	0x02,
	0x0A,
	0x0A,
	0x0A,
	0x12,
	0x02,
	0x0A,
	0x02,
	0x02,
	0x0A,
	0x0A,
	0x02,
];
static MEM_SIZE_TBL_STRINGS: [&str; 19] = [
	"\x00                ",
	"\x09byte ptr        ",
	"\x0Bdword bcst      ",
	"\x0Adword ptr       ",
	"\x0Dfpuenv14 ptr    ",
	"\x0Dfpuenv28 ptr    ",
	"\x10fpustate108 ptr ",
	"\x0Ffpustate94 ptr  ",
	"\x0Afword ptr       ",
	"\x0Aoword ptr       ",
	"\x0Bqword bcst      ",
	"\x0Aqword ptr       ",
	"\x0Atbyte ptr       ",
	"\x09word ptr        ",
	"\x0Cxmmword ptr     ",
	"\x0Cymmword ptr     ",
	"\x0Czmmword ptr     ",
	"\x0Bmem384 ptr      ",
	"\x0Aword bcst       ",
];
#[allow(dead_code)]
const MAX_MEMORY_SIZE_STR_LEN: usize = 16;
// GENERATOR-END: MemorySizes

lazy_static! {
	pub(super) static ref MEM_SIZE_TBL: Box<[FastStringMemorySize; IcedConstants::MEMORY_SIZE_ENUM_COUNT]> = {
		// If this fails, update the FastStringMemorySize type in fast.rs
		const_assert!(MAX_MEMORY_SIZE_STR_LEN <= FastStringMemorySize::SIZE);
		const_assert!(MAX_MEMORY_SIZE_STR_LEN > FastStringMemorySize::SIZE - 4);

		let mut v = Vec::with_capacity(IcedConstants::MEMORY_SIZE_ENUM_COUNT);
		for &mem_keywords in MEM_SIZE_TBL_DATA.iter() {
			let keywords = MEM_SIZE_TBL_STRINGS[mem_keywords as usize];
			debug_assert!(keywords.len() == 1 + FastStringMemorySize::SIZE);
			v.push(FastStringMemorySize::new(keywords.as_ptr()));
		}
		let v = v.into_boxed_slice();
		debug_assert_eq!(v.len(), IcedConstants::MEMORY_SIZE_ENUM_COUNT);
		// SAFETY: Size is verified above
		unsafe { Box::from_raw(Box::into_raw(v) as *mut [_; IcedConstants::MEMORY_SIZE_ENUM_COUNT]) }
	};
}
