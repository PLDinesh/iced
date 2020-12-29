/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::super::iced_constants::IcedConstants;
use super::super::fmt_consts::*;
use super::FormatterString;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

pub(super) struct Info {
	pub(super) keywords: &'static [&'static FormatterString],
	pub(super) size: u32,
	pub(super) is_broadcast: bool,
}

// GENERATOR-BEGIN: ConstData
// ⚠️This was generated by GENERATOR!🦹‍♂️
const SIZE_KIND_SHIFT: u32 = 5;
const MEMORY_KEYWORDS_MASK: u16 = 31;
#[rustfmt::skip]
static SIZES: [u16; 16] = [
	0,
	1,
	2,
	4,
	6,
	8,
	10,
	14,
	16,
	28,
	32,
	48,
	64,
	94,
	108,
	512,
];
// GENERATOR-END: ConstData

// GENERATOR-BEGIN: MemorySizes
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
static MEM_SIZE_TBL_DATA: [u16; 141] = [
	0x0000,
	0x0021,
	0x004D,
	0x0063,
	0x00AB,
	0x00AB,
	0x010E,
	0x014F,
	0x0190,
	0x0021,
	0x004D,
	0x0063,
	0x00AB,
	0x010E,
	0x014F,
	0x0190,
	0x0063,
	0x0088,
	0x00CC,
	0x004D,
	0x0063,
	0x00AB,
	0x0063,
	0x00AB,
	0x00AB,
	0x0109,
	0x0088,
	0x00C8,
	0x004D,
	0x0063,
	0x00AB,
	0x00CC,
	0x010E,
	0x004D,
	0x00E4,
	0x0125,
	0x01A7,
	0x01C6,
	0x01E0,
	0x01E0,
	0x0000,
	0x0000,
	0x00CC,
	0x0190,
	0x0000,
	0x00CC,
	0x0171,
	0x0190,
	0x004D,
	0x004D,
	0x0063,
	0x0063,
	0x0063,
	0x0063,
	0x0063,
	0x00AB,
	0x00AB,
	0x00AB,
	0x00AB,
	0x00AB,
	0x00AB,
	0x00AB,
	0x00AB,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x010E,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x014F,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0190,
	0x0062,
	0x0062,
	0x0062,
	0x0062,
	0x0062,
	0x00AA,
	0x00AA,
	0x00AA,
	0x0062,
	0x00AA,
	0x0062,
	0x0062,
	0x00AA,
	0x00AA,
	0x00AA,
	0x0062,
	0x00AA,
	0x0062,
	0x0062,
	0x00AA,
	0x00AA,
	0x00AA,
	0x0062,
	0x00AA,
	0x0062,
	0x0062,
	0x0062,
	0x00AA,
	0x00AA,
	0x00AA,
	0x00AA,
	0x00AA,
	0x00AA,
	0x0062,
	0x0062,
	0x0062,
];
// GENERATOR-END: MemorySizes

lazy_static! {
	pub(super) static ref MEM_SIZE_TBL: Vec<Info> = {
		let mut v = Vec::with_capacity(IcedConstants::MEMORY_SIZE_ENUM_COUNT);
		let ac = &*ARRAY_CONSTS;
		for (i, &d) in MEM_SIZE_TBL_DATA.iter().enumerate() {
			let keywords: &'static [&'static FormatterString] = match d & MEMORY_KEYWORDS_MASK {
				// GENERATOR-BEGIN: MemoryKeywordsMatch
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				0x00 => &ac.nothing,
				0x01 => &ac.byte_ptr,
				0x02 => &ac.dword_bcst,
				0x03 => &ac.dword_ptr,
				0x04 => &ac.fpuenv14_ptr,
				0x05 => &ac.fpuenv28_ptr,
				0x06 => &ac.fpustate108_ptr,
				0x07 => &ac.fpustate94_ptr,
				0x08 => &ac.fword_ptr,
				0x09 => &ac.oword_ptr,
				0x0A => &ac.qword_bcst,
				0x0B => &ac.qword_ptr,
				0x0C => &ac.tbyte_ptr,
				0x0D => &ac.word_ptr,
				0x0E => &ac.xmmword_ptr,
				0x0F => &ac.ymmword_ptr,
				0x10 => &ac.zmmword_ptr,
				0x11 => &ac.mem384_ptr,
				// GENERATOR-END: MemoryKeywordsMatch
				_ => unreachable!(),
			};
			let size = SIZES[d as usize >> SIZE_KIND_SHIFT] as u32;
			v.push(Info { keywords, size, is_broadcast: i >= IcedConstants::FIRST_BROADCAST_MEMORY_SIZE as usize });
		}
		v
	};
}
