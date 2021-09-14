// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: Enum
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Size of a memory reference
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum MemorySize {
	/// Unknown size or the instruction doesn't reference any memory (eg. `LEA`)
	Unknown = 0,
	/// Memory location contains a `u8`
	UInt8 = 1,
	/// Memory location contains a `u16`
	UInt16 = 2,
	/// Memory location contains a `u32`
	UInt32 = 3,
	/// Memory location contains a `u52`
	UInt52 = 4,
	/// Memory location contains a `u64`
	UInt64 = 5,
	/// Memory location contains a `u128`
	UInt128 = 6,
	/// Memory location contains a `u256`
	UInt256 = 7,
	/// Memory location contains a `u512`
	UInt512 = 8,
	/// Memory location contains a `i8`
	Int8 = 9,
	/// Memory location contains a `i16`
	Int16 = 10,
	/// Memory location contains a `i32`
	Int32 = 11,
	/// Memory location contains a `i64`
	Int64 = 12,
	/// Memory location contains a `i128`
	Int128 = 13,
	/// Memory location contains a `i256`
	Int256 = 14,
	/// Memory location contains a `i512`
	Int512 = 15,
	/// Memory location contains a seg:ptr pair, `u16` (offset) + `u16` (segment/selector)
	SegPtr16 = 16,
	/// Memory location contains a seg:ptr pair, `u32` (offset) + `u16` (segment/selector)
	SegPtr32 = 17,
	/// Memory location contains a seg:ptr pair, `u64` (offset) + `u16` (segment/selector)
	SegPtr64 = 18,
	/// Memory location contains a 16-bit offset (`JMP/CALL WORD PTR [mem]`)
	WordOffset = 19,
	/// Memory location contains a 32-bit offset (`JMP/CALL DWORD PTR [mem]`)
	DwordOffset = 20,
	/// Memory location contains a 64-bit offset (`JMP/CALL QWORD PTR [mem]`)
	QwordOffset = 21,
	/// Memory location contains two `u16`s (16-bit `BOUND`)
	Bound16_WordWord = 22,
	/// Memory location contains two `u32`s (32-bit `BOUND`)
	Bound32_DwordDword = 23,
	/// 32-bit `BNDMOV`, 2 x `u32`
	Bnd32 = 24,
	/// 64-bit `BNDMOV`, 2 x `u64`
	Bnd64 = 25,
	/// Memory location contains a 16-bit limit and a 32-bit address (eg. `LGDTW`, `LGDTD`)
	Fword6 = 26,
	/// Memory location contains a 16-bit limit and a 64-bit address (eg. `LGDTQ`)
	Fword10 = 27,
	/// Memory location contains a `f16`
	Float16 = 28,
	/// Memory location contains a `f32`
	Float32 = 29,
	/// Memory location contains a `f64`
	Float64 = 30,
	/// Memory location contains a `f80`
	Float80 = 31,
	/// Memory location contains a `f128`
	Float128 = 32,
	/// Memory location contains a `bfloat16`
	BFloat16 = 33,
	/// Memory location contains a 14-byte FPU environment (16-bit `FLDENV`/`FSTENV`)
	FpuEnv14 = 34,
	/// Memory location contains a 28-byte FPU environment (32/64-bit `FLDENV`/`FSTENV`)
	FpuEnv28 = 35,
	/// Memory location contains a 94-byte FPU environment (16-bit `FSAVE`/`FRSTOR`)
	FpuState94 = 36,
	/// Memory location contains a 108-byte FPU environment (32/64-bit `FSAVE`/`FRSTOR`)
	FpuState108 = 37,
	/// Memory location contains 512-bytes of `FXSAVE`/`FXRSTOR` data
	Fxsave_512Byte = 38,
	/// Memory location contains 512-bytes of `FXSAVE64`/`FXRSTOR64` data
	Fxsave64_512Byte = 39,
	/// 32-bit `XSAVE` area
	Xsave = 40,
	/// 64-bit `XSAVE` area
	Xsave64 = 41,
	/// Memory location contains a 10-byte `bcd` value (`FBLD`/`FBSTP`)
	Bcd = 42,
	/// 64-bit location: TILECFG (`LDTILECFG`/`STTILECFG`)
	Tilecfg = 43,
	/// Tile data
	Tile = 44,
	/// 80-bit segment descriptor and selector: 0-7 = descriptor, 8-9 = selector
	SegmentDescSelector = 45,
	/// 384-bit AES 128 handle (Key Locker)
	KLHandleAes128 = 46,
	/// 512-bit AES 256 handle (Key Locker)
	KLHandleAes256 = 47,
	/// 16-bit location: 2 x `u8`
	Packed16_UInt8 = 48,
	/// 16-bit location: 2 x `i8`
	Packed16_Int8 = 49,
	/// 32-bit location: 4 x `u8`
	Packed32_UInt8 = 50,
	/// 32-bit location: 4 x `i8`
	Packed32_Int8 = 51,
	/// 32-bit location: 2 x `u16`
	Packed32_UInt16 = 52,
	/// 32-bit location: 2 x `i16`
	Packed32_Int16 = 53,
	/// 32-bit location: 2 x `f16`
	Packed32_Float16 = 54,
	/// 32-bit location: 2 x `bfloat16`
	Packed32_BFloat16 = 55,
	/// 64-bit location: 8 x `u8`
	Packed64_UInt8 = 56,
	/// 64-bit location: 8 x `i8`
	Packed64_Int8 = 57,
	/// 64-bit location: 4 x `u16`
	Packed64_UInt16 = 58,
	/// 64-bit location: 4 x `i16`
	Packed64_Int16 = 59,
	/// 64-bit location: 2 x `u32`
	Packed64_UInt32 = 60,
	/// 64-bit location: 2 x `i32`
	Packed64_Int32 = 61,
	/// 64-bit location: 4 x `f16`
	Packed64_Float16 = 62,
	/// 64-bit location: 2 x `f32`
	Packed64_Float32 = 63,
	/// 128-bit location: 16 x `u8`
	Packed128_UInt8 = 64,
	/// 128-bit location: 16 x `i8`
	Packed128_Int8 = 65,
	/// 128-bit location: 8 x `u16`
	Packed128_UInt16 = 66,
	/// 128-bit location: 8 x `i16`
	Packed128_Int16 = 67,
	/// 128-bit location: 4 x `u32`
	Packed128_UInt32 = 68,
	/// 128-bit location: 4 x `i32`
	Packed128_Int32 = 69,
	/// 128-bit location: 2 x `u52`
	Packed128_UInt52 = 70,
	/// 128-bit location: 2 x `u64`
	Packed128_UInt64 = 71,
	/// 128-bit location: 2 x `i64`
	Packed128_Int64 = 72,
	/// 128-bit location: 8 x `f16`
	Packed128_Float16 = 73,
	/// 128-bit location: 4 x `f32`
	Packed128_Float32 = 74,
	/// 128-bit location: 2 x `f64`
	Packed128_Float64 = 75,
	/// 128-bit location: 4 x (2 x `f16`)
	Packed128_2xFloat16 = 76,
	/// 128-bit location: 4 x (2 x `bfloat16`)
	Packed128_2xBFloat16 = 77,
	/// 256-bit location: 32 x `u8`
	Packed256_UInt8 = 78,
	/// 256-bit location: 32 x `i8`
	Packed256_Int8 = 79,
	/// 256-bit location: 16 x `u16`
	Packed256_UInt16 = 80,
	/// 256-bit location: 16 x `i16`
	Packed256_Int16 = 81,
	/// 256-bit location: 8 x `u32`
	Packed256_UInt32 = 82,
	/// 256-bit location: 8 x `i32`
	Packed256_Int32 = 83,
	/// 256-bit location: 4 x `u52`
	Packed256_UInt52 = 84,
	/// 256-bit location: 4 x `u64`
	Packed256_UInt64 = 85,
	/// 256-bit location: 4 x `i64`
	Packed256_Int64 = 86,
	/// 256-bit location: 2 x `u128`
	Packed256_UInt128 = 87,
	/// 256-bit location: 2 x `i128`
	Packed256_Int128 = 88,
	/// 256-bit location: 16 x `f16`
	Packed256_Float16 = 89,
	/// 256-bit location: 8 x `f32`
	Packed256_Float32 = 90,
	/// 256-bit location: 4 x `f64`
	Packed256_Float64 = 91,
	/// 256-bit location: 2 x `f128`
	Packed256_Float128 = 92,
	/// 256-bit location: 8 x (2 x `f16`)
	Packed256_2xFloat16 = 93,
	/// 256-bit location: 8 x (2 x `bfloat16`)
	Packed256_2xBFloat16 = 94,
	/// 512-bit location: 64 x `u8`
	Packed512_UInt8 = 95,
	/// 512-bit location: 64 x `i8`
	Packed512_Int8 = 96,
	/// 512-bit location: 32 x `u16`
	Packed512_UInt16 = 97,
	/// 512-bit location: 32 x `i16`
	Packed512_Int16 = 98,
	/// 512-bit location: 16 x `u32`
	Packed512_UInt32 = 99,
	/// 512-bit location: 16 x `i32`
	Packed512_Int32 = 100,
	/// 512-bit location: 8 x `u52`
	Packed512_UInt52 = 101,
	/// 512-bit location: 8 x `u64`
	Packed512_UInt64 = 102,
	/// 512-bit location: 8 x `i64`
	Packed512_Int64 = 103,
	/// 256-bit location: 4 x `u128`
	Packed512_UInt128 = 104,
	/// 512-bit location: 32 x `f16`
	Packed512_Float16 = 105,
	/// 512-bit location: 16 x `f32`
	Packed512_Float32 = 106,
	/// 512-bit location: 8 x `f64`
	Packed512_Float64 = 107,
	/// 512-bit location: 16 x (2 x `f16`)
	Packed512_2xFloat16 = 108,
	/// 512-bit location: 16 x (2 x `bfloat16`)
	Packed512_2xBFloat16 = 109,
	/// Broadcast `f16` to 32-bits
	Broadcast32_Float16 = 110,
	/// Broadcast `u32` to 64-bits
	Broadcast64_UInt32 = 111,
	/// Broadcast `i32` to 64-bits
	Broadcast64_Int32 = 112,
	/// Broadcast `f16` to 64-bits
	Broadcast64_Float16 = 113,
	/// Broadcast `f32` to 64-bits
	Broadcast64_Float32 = 114,
	/// Broadcast `i16` to 128-bits
	Broadcast128_Int16 = 115,
	/// Broadcast `u16` to 128-bits
	Broadcast128_UInt16 = 116,
	/// Broadcast `u32` to 128-bits
	Broadcast128_UInt32 = 117,
	/// Broadcast `i32` to 128-bits
	Broadcast128_Int32 = 118,
	/// Broadcast `u52` to 128-bits
	Broadcast128_UInt52 = 119,
	/// Broadcast `u64` to 128-bits
	Broadcast128_UInt64 = 120,
	/// Broadcast `i64` to 128-bits
	Broadcast128_Int64 = 121,
	/// Broadcast `f16` to 128-bits
	Broadcast128_Float16 = 122,
	/// Broadcast `f32` to 128-bits
	Broadcast128_Float32 = 123,
	/// Broadcast `f64` to 128-bits
	Broadcast128_Float64 = 124,
	/// Broadcast 2 x `i16` to 128-bits
	Broadcast128_2xInt16 = 125,
	/// Broadcast 2 x `i32` to 128-bits
	Broadcast128_2xInt32 = 126,
	/// Broadcast 2 x `u32` to 128-bits
	Broadcast128_2xUInt32 = 127,
	/// Broadcast 2 x `f16` to 128-bits
	Broadcast128_2xFloat16 = 128,
	/// Broadcast 2 x `bfloat16` to 128-bits
	Broadcast128_2xBFloat16 = 129,
	/// Broadcast `i16` to 256-bits
	Broadcast256_Int16 = 130,
	/// Broadcast `u16` to 256-bits
	Broadcast256_UInt16 = 131,
	/// Broadcast `u32` to 256-bits
	Broadcast256_UInt32 = 132,
	/// Broadcast `i32` to 256-bits
	Broadcast256_Int32 = 133,
	/// Broadcast `u52` to 256-bits
	Broadcast256_UInt52 = 134,
	/// Broadcast `u64` to 256-bits
	Broadcast256_UInt64 = 135,
	/// Broadcast `i64` to 256-bits
	Broadcast256_Int64 = 136,
	/// Broadcast `f16` to 256-bits
	Broadcast256_Float16 = 137,
	/// Broadcast `f32` to 256-bits
	Broadcast256_Float32 = 138,
	/// Broadcast `f64` to 256-bits
	Broadcast256_Float64 = 139,
	/// Broadcast 2 x `i16` to 256-bits
	Broadcast256_2xInt16 = 140,
	/// Broadcast 2 x `i32` to 256-bits
	Broadcast256_2xInt32 = 141,
	/// Broadcast 2 x `u32` to 256-bits
	Broadcast256_2xUInt32 = 142,
	/// Broadcast 2 x `f16` to 256-bits
	Broadcast256_2xFloat16 = 143,
	/// Broadcast 2 x `bfloat16` to 256-bits
	Broadcast256_2xBFloat16 = 144,
	/// Broadcast `i16` to 512-bits
	Broadcast512_Int16 = 145,
	/// Broadcast `u16` to 512-bits
	Broadcast512_UInt16 = 146,
	/// Broadcast `u32` to 512-bits
	Broadcast512_UInt32 = 147,
	/// Broadcast `i32` to 512-bits
	Broadcast512_Int32 = 148,
	/// Broadcast `u52` to 512-bits
	Broadcast512_UInt52 = 149,
	/// Broadcast `u64` to 512-bits
	Broadcast512_UInt64 = 150,
	/// Broadcast `i64` to 512-bits
	Broadcast512_Int64 = 151,
	/// Broadcast `f16` to 512-bits
	Broadcast512_Float16 = 152,
	/// Broadcast `f32` to 512-bits
	Broadcast512_Float32 = 153,
	/// Broadcast `f64` to 512-bits
	Broadcast512_Float64 = 154,
	/// Broadcast 2 x `f16` to 512-bits
	Broadcast512_2xFloat16 = 155,
	/// Broadcast 2 x `i16` to 512-bits
	Broadcast512_2xInt16 = 156,
	/// Broadcast 2 x `u32` to 512-bits
	Broadcast512_2xUInt32 = 157,
	/// Broadcast 2 x `i32` to 512-bits
	Broadcast512_2xInt32 = 158,
	/// Broadcast 2 x `bfloat16` to 512-bits
	Broadcast512_2xBFloat16 = 159,
}
// GENERATOR-END: Enum

#[allow(dead_code)]
pub(crate) fn iced_to_memory_size(value: iced_x86_rust::MemorySize) -> MemorySize {
	// SAFETY: the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn memory_size_to_iced(value: MemorySize) -> iced_x86_rust::MemorySize {
	// SAFETY: the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}
