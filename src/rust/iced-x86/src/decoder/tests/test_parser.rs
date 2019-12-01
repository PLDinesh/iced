/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software with&mut restriction, including
with&mut limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITH&mut WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, &mut OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::super::test_utils::from_str_conv::*;
use super::super::super::*;
use super::decoder_test_case::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::iter::IntoIterator;
use std::path::Path;
use std::u32;

pub(crate) struct DecoderTestParser {
	filename: String,
	lines: Lines<BufReader<File>>,
	bitness: u32,
}

impl DecoderTestParser {
	pub fn new(bitness: u32, filename: &Path) -> Self {
		let display_filename = filename.display().to_string();
		let file = File::open(filename).expect(format!("Couldn't open file {}", display_filename).as_str());
		let lines = BufReader::new(file).lines();
		DecoderTestParser {
			filename: display_filename,
			lines,
			bitness,
		}
	}
}

impl IntoIterator for DecoderTestParser {
	type Item = DecoderTestCase;
	type IntoIter = IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			filename: self.filename,
			lines: self.lines,
			bitness: self.bitness,
			line_number: 0,
		}
	}
}

pub(crate) struct IntoIter {
	filename: String,
	lines: Lines<BufReader<File>>,
	bitness: u32,
	line_number: u32,
}

// GENERATOR-BEGIN: DecoderTestText
// ⚠️This was generated by GENERATOR!🦹‍♂️
lazy_static! {
	pub(super) static ref TO_DECODER_TEST_PARSER_CONSTANTS: HashMap<&'static str, u32> = {
		let mut h = HashMap::with_capacity(81);
		let _ = h.insert("noencode", DecoderTestParserConstants::NO_ENCODE);
		let _ = h.insert("bcst", DecoderTestParserConstants::BROADCAST);
		let _ = h.insert("xacquire", DecoderTestParserConstants::XACQUIRE);
		let _ = h.insert("xrelease", DecoderTestParserConstants::XRELEASE);
		let _ = h.insert("rep", DecoderTestParserConstants::REP);
		let _ = h.insert("repe", DecoderTestParserConstants::REPE);
		let _ = h.insert("repne", DecoderTestParserConstants::REPNE);
		let _ = h.insert("lock", DecoderTestParserConstants::LOCK);
		let _ = h.insert("zmsk", DecoderTestParserConstants::ZEROING_MASKING);
		let _ = h.insert("sae", DecoderTestParserConstants::SUPPRESS_ALL_EXCEPTIONS);
		let _ = h.insert("vsib32", DecoderTestParserConstants::VSIB32);
		let _ = h.insert("vsib64", DecoderTestParserConstants::VSIB64);
		let _ = h.insert("rc-rn", DecoderTestParserConstants::ROUND_TO_NEAREST);
		let _ = h.insert("rc-rd", DecoderTestParserConstants::ROUND_DOWN);
		let _ = h.insert("rc-ru", DecoderTestParserConstants::ROUND_UP);
		let _ = h.insert("rc-rz", DecoderTestParserConstants::ROUND_TOWARD_ZERO);
		let _ = h.insert("op0", DecoderTestParserConstants::OP0_KIND);
		let _ = h.insert("op1", DecoderTestParserConstants::OP1_KIND);
		let _ = h.insert("op2", DecoderTestParserConstants::OP2_KIND);
		let _ = h.insert("op3", DecoderTestParserConstants::OP3_KIND);
		let _ = h.insert("op4", DecoderTestParserConstants::OP4_KIND);
		let _ = h.insert("enc", DecoderTestParserConstants::ENCODED_HEX_BYTES);
		let _ = h.insert("amdbr", DecoderTestParserConstants::DECODER_OPTIONS_AMD_BRANCHES);
		let _ = h.insert("resnop", DecoderTestParserConstants::DECODER_OPTIONS_FORCE_RESERVED_NOP);
		let _ = h.insert("umov", DecoderTestParserConstants::DECODER_OPTIONS_UMOV);
		let _ = h.insert("xbts", DecoderTestParserConstants::DECODER_OPTIONS_XBTS);
		let _ = h.insert("cmpxchg486a", DecoderTestParserConstants::DECODER_OPTIONS_CMPXCHG486A);
		let _ = h.insert("oldfpu", DecoderTestParserConstants::DECODER_OPTIONS_OLD_FPU);
		let _ = h.insert("pcommit", DecoderTestParserConstants::DECODER_OPTIONS_PCOMMIT);
		let _ = h.insert("loadall286", DecoderTestParserConstants::DECODER_OPTIONS_LOADALL286);
		let _ = h.insert("loadall386", DecoderTestParserConstants::DECODER_OPTIONS_LOADALL386);
		let _ = h.insert("cl1invmb", DecoderTestParserConstants::DECODER_OPTIONS_CL1INVMB);
		let _ = h.insert("movtr", DecoderTestParserConstants::DECODER_OPTIONS_MOV_TR);
		let _ = h.insert("jmpe", DecoderTestParserConstants::DECODER_OPTIONS_JMPE);
		let _ = h.insert("nopause", DecoderTestParserConstants::DECODER_OPTIONS_NO_PAUSE);
		let _ = h.insert("nowbnoinvd", DecoderTestParserConstants::DECODER_OPTIONS_NO_WBNOINVD);
		let _ = h.insert("nolockmovcr0", DecoderTestParserConstants::DECODER_OPTIONS_NO_LOCK_MOV_CR0);
		let _ = h.insert("nompfx_0fbc", DecoderTestParserConstants::DECODER_OPTIONS_NO_MPFX_0FBC);
		let _ = h.insert("nompfx_0fbd", DecoderTestParserConstants::DECODER_OPTIONS_NO_MPFX_0FBD);
		let _ = h.insert("nolahfsahf64", DecoderTestParserConstants::DECODER_OPTIONS_NO_LAHF_SAHF_64);
		let _ = h.insert("noinvalidcheck", DecoderTestParserConstants::DECODER_OPTIONS_NO_INVALID_CHECK);
		let _ = h.insert("es:", DecoderTestParserConstants::SEGMENT_PREFIX_ES);
		let _ = h.insert("cs:", DecoderTestParserConstants::SEGMENT_PREFIX_CS);
		let _ = h.insert("ss:", DecoderTestParserConstants::SEGMENT_PREFIX_SS);
		let _ = h.insert("ds:", DecoderTestParserConstants::SEGMENT_PREFIX_DS);
		let _ = h.insert("fs:", DecoderTestParserConstants::SEGMENT_PREFIX_FS);
		let _ = h.insert("gs:", DecoderTestParserConstants::SEGMENT_PREFIX_GS);
		let _ = h.insert("k1", DecoderTestParserConstants::OP_MASK_K1);
		let _ = h.insert("k2", DecoderTestParserConstants::OP_MASK_K2);
		let _ = h.insert("k3", DecoderTestParserConstants::OP_MASK_K3);
		let _ = h.insert("k4", DecoderTestParserConstants::OP_MASK_K4);
		let _ = h.insert("k5", DecoderTestParserConstants::OP_MASK_K5);
		let _ = h.insert("k6", DecoderTestParserConstants::OP_MASK_K6);
		let _ = h.insert("k7", DecoderTestParserConstants::OP_MASK_K7);
		let _ = h.insert("co", DecoderTestParserConstants::CONSTANT_OFFSETS);
		let _ = h.insert("r", DecoderTestParserConstants::OP_KIND_REGISTER);
		let _ = h.insert("nb16", DecoderTestParserConstants::OP_KIND_NEAR_BRANCH16);
		let _ = h.insert("nb32", DecoderTestParserConstants::OP_KIND_NEAR_BRANCH32);
		let _ = h.insert("nb64", DecoderTestParserConstants::OP_KIND_NEAR_BRANCH64);
		let _ = h.insert("fb16", DecoderTestParserConstants::OP_KIND_FAR_BRANCH16);
		let _ = h.insert("fb32", DecoderTestParserConstants::OP_KIND_FAR_BRANCH32);
		let _ = h.insert("i8", DecoderTestParserConstants::OP_KIND_IMMEDIATE8);
		let _ = h.insert("i16", DecoderTestParserConstants::OP_KIND_IMMEDIATE16);
		let _ = h.insert("i32", DecoderTestParserConstants::OP_KIND_IMMEDIATE32);
		let _ = h.insert("i64", DecoderTestParserConstants::OP_KIND_IMMEDIATE64);
		let _ = h.insert("i8to16", DecoderTestParserConstants::OP_KIND_IMMEDIATE8TO16);
		let _ = h.insert("i8to32", DecoderTestParserConstants::OP_KIND_IMMEDIATE8TO32);
		let _ = h.insert("i8to64", DecoderTestParserConstants::OP_KIND_IMMEDIATE8TO64);
		let _ = h.insert("i32to64", DecoderTestParserConstants::OP_KIND_IMMEDIATE32TO64);
		let _ = h.insert("i8_2nd", DecoderTestParserConstants::OP_KIND_IMMEDIATE8_2ND);
		let _ = h.insert("segsi", DecoderTestParserConstants::OP_KIND_MEMORY_SEG_SI);
		let _ = h.insert("segesi", DecoderTestParserConstants::OP_KIND_MEMORY_SEG_ESI);
		let _ = h.insert("segrsi", DecoderTestParserConstants::OP_KIND_MEMORY_SEG_RSI);
		let _ = h.insert("segdi", DecoderTestParserConstants::OP_KIND_MEMORY_SEG_DI);
		let _ = h.insert("segedi", DecoderTestParserConstants::OP_KIND_MEMORY_SEG_EDI);
		let _ = h.insert("segrdi", DecoderTestParserConstants::OP_KIND_MEMORY_SEG_RDI);
		let _ = h.insert("esdi", DecoderTestParserConstants::OP_KIND_MEMORY_ES_DI);
		let _ = h.insert("esedi", DecoderTestParserConstants::OP_KIND_MEMORY_ES_EDI);
		let _ = h.insert("esrdi", DecoderTestParserConstants::OP_KIND_MEMORY_ES_RDI);
		let _ = h.insert("m64", DecoderTestParserConstants::OP_KIND_MEMORY64);
		let _ = h.insert("m", DecoderTestParserConstants::OP_KIND_MEMORY);
		h
	};
}

pub(crate) struct DecoderTestParserConstants;
impl DecoderTestParserConstants {
	pub(crate) const NO_ENCODE: u32 = 0;
	pub(crate) const BROADCAST: u32 = 1;
	pub(crate) const XACQUIRE: u32 = 2;
	pub(crate) const XRELEASE: u32 = 3;
	pub(crate) const REP: u32 = 4;
	pub(crate) const REPE: u32 = 5;
	pub(crate) const REPNE: u32 = 6;
	pub(crate) const LOCK: u32 = 7;
	pub(crate) const ZEROING_MASKING: u32 = 8;
	pub(crate) const SUPPRESS_ALL_EXCEPTIONS: u32 = 9;
	pub(crate) const VSIB32: u32 = 10;
	pub(crate) const VSIB64: u32 = 11;
	pub(crate) const ROUND_TO_NEAREST: u32 = 12;
	pub(crate) const ROUND_DOWN: u32 = 13;
	pub(crate) const ROUND_UP: u32 = 14;
	pub(crate) const ROUND_TOWARD_ZERO: u32 = 15;
	pub(crate) const OP0_KIND: u32 = 16;
	pub(crate) const OP1_KIND: u32 = 17;
	pub(crate) const OP2_KIND: u32 = 18;
	pub(crate) const OP3_KIND: u32 = 19;
	pub(crate) const OP4_KIND: u32 = 20;
	pub(crate) const ENCODED_HEX_BYTES: u32 = 21;
	pub(crate) const DECODER_OPTIONS_AMD_BRANCHES: u32 = 22;
	pub(crate) const DECODER_OPTIONS_FORCE_RESERVED_NOP: u32 = 23;
	pub(crate) const DECODER_OPTIONS_UMOV: u32 = 24;
	pub(crate) const DECODER_OPTIONS_XBTS: u32 = 25;
	pub(crate) const DECODER_OPTIONS_CMPXCHG486A: u32 = 26;
	pub(crate) const DECODER_OPTIONS_OLD_FPU: u32 = 27;
	pub(crate) const DECODER_OPTIONS_PCOMMIT: u32 = 28;
	pub(crate) const DECODER_OPTIONS_LOADALL286: u32 = 29;
	pub(crate) const DECODER_OPTIONS_LOADALL386: u32 = 30;
	pub(crate) const DECODER_OPTIONS_CL1INVMB: u32 = 31;
	pub(crate) const DECODER_OPTIONS_MOV_TR: u32 = 32;
	pub(crate) const DECODER_OPTIONS_JMPE: u32 = 33;
	pub(crate) const DECODER_OPTIONS_NO_PAUSE: u32 = 34;
	pub(crate) const DECODER_OPTIONS_NO_WBNOINVD: u32 = 35;
	pub(crate) const DECODER_OPTIONS_NO_LOCK_MOV_CR0: u32 = 36;
	pub(crate) const DECODER_OPTIONS_NO_MPFX_0FBC: u32 = 37;
	pub(crate) const DECODER_OPTIONS_NO_MPFX_0FBD: u32 = 38;
	pub(crate) const DECODER_OPTIONS_NO_LAHF_SAHF_64: u32 = 39;
	pub(crate) const DECODER_OPTIONS_NO_INVALID_CHECK: u32 = 40;
	pub(crate) const SEGMENT_PREFIX_ES: u32 = 41;
	pub(crate) const SEGMENT_PREFIX_CS: u32 = 42;
	pub(crate) const SEGMENT_PREFIX_SS: u32 = 43;
	pub(crate) const SEGMENT_PREFIX_DS: u32 = 44;
	pub(crate) const SEGMENT_PREFIX_FS: u32 = 45;
	pub(crate) const SEGMENT_PREFIX_GS: u32 = 46;
	pub(crate) const OP_MASK_K1: u32 = 47;
	pub(crate) const OP_MASK_K2: u32 = 48;
	pub(crate) const OP_MASK_K3: u32 = 49;
	pub(crate) const OP_MASK_K4: u32 = 50;
	pub(crate) const OP_MASK_K5: u32 = 51;
	pub(crate) const OP_MASK_K6: u32 = 52;
	pub(crate) const OP_MASK_K7: u32 = 53;
	pub(crate) const CONSTANT_OFFSETS: u32 = 54;
	pub(crate) const OP_KIND_REGISTER: u32 = 55;
	pub(crate) const OP_KIND_NEAR_BRANCH16: u32 = 56;
	pub(crate) const OP_KIND_NEAR_BRANCH32: u32 = 57;
	pub(crate) const OP_KIND_NEAR_BRANCH64: u32 = 58;
	pub(crate) const OP_KIND_FAR_BRANCH16: u32 = 59;
	pub(crate) const OP_KIND_FAR_BRANCH32: u32 = 60;
	pub(crate) const OP_KIND_IMMEDIATE8: u32 = 61;
	pub(crate) const OP_KIND_IMMEDIATE16: u32 = 62;
	pub(crate) const OP_KIND_IMMEDIATE32: u32 = 63;
	pub(crate) const OP_KIND_IMMEDIATE64: u32 = 64;
	pub(crate) const OP_KIND_IMMEDIATE8TO16: u32 = 65;
	pub(crate) const OP_KIND_IMMEDIATE8TO32: u32 = 66;
	pub(crate) const OP_KIND_IMMEDIATE8TO64: u32 = 67;
	pub(crate) const OP_KIND_IMMEDIATE32TO64: u32 = 68;
	pub(crate) const OP_KIND_IMMEDIATE8_2ND: u32 = 69;
	pub(crate) const OP_KIND_MEMORY_SEG_SI: u32 = 70;
	pub(crate) const OP_KIND_MEMORY_SEG_ESI: u32 = 71;
	pub(crate) const OP_KIND_MEMORY_SEG_RSI: u32 = 72;
	pub(crate) const OP_KIND_MEMORY_SEG_DI: u32 = 73;
	pub(crate) const OP_KIND_MEMORY_SEG_EDI: u32 = 74;
	pub(crate) const OP_KIND_MEMORY_SEG_RDI: u32 = 75;
	pub(crate) const OP_KIND_MEMORY_ES_DI: u32 = 76;
	pub(crate) const OP_KIND_MEMORY_ES_EDI: u32 = 77;
	pub(crate) const OP_KIND_MEMORY_ES_RDI: u32 = 78;
	pub(crate) const OP_KIND_MEMORY64: u32 = 79;
	pub(crate) const OP_KIND_MEMORY: u32 = 80;
}
// GENERATOR-END: DecoderTestText

impl Iterator for IntoIter {
	type Item = DecoderTestCase;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.lines.next() {
				None => return None,
				Some(info) => {
					self.line_number += 1;
					let result = match info {
						Ok(line) => {
							if line.is_empty() || line.starts_with("#") {
								continue;
							}
							self.read_next_test_case(line, self.line_number)
						}
						Err(err) => Err(err.to_string()),
					};
					match result {
						Ok(tc) => return Some(tc),
						Err(err) => panic!(
							"Error parsing decoder test case file '{}', line {}: {}",
							self.filename, self.line_number, err
						),
					}
				}
			}
		}
	}
}

impl IntoIter {
	fn read_next_test_case(&self, line: String, line_number: u32) -> Result<DecoderTestCase, String> {
		let parts: Vec<&str> = line.split(",").collect();
		if parts.len() != 5 {
			return Err(format!("Invalid number of commas ({} commas)", parts.len() - 1));
		}

		let mut tc: DecoderTestCase = Default::default();
		tc.line_number = line_number;
		tc.can_encode = true;
		tc.bitness = self.bitness;
		tc.hex_bytes = parts[0].trim().to_string();
		let _ = to_vec_u8(&tc.hex_bytes)?;
		tc.encoded_hex_bytes = tc.hex_bytes.clone();
		tc.code = to_code(parts[1])?;
		tc.mnemonic = to_mnemonic(parts[2])?;
		tc.op_count = to_u32(parts[3])?;

		for key in parts[4].split(" ") {
			let mut key = key;
			if key.is_empty() {
				continue;
			}
			let kv_parts: Vec<&str> = key.splitn(2, "=").collect();
			let value = if kv_parts.len() == 1 {
				""
			} else {
				assert_eq!(2, kv_parts.len());
				key = kv_parts[0];
				kv_parts[1]
			};

			match *(*TO_DECODER_TEST_PARSER_CONSTANTS).get(key).unwrap_or(&u32::MAX) {
				DecoderTestParserConstants::NO_ENCODE => tc.can_encode = false,
				DecoderTestParserConstants::BROADCAST => tc.is_broadcast = true,
				DecoderTestParserConstants::XACQUIRE => tc.has_xacquire_prefix = true,
				DecoderTestParserConstants::XRELEASE => tc.has_xrelease_prefix = true,
				DecoderTestParserConstants::REP | DecoderTestParserConstants::REPE => tc.has_repe_prefix = true,
				DecoderTestParserConstants::REPNE => tc.has_repne_prefix = true,
				DecoderTestParserConstants::LOCK => tc.has_lock_prefix = true,
				DecoderTestParserConstants::ZEROING_MASKING => tc.zeroing_masking = true,
				DecoderTestParserConstants::SUPPRESS_ALL_EXCEPTIONS => tc.suppress_all_exceptions = true,
				DecoderTestParserConstants::VSIB32 => tc.vsib_bitness = 32,
				DecoderTestParserConstants::VSIB64 => tc.vsib_bitness = 64,
				DecoderTestParserConstants::ROUND_TO_NEAREST => tc.rounding_control = RoundingControl::RoundToNearest,
				DecoderTestParserConstants::ROUND_DOWN => tc.rounding_control = RoundingControl::RoundDown,
				DecoderTestParserConstants::ROUND_UP => tc.rounding_control = RoundingControl::RoundUp,
				DecoderTestParserConstants::ROUND_TOWARD_ZERO => tc.rounding_control = RoundingControl::RoundTowardZero,
				DecoderTestParserConstants::OP0_KIND => {
					if tc.op_count < 1 {
						return Err(format!("Invalid OpCount: {} < 1", tc.op_count));
					}
					self.read_op_kind(&mut tc, 0, value)?;
				}

				DecoderTestParserConstants::OP1_KIND => {
					if tc.op_count < 2 {
						return Err(format!("Invalid OpCount: {} < 2", tc.op_count));
					}
					self.read_op_kind(&mut tc, 1, value)?;
				}

				DecoderTestParserConstants::OP2_KIND => {
					if tc.op_count < 3 {
						return Err(format!("Invalid OpCount: {} < 3", tc.op_count));
					}
					self.read_op_kind(&mut tc, 2, value)?;
				}

				DecoderTestParserConstants::OP3_KIND => {
					if tc.op_count < 4 {
						return Err(format!("Invalid OpCount: {} < 4", tc.op_count));
					}
					self.read_op_kind(&mut tc, 3, value)?;
				}

				DecoderTestParserConstants::OP4_KIND => {
					if tc.op_count < 5 {
						return Err(format!("Invalid OpCount: {} < 5", tc.op_count));
					}
					self.read_op_kind(&mut tc, 4, value)?;
				}

				DecoderTestParserConstants::ENCODED_HEX_BYTES => {
					if value.is_empty() {
						return Err(format!("Invalid encoded hex bytes: '{}'", value));
					}
					tc.encoded_hex_bytes = value.to_string();
					let _ = to_vec_u8(&tc.encoded_hex_bytes)?;
				}

				DecoderTestParserConstants::DECODER_OPTIONS_AMD_BRANCHES => tc.decoder_options |= DecoderOptions::AMD_BRANCHES,
				DecoderTestParserConstants::DECODER_OPTIONS_FORCE_RESERVED_NOP => tc.decoder_options |= DecoderOptions::FORCE_RESERVED_NOP,
				DecoderTestParserConstants::DECODER_OPTIONS_UMOV => tc.decoder_options |= DecoderOptions::UMOV,
				DecoderTestParserConstants::DECODER_OPTIONS_XBTS => tc.decoder_options |= DecoderOptions::XBTS,
				DecoderTestParserConstants::DECODER_OPTIONS_CMPXCHG486A => tc.decoder_options |= DecoderOptions::CMPXCHG486A,
				DecoderTestParserConstants::DECODER_OPTIONS_OLD_FPU => tc.decoder_options |= DecoderOptions::OLD_FPU,
				DecoderTestParserConstants::DECODER_OPTIONS_PCOMMIT => tc.decoder_options |= DecoderOptions::PCOMMIT,
				DecoderTestParserConstants::DECODER_OPTIONS_LOADALL286 => tc.decoder_options |= DecoderOptions::LOADALL286,
				DecoderTestParserConstants::DECODER_OPTIONS_LOADALL386 => tc.decoder_options |= DecoderOptions::LOADALL386,
				DecoderTestParserConstants::DECODER_OPTIONS_CL1INVMB => tc.decoder_options |= DecoderOptions::CL1INVMB,
				DecoderTestParserConstants::DECODER_OPTIONS_MOV_TR => tc.decoder_options |= DecoderOptions::MOV_TR,
				DecoderTestParserConstants::DECODER_OPTIONS_JMPE => tc.decoder_options |= DecoderOptions::JMPE,
				DecoderTestParserConstants::DECODER_OPTIONS_NO_PAUSE => tc.decoder_options |= DecoderOptions::NO_PAUSE,
				DecoderTestParserConstants::DECODER_OPTIONS_NO_WBNOINVD => tc.decoder_options |= DecoderOptions::NO_WBNOINVD,
				DecoderTestParserConstants::DECODER_OPTIONS_NO_LOCK_MOV_CR0 => tc.decoder_options |= DecoderOptions::NO_LOCK_MOV_CR0,
				DecoderTestParserConstants::DECODER_OPTIONS_NO_MPFX_0FBC => tc.decoder_options |= DecoderOptions::NO_MPFX_0FBC,
				DecoderTestParserConstants::DECODER_OPTIONS_NO_MPFX_0FBD => tc.decoder_options |= DecoderOptions::NO_MPFX_0FBD,
				DecoderTestParserConstants::DECODER_OPTIONS_NO_LAHF_SAHF_64 => tc.decoder_options |= DecoderOptions::NO_LAHF_SAHF_64,
				DecoderTestParserConstants::DECODER_OPTIONS_NO_INVALID_CHECK => tc.decoder_options |= DecoderOptions::NO_INVALID_CHECK,
				DecoderTestParserConstants::SEGMENT_PREFIX_ES => tc.segment_prefix = Register::ES,
				DecoderTestParserConstants::SEGMENT_PREFIX_CS => tc.segment_prefix = Register::CS,
				DecoderTestParserConstants::SEGMENT_PREFIX_SS => tc.segment_prefix = Register::SS,
				DecoderTestParserConstants::SEGMENT_PREFIX_DS => tc.segment_prefix = Register::DS,
				DecoderTestParserConstants::SEGMENT_PREFIX_FS => tc.segment_prefix = Register::FS,
				DecoderTestParserConstants::SEGMENT_PREFIX_GS => tc.segment_prefix = Register::GS,
				DecoderTestParserConstants::OP_MASK_K1 => tc.op_mask = Register::K1,
				DecoderTestParserConstants::OP_MASK_K2 => tc.op_mask = Register::K2,
				DecoderTestParserConstants::OP_MASK_K3 => tc.op_mask = Register::K3,
				DecoderTestParserConstants::OP_MASK_K4 => tc.op_mask = Register::K4,
				DecoderTestParserConstants::OP_MASK_K5 => tc.op_mask = Register::K5,
				DecoderTestParserConstants::OP_MASK_K6 => tc.op_mask = Register::K6,
				DecoderTestParserConstants::OP_MASK_K7 => tc.op_mask = Register::K7,
				DecoderTestParserConstants::CONSTANT_OFFSETS => tc.constant_offsets = parse_constant_offsets(value)?,
				_ => return Err(format!("Invalid key '{}'", key)),
			}
		}

		Ok(tc)
	}

	fn read_op_kind(&self, tc: &mut DecoderTestCase, operand: u32, value: &str) -> Result<(), String> {
		let parts: Vec<&str> = value.split(";").collect();
		match *(*TO_DECODER_TEST_PARSER_CONSTANTS).get(parts[0]).unwrap_or(&u32::MAX) {
			DecoderTestParserConstants::OP_KIND_REGISTER => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_register(operand, to_register(parts[1])?);
				tc.set_op_kind(operand, OpKind::Register);
			}

			DecoderTestParserConstants::OP_KIND_NEAR_BRANCH16 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::NearBranch16);
				tc.near_branch = to_u16(parts[1])? as u64;
			}

			DecoderTestParserConstants::OP_KIND_NEAR_BRANCH32 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::NearBranch32);
				tc.near_branch = to_u32(parts[1])? as u64;
			}

			DecoderTestParserConstants::OP_KIND_NEAR_BRANCH64 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::NearBranch64);
				tc.near_branch = to_u64(parts[1])?;
			}

			DecoderTestParserConstants::OP_KIND_FAR_BRANCH16 => {
				if parts.len() != 3 {
					return Err(format!("Operand {}: expected 3 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::FarBranch16);
				tc.far_branch_selector = to_u16(parts[1])?;
				tc.far_branch = to_u16(parts[2])? as u32;
			}

			DecoderTestParserConstants::OP_KIND_FAR_BRANCH32 => {
				if parts.len() != 3 {
					return Err(format!("Operand {}: expected 3 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::FarBranch32);
				tc.far_branch_selector = to_u16(parts[1])?;
				tc.far_branch = to_u32(parts[2])?;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE8 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate8);
				tc.immediate = to_u8(parts[1])? as u64;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE16 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate16);
				tc.immediate = to_u16(parts[1])? as u64;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE32 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate32);
				tc.immediate = to_u32(parts[1])? as u64;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE64 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate64);
				tc.immediate = to_u64(parts[1])?;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE8TO16 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate8to16);
				tc.immediate = to_u16(parts[1])? as u64;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE8TO32 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate8to32);
				tc.immediate = to_u32(parts[1])? as u64;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE8TO64 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate8to64);
				tc.immediate = to_u64(parts[1])?;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE32TO64 => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate32to64);
				tc.immediate = to_u64(parts[1])?;
			}

			DecoderTestParserConstants::OP_KIND_IMMEDIATE8_2ND => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Immediate8_2nd);
				tc.immediate_2nd = to_u8(parts[1])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_SEG_SI => {
				if parts.len() != 3 {
					return Err(format!("Operand {}: expected 3 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemorySegSI);
				tc.memory_segment = to_register(parts[1])?;
				tc.memory_size = to_memory_size(parts[2])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_SEG_ESI => {
				if parts.len() != 3 {
					return Err(format!("Operand {}: expected 3 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemorySegESI);
				tc.memory_segment = to_register(parts[1])?;
				tc.memory_size = to_memory_size(parts[2])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_SEG_RSI => {
				if parts.len() != 3 {
					return Err(format!("Operand {}: expected 3 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemorySegRSI);
				tc.memory_segment = to_register(parts[1])?;
				tc.memory_size = to_memory_size(parts[2])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_SEG_DI => {
				if parts.len() != 3 {
					return Err(format!("Operand {}: expected 3 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemorySegDI);
				tc.memory_segment = to_register(parts[1])?;
				tc.memory_size = to_memory_size(parts[2])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_SEG_EDI => {
				if parts.len() != 3 {
					return Err(format!("Operand {}: expected 3 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemorySegEDI);
				tc.memory_segment = to_register(parts[1])?;
				tc.memory_size = to_memory_size(parts[2])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_SEG_RDI => {
				if parts.len() != 3 {
					return Err(format!("Operand {}: expected 3 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemorySegRDI);
				tc.memory_segment = to_register(parts[1])?;
				tc.memory_size = to_memory_size(parts[2])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_ES_DI => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemoryESDI);
				tc.memory_size = to_memory_size(parts[1])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_ES_EDI => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemoryESEDI);
				tc.memory_size = to_memory_size(parts[1])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY_ES_RDI => {
				if parts.len() != 2 {
					return Err(format!("Operand {}: expected 2 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::MemoryESRDI);
				tc.memory_size = to_memory_size(parts[1])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY64 => {
				if parts.len() != 4 {
					return Err(format!("Operand {}: expected 4 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Memory64);
				tc.memory_segment = to_register(parts[1])?;
				tc.memory_address64 = to_u64(parts[2])?;
				tc.memory_size = to_memory_size(parts[3])?;
			}

			DecoderTestParserConstants::OP_KIND_MEMORY => {
				if parts.len() != 8 {
					return Err(format!("Operand {}: expected 8 values, actual = {}", operand, parts.len()));
				}
				tc.set_op_kind(operand, OpKind::Memory);
				tc.memory_segment = to_register(parts[1])?;
				tc.memory_base = to_register(parts[2])?;
				tc.memory_index = to_register(parts[3])?;
				tc.memory_index_scale = to_u32(parts[4])?;
				tc.memory_displacement = to_u32(parts[5])?;
				tc.memory_displ_size = to_u32(parts[6])?;
				tc.memory_size = to_memory_size(parts[7])?;
			}

			_ => return Err(format!("Invalid opkind: '{}'", parts[0])),
		}
		Ok(())
	}
}

pub(crate) fn parse_constant_offsets(value: &str) -> Result<ConstantOffsets, String> {
	let parts: Vec<&str> = value.split(";").collect();
	if parts.len() != 6 {
		return Err(format!("Invalid ConstantOffsets: '{}'", value));
	}
	let mut constant_offsets: ConstantOffsets = Default::default();
	constant_offsets.immediate_offset = to_u8(parts[0])?;
	constant_offsets.immediate_size = to_u8(parts[1])?;
	constant_offsets.immediate_offset2 = to_u8(parts[2])?;
	constant_offsets.immediate_size2 = to_u8(parts[3])?;
	constant_offsets.displacement_offset = to_u8(parts[4])?;
	constant_offsets.displacement_size = to_u8(parts[5])?;
	Ok(constant_offsets)
}
