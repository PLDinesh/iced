// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::{MemorySize, Register};

// GENERATOR-BEGIN: IcedConstants
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct IcedConstants;
#[allow(dead_code)]
impl IcedConstants {
	pub(crate) const MAX_OP_COUNT: usize = 5;
	pub(crate) const MAX_INSTRUCTION_LENGTH: usize = 15;
	pub(crate) const REGISTER_BITS: u32 = 8;
	pub(crate) const VMM_FIRST: Register = Register::ZMM0;
	pub(crate) const VMM_LAST: Register = Register::ZMM31;
	pub(crate) const VMM_COUNT: u32 = 32;
	pub(crate) const XMM_LAST: Register = Register::XMM31;
	pub(crate) const YMM_LAST: Register = Register::YMM31;
	pub(crate) const ZMM_LAST: Register = Register::ZMM31;
	pub(crate) const TMM_LAST: Register = Register::TMM7;
	pub(crate) const MAX_CPUID_FEATURE_INTERNAL_VALUES: usize = 182;
	pub(crate) const FIRST_BROADCAST_MEMORY_SIZE: MemorySize = MemorySize::Broadcast32_Float16;
	pub(crate) const MVEX_START: u32 = 4611;
	pub(crate) const MVEX_LENGTH: u32 = 207;
	pub(crate) const CC_A_ENUM_COUNT: usize = 2;
	pub(crate) const CC_AE_ENUM_COUNT: usize = 3;
	pub(crate) const CC_B_ENUM_COUNT: usize = 3;
	pub(crate) const CC_BE_ENUM_COUNT: usize = 2;
	pub(crate) const CC_E_ENUM_COUNT: usize = 2;
	pub(crate) const CC_G_ENUM_COUNT: usize = 2;
	pub(crate) const CC_GE_ENUM_COUNT: usize = 2;
	pub(crate) const CC_L_ENUM_COUNT: usize = 2;
	pub(crate) const CC_LE_ENUM_COUNT: usize = 2;
	pub(crate) const CC_NE_ENUM_COUNT: usize = 2;
	pub(crate) const CC_NP_ENUM_COUNT: usize = 2;
	pub(crate) const CC_P_ENUM_COUNT: usize = 2;
	pub(crate) const CODE_ENUM_COUNT: usize = 4818;
	pub(crate) const CODE_SIZE_ENUM_COUNT: usize = 4;
	pub(crate) const CONDITION_CODE_ENUM_COUNT: usize = 17;
	pub(crate) const CPUID_FEATURE_ENUM_COUNT: usize = 161;
	pub(crate) const DECODER_ERROR_ENUM_COUNT: usize = 3;
	pub(crate) const DECORATOR_KIND_ENUM_COUNT: usize = 4;
	pub(crate) const ENCODING_KIND_ENUM_COUNT: usize = 6;
	pub(crate) const FLOW_CONTROL_ENUM_COUNT: usize = 10;
	pub(crate) const FORMATTER_SYNTAX_ENUM_COUNT: usize = 4;
	pub(crate) const FORMATTER_TEXT_KIND_ENUM_COUNT: usize = 16;
	pub(crate) const MANDATORY_PREFIX_ENUM_COUNT: usize = 5;
	pub(crate) const MEMORY_SIZE_ENUM_COUNT: usize = 160;
	pub(crate) const MEMORY_SIZE_OPTIONS_ENUM_COUNT: usize = 4;
	pub(crate) const MNEMONIC_ENUM_COUNT: usize = 1834;
	pub(crate) const MVEX_CONV_FN_ENUM_COUNT: usize = 13;
	pub(crate) const MVEX_EHBIT_ENUM_COUNT: usize = 3;
	pub(crate) const NUMBER_BASE_ENUM_COUNT: usize = 4;
	pub(crate) const NUMBER_KIND_ENUM_COUNT: usize = 8;
	pub(crate) const OP_ACCESS_ENUM_COUNT: usize = 8;
	pub(crate) const OP_CODE_OPERAND_KIND_ENUM_COUNT: usize = 109;
	pub(crate) const OP_CODE_TABLE_KIND_ENUM_COUNT: usize = 9;
	pub(crate) const OP_KIND_ENUM_COUNT: usize = 25;
	pub(crate) const PREFIX_KIND_ENUM_COUNT: usize = 18;
	pub(crate) const REGISTER_ENUM_COUNT: usize = 256;
	pub(crate) const RELOC_KIND_ENUM_COUNT: usize = 1;
	pub(crate) const REP_PREFIX_KIND_ENUM_COUNT: usize = 3;
	pub(crate) const ROUNDING_CONTROL_ENUM_COUNT: usize = 5;
	pub(crate) const TUPLE_TYPE_ENUM_COUNT: usize = 19;
}
// GENERATOR-END: IcedConstants
