// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::formatter::iced_constants::IcedConstants;
use crate::formatter::iced_error::IcedError;
use core::iter::{ExactSizeIterator, FusedIterator, Iterator};
use core::{fmt, mem};

// GENERATOR-BEGIN: SymbolFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Symbol flags
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct SymbolFlags;
impl SymbolFlags {
	/// No bit is set
	pub const NONE: u32 = 0x0000_0000;
	/// It's a symbol relative to a register, eg. a struct offset `[ebx+some_struct.field1]`. If this is cleared, it's the address of a symbol.
	pub const RELATIVE: u32 = 0x0000_0001;
	/// It's a signed symbol and it should be displayed as `-symbol` or `reg-symbol` instead of `symbol` or `reg+symbol`
	pub const SIGNED: u32 = 0x0000_0002;
}
// GENERATOR-END: SymbolFlags

// GENERATOR-BEGIN: FormatterTextKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Formatter text kind
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum FormatterTextKind {
	/// Normal text
	Text = 0,
	/// Assembler directive
	Directive = 1,
	/// Any prefix
	Prefix = 2,
	/// Any mnemonic
	Mnemonic = 3,
	/// Any keyword
	Keyword = 4,
	/// Any operator
	Operator = 5,
	/// Any punctuation
	Punctuation = 6,
	/// Number
	Number = 7,
	/// Any register
	Register = 8,
	/// A decorator, eg. `sae` in `{sae}`
	Decorator = 9,
	/// Selector value (eg. far `JMP`/`CALL`)
	SelectorValue = 10,
	/// Label address (eg. `JE XXXXXX`)
	LabelAddress = 11,
	/// Function address (eg. `CALL XXXXXX`)
	FunctionAddress = 12,
	/// Data symbol
	Data = 13,
	/// Label symbol
	Label = 14,
	/// Function symbol
	Function = 15,
}
#[rustfmt::skip]
static GEN_DEBUG_FORMATTER_TEXT_KIND: [&str; 16] = [
	"Text",
	"Directive",
	"Prefix",
	"Mnemonic",
	"Keyword",
	"Operator",
	"Punctuation",
	"Number",
	"Register",
	"Decorator",
	"SelectorValue",
	"LabelAddress",
	"FunctionAddress",
	"Data",
	"Label",
	"Function",
];
impl fmt::Debug for FormatterTextKind {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_FORMATTER_TEXT_KIND[*self as usize])
	}
}
impl Default for FormatterTextKind {
	#[must_use]
	#[inline]
	fn default() -> Self {
		FormatterTextKind::Text
	}
}
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) type FormatterTextKindUnderlyingType = u8;
#[rustfmt::skip]
impl FormatterTextKind {
	/// Iterates over all `FormatterTextKind` enum values
	#[inline]
	pub fn values() -> impl Iterator<Item = FormatterTextKind> + DoubleEndedIterator + ExactSizeIterator + FusedIterator {
		// SAFETY: all values 0-max are valid enum values
		(0..IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT).map(|x| unsafe { mem::transmute::<u8, FormatterTextKind>(x as u8) })
	}
}
#[test]
#[rustfmt::skip]
fn test_formattertextkind_values() {
	let mut iter = FormatterTextKind::values();
	assert_eq!(iter.size_hint(), (IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT, Some(IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT)));
	assert_eq!(iter.len(), IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT);
	assert!(iter.next().is_some());
	assert_eq!(iter.size_hint(), (IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT - 1, Some(IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT - 1)));
	assert_eq!(iter.len(), IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT - 1);

	let values: Vec<FormatterTextKind> = FormatterTextKind::values().collect();
	assert_eq!(values.len(), IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT);
	for (i, value) in values.into_iter().enumerate() {
		assert_eq!(i, value as usize);
	}

	let values1: Vec<FormatterTextKind> = FormatterTextKind::values().collect();
	let mut values2: Vec<FormatterTextKind> = FormatterTextKind::values().rev().collect();
	values2.reverse();
	assert_eq!(values1, values2);
}
#[rustfmt::skip]
impl TryFrom<usize> for FormatterTextKind {
	type Error = IcedError;
	#[inline]
	fn try_from(value: usize) -> Result<Self, Self::Error> {
		if value < IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT {
			// SAFETY: all values 0-max are valid enum values
			Ok(unsafe { mem::transmute(value as u8) })
		} else {
			Err(IcedError::new("Invalid FormatterTextKind value"))
		}
	}
}
#[test]
#[rustfmt::skip]
fn test_formattertextkind_try_from_usize() {
	for value in FormatterTextKind::values() {
		let converted = <FormatterTextKind as TryFrom<usize>>::try_from(value as usize).unwrap();
		assert_eq!(converted, value);
	}
	assert!(<FormatterTextKind as TryFrom<usize>>::try_from(IcedConstants::FORMATTER_TEXT_KIND_ENUM_COUNT).is_err());
	assert!(<FormatterTextKind as TryFrom<usize>>::try_from(usize::MAX).is_err());
}
#[cfg(feature = "serde")]
#[rustfmt::skip]
#[allow(clippy::zero_sized_map_values)]
const _: () = {
	use core::marker::PhantomData;
	use serde::de;
	use serde::{Deserialize, Deserializer, Serialize, Serializer};
	type EnumType = FormatterTextKind;
	impl Serialize for EnumType {
		#[inline]
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_u8(*self as u8)
		}
	}
	impl<'de> Deserialize<'de> for EnumType {
		#[inline]
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct Visitor<'de> {
				marker: PhantomData<EnumType>,
				lifetime: PhantomData<&'de ()>,
			}
			impl<'de> de::Visitor<'de> for Visitor<'de> {
				type Value = EnumType;
				#[inline]
				fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
					formatter.write_str("enum FormatterTextKind")
				}
				#[inline]
				fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					if let Ok(v) = <usize as TryFrom<_>>::try_from(v) {
						if let Ok(value) = <EnumType as TryFrom<_>>::try_from(v) {
							return Ok(value);
						}
					}
					Err(de::Error::invalid_value(de::Unexpected::Unsigned(v), &"a valid FormatterTextKind variant value"))
				}
			}
			deserializer.deserialize_u8(Visitor { marker: PhantomData::<EnumType>, lifetime: PhantomData })
		}
	}
};
// GENERATOR-END: FormatterTextKind

// GENERATOR-BEGIN: PseudoOpsKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum PseudoOpsKind {
	cmpps,
	vcmpps,
	cmppd,
	vcmppd,
	cmpss,
	vcmpss,
	cmpsd,
	vcmpsd,
	pclmulqdq,
	vpclmulqdq,
	vpcomb,
	vpcomw,
	vpcomd,
	vpcomq,
	vpcomub,
	vpcomuw,
	vpcomud,
	vpcomuq,
	vpcmpb,
	vpcmpw,
	vpcmpd,
	vpcmpq,
	vpcmpub,
	vpcmpuw,
	vpcmpud,
	vpcmpuq,
	vcmpph,
	vcmpsh,
	vcmpps8,
	vcmppd8,
	vpcmpd6,
	vpcmpud6,
}
#[rustfmt::skip]
static GEN_DEBUG_PSEUDO_OPS_KIND: [&str; 32] = [
	"cmpps",
	"vcmpps",
	"cmppd",
	"vcmppd",
	"cmpss",
	"vcmpss",
	"cmpsd",
	"vcmpsd",
	"pclmulqdq",
	"vpclmulqdq",
	"vpcomb",
	"vpcomw",
	"vpcomd",
	"vpcomq",
	"vpcomub",
	"vpcomuw",
	"vpcomud",
	"vpcomuq",
	"vpcmpb",
	"vpcmpw",
	"vpcmpd",
	"vpcmpq",
	"vpcmpub",
	"vpcmpuw",
	"vpcmpud",
	"vpcmpuq",
	"vcmpph",
	"vcmpsh",
	"vcmpps8",
	"vcmppd8",
	"vpcmpd6",
	"vpcmpud6",
];
impl fmt::Debug for PseudoOpsKind {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_PSEUDO_OPS_KIND[*self as usize])
	}
}
impl Default for PseudoOpsKind {
	#[must_use]
	#[inline]
	fn default() -> Self {
		PseudoOpsKind::cmpps
	}
}
// GENERATOR-END: PseudoOpsKind

// GENERATOR-BEGIN: MemorySizeOptions
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Memory size options used by the formatters
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MemorySizeOptions {
	/// Show memory size if the assembler requires it, else don't show anything
	Default = 0,
	/// Always show the memory size, even if the assembler doesn't need it
	Always = 1,
	/// Show memory size if a human can't figure out the size of the operand
	Minimal = 2,
	/// Never show memory size
	Never = 3,
}
#[rustfmt::skip]
static GEN_DEBUG_MEMORY_SIZE_OPTIONS: [&str; 4] = [
	"Default",
	"Always",
	"Minimal",
	"Never",
];
impl fmt::Debug for MemorySizeOptions {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_MEMORY_SIZE_OPTIONS[*self as usize])
	}
}
impl Default for MemorySizeOptions {
	#[must_use]
	#[inline]
	fn default() -> Self {
		MemorySizeOptions::Default
	}
}
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) type MemorySizeOptionsUnderlyingType = u8;
#[rustfmt::skip]
impl MemorySizeOptions {
	/// Iterates over all `MemorySizeOptions` enum values
	#[inline]
	pub fn values() -> impl Iterator<Item = MemorySizeOptions> + DoubleEndedIterator + ExactSizeIterator + FusedIterator {
		// SAFETY: all values 0-max are valid enum values
		(0..IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT).map(|x| unsafe { mem::transmute::<u8, MemorySizeOptions>(x as u8) })
	}
}
#[test]
#[rustfmt::skip]
fn test_memorysizeoptions_values() {
	let mut iter = MemorySizeOptions::values();
	assert_eq!(iter.size_hint(), (IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT, Some(IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT)));
	assert_eq!(iter.len(), IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT);
	assert!(iter.next().is_some());
	assert_eq!(iter.size_hint(), (IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT - 1, Some(IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT - 1)));
	assert_eq!(iter.len(), IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT - 1);

	let values: Vec<MemorySizeOptions> = MemorySizeOptions::values().collect();
	assert_eq!(values.len(), IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT);
	for (i, value) in values.into_iter().enumerate() {
		assert_eq!(i, value as usize);
	}

	let values1: Vec<MemorySizeOptions> = MemorySizeOptions::values().collect();
	let mut values2: Vec<MemorySizeOptions> = MemorySizeOptions::values().rev().collect();
	values2.reverse();
	assert_eq!(values1, values2);
}
#[rustfmt::skip]
impl TryFrom<usize> for MemorySizeOptions {
	type Error = IcedError;
	#[inline]
	fn try_from(value: usize) -> Result<Self, Self::Error> {
		if value < IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT {
			// SAFETY: all values 0-max are valid enum values
			Ok(unsafe { mem::transmute(value as u8) })
		} else {
			Err(IcedError::new("Invalid MemorySizeOptions value"))
		}
	}
}
#[test]
#[rustfmt::skip]
fn test_memorysizeoptions_try_from_usize() {
	for value in MemorySizeOptions::values() {
		let converted = <MemorySizeOptions as TryFrom<usize>>::try_from(value as usize).unwrap();
		assert_eq!(converted, value);
	}
	assert!(<MemorySizeOptions as TryFrom<usize>>::try_from(IcedConstants::MEMORY_SIZE_OPTIONS_ENUM_COUNT).is_err());
	assert!(<MemorySizeOptions as TryFrom<usize>>::try_from(usize::MAX).is_err());
}
#[cfg(feature = "serde")]
#[rustfmt::skip]
#[allow(clippy::zero_sized_map_values)]
const _: () = {
	use core::marker::PhantomData;
	use serde::de;
	use serde::{Deserialize, Deserializer, Serialize, Serializer};
	type EnumType = MemorySizeOptions;
	impl Serialize for EnumType {
		#[inline]
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_u8(*self as u8)
		}
	}
	impl<'de> Deserialize<'de> for EnumType {
		#[inline]
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct Visitor<'de> {
				marker: PhantomData<EnumType>,
				lifetime: PhantomData<&'de ()>,
			}
			impl<'de> de::Visitor<'de> for Visitor<'de> {
				type Value = EnumType;
				#[inline]
				fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
					formatter.write_str("enum MemorySizeOptions")
				}
				#[inline]
				fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					if let Ok(v) = <usize as TryFrom<_>>::try_from(v) {
						if let Ok(value) = <EnumType as TryFrom<_>>::try_from(v) {
							return Ok(value);
						}
					}
					Err(de::Error::invalid_value(de::Unexpected::Unsigned(v), &"a valid MemorySizeOptions variant value"))
				}
			}
			deserializer.deserialize_u8(Visitor { marker: PhantomData::<EnumType>, lifetime: PhantomData })
		}
	}
};
// GENERATOR-END: MemorySizeOptions
