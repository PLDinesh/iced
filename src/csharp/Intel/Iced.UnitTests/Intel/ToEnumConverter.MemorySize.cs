// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

using System;
using System.Collections.Generic;
using Iced.Intel;

namespace Iced.UnitTests.Intel {
	static partial class ToEnumConverter {
		public static bool TryMemorySize(string value, out MemorySize memorySize) => memorySizeDict.TryGetValue(value, out memorySize);
		public static MemorySize GetMemorySize(string value) => TryMemorySize(value, out var memorySize) ? memorySize : throw new InvalidOperationException($"Invalid MemorySize value: {value}");

		static readonly Dictionary<string, MemorySize> memorySizeDict =
			// GENERATOR-BEGIN: MemorySizeHash
			// ⚠️This was generated by GENERATOR!🦹‍♂️
			new Dictionary<string, MemorySize>(160, StringComparer.Ordinal) {
				{ "Unknown", MemorySize.Unknown },
				{ "UInt8", MemorySize.UInt8 },
				{ "UInt16", MemorySize.UInt16 },
				{ "UInt32", MemorySize.UInt32 },
				{ "UInt52", MemorySize.UInt52 },
				{ "UInt64", MemorySize.UInt64 },
				{ "UInt128", MemorySize.UInt128 },
				{ "UInt256", MemorySize.UInt256 },
				{ "UInt512", MemorySize.UInt512 },
				{ "Int8", MemorySize.Int8 },
				{ "Int16", MemorySize.Int16 },
				{ "Int32", MemorySize.Int32 },
				{ "Int64", MemorySize.Int64 },
				{ "Int128", MemorySize.Int128 },
				{ "Int256", MemorySize.Int256 },
				{ "Int512", MemorySize.Int512 },
				{ "SegPtr16", MemorySize.SegPtr16 },
				{ "SegPtr32", MemorySize.SegPtr32 },
				{ "SegPtr64", MemorySize.SegPtr64 },
				{ "WordOffset", MemorySize.WordOffset },
				{ "DwordOffset", MemorySize.DwordOffset },
				{ "QwordOffset", MemorySize.QwordOffset },
				{ "Bound16_WordWord", MemorySize.Bound16_WordWord },
				{ "Bound32_DwordDword", MemorySize.Bound32_DwordDword },
				{ "Bnd32", MemorySize.Bnd32 },
				{ "Bnd64", MemorySize.Bnd64 },
				{ "Fword6", MemorySize.Fword6 },
				{ "Fword10", MemorySize.Fword10 },
				{ "Float16", MemorySize.Float16 },
				{ "Float32", MemorySize.Float32 },
				{ "Float64", MemorySize.Float64 },
				{ "Float80", MemorySize.Float80 },
				{ "Float128", MemorySize.Float128 },
				{ "BFloat16", MemorySize.BFloat16 },
				{ "FpuEnv14", MemorySize.FpuEnv14 },
				{ "FpuEnv28", MemorySize.FpuEnv28 },
				{ "FpuState94", MemorySize.FpuState94 },
				{ "FpuState108", MemorySize.FpuState108 },
				{ "Fxsave_512Byte", MemorySize.Fxsave_512Byte },
				{ "Fxsave64_512Byte", MemorySize.Fxsave64_512Byte },
				{ "Xsave", MemorySize.Xsave },
				{ "Xsave64", MemorySize.Xsave64 },
				{ "Bcd", MemorySize.Bcd },
				{ "Tilecfg", MemorySize.Tilecfg },
				{ "Tile", MemorySize.Tile },
				{ "SegmentDescSelector", MemorySize.SegmentDescSelector },
				{ "KLHandleAes128", MemorySize.KLHandleAes128 },
				{ "KLHandleAes256", MemorySize.KLHandleAes256 },
				{ "Packed16_UInt8", MemorySize.Packed16_UInt8 },
				{ "Packed16_Int8", MemorySize.Packed16_Int8 },
				{ "Packed32_UInt8", MemorySize.Packed32_UInt8 },
				{ "Packed32_Int8", MemorySize.Packed32_Int8 },
				{ "Packed32_UInt16", MemorySize.Packed32_UInt16 },
				{ "Packed32_Int16", MemorySize.Packed32_Int16 },
				{ "Packed32_Float16", MemorySize.Packed32_Float16 },
				{ "Packed32_BFloat16", MemorySize.Packed32_BFloat16 },
				{ "Packed64_UInt8", MemorySize.Packed64_UInt8 },
				{ "Packed64_Int8", MemorySize.Packed64_Int8 },
				{ "Packed64_UInt16", MemorySize.Packed64_UInt16 },
				{ "Packed64_Int16", MemorySize.Packed64_Int16 },
				{ "Packed64_UInt32", MemorySize.Packed64_UInt32 },
				{ "Packed64_Int32", MemorySize.Packed64_Int32 },
				{ "Packed64_Float16", MemorySize.Packed64_Float16 },
				{ "Packed64_Float32", MemorySize.Packed64_Float32 },
				{ "Packed128_UInt8", MemorySize.Packed128_UInt8 },
				{ "Packed128_Int8", MemorySize.Packed128_Int8 },
				{ "Packed128_UInt16", MemorySize.Packed128_UInt16 },
				{ "Packed128_Int16", MemorySize.Packed128_Int16 },
				{ "Packed128_UInt32", MemorySize.Packed128_UInt32 },
				{ "Packed128_Int32", MemorySize.Packed128_Int32 },
				{ "Packed128_UInt52", MemorySize.Packed128_UInt52 },
				{ "Packed128_UInt64", MemorySize.Packed128_UInt64 },
				{ "Packed128_Int64", MemorySize.Packed128_Int64 },
				{ "Packed128_Float16", MemorySize.Packed128_Float16 },
				{ "Packed128_Float32", MemorySize.Packed128_Float32 },
				{ "Packed128_Float64", MemorySize.Packed128_Float64 },
				{ "Packed128_2xFloat16", MemorySize.Packed128_2xFloat16 },
				{ "Packed128_2xBFloat16", MemorySize.Packed128_2xBFloat16 },
				{ "Packed256_UInt8", MemorySize.Packed256_UInt8 },
				{ "Packed256_Int8", MemorySize.Packed256_Int8 },
				{ "Packed256_UInt16", MemorySize.Packed256_UInt16 },
				{ "Packed256_Int16", MemorySize.Packed256_Int16 },
				{ "Packed256_UInt32", MemorySize.Packed256_UInt32 },
				{ "Packed256_Int32", MemorySize.Packed256_Int32 },
				{ "Packed256_UInt52", MemorySize.Packed256_UInt52 },
				{ "Packed256_UInt64", MemorySize.Packed256_UInt64 },
				{ "Packed256_Int64", MemorySize.Packed256_Int64 },
				{ "Packed256_UInt128", MemorySize.Packed256_UInt128 },
				{ "Packed256_Int128", MemorySize.Packed256_Int128 },
				{ "Packed256_Float16", MemorySize.Packed256_Float16 },
				{ "Packed256_Float32", MemorySize.Packed256_Float32 },
				{ "Packed256_Float64", MemorySize.Packed256_Float64 },
				{ "Packed256_Float128", MemorySize.Packed256_Float128 },
				{ "Packed256_2xFloat16", MemorySize.Packed256_2xFloat16 },
				{ "Packed256_2xBFloat16", MemorySize.Packed256_2xBFloat16 },
				{ "Packed512_UInt8", MemorySize.Packed512_UInt8 },
				{ "Packed512_Int8", MemorySize.Packed512_Int8 },
				{ "Packed512_UInt16", MemorySize.Packed512_UInt16 },
				{ "Packed512_Int16", MemorySize.Packed512_Int16 },
				{ "Packed512_UInt32", MemorySize.Packed512_UInt32 },
				{ "Packed512_Int32", MemorySize.Packed512_Int32 },
				{ "Packed512_UInt52", MemorySize.Packed512_UInt52 },
				{ "Packed512_UInt64", MemorySize.Packed512_UInt64 },
				{ "Packed512_Int64", MemorySize.Packed512_Int64 },
				{ "Packed512_UInt128", MemorySize.Packed512_UInt128 },
				{ "Packed512_Float16", MemorySize.Packed512_Float16 },
				{ "Packed512_Float32", MemorySize.Packed512_Float32 },
				{ "Packed512_Float64", MemorySize.Packed512_Float64 },
				{ "Packed512_2xFloat16", MemorySize.Packed512_2xFloat16 },
				{ "Packed512_2xBFloat16", MemorySize.Packed512_2xBFloat16 },
				{ "Broadcast32_Float16", MemorySize.Broadcast32_Float16 },
				{ "Broadcast64_UInt32", MemorySize.Broadcast64_UInt32 },
				{ "Broadcast64_Int32", MemorySize.Broadcast64_Int32 },
				{ "Broadcast64_Float16", MemorySize.Broadcast64_Float16 },
				{ "Broadcast64_Float32", MemorySize.Broadcast64_Float32 },
				{ "Broadcast128_Int16", MemorySize.Broadcast128_Int16 },
				{ "Broadcast128_UInt16", MemorySize.Broadcast128_UInt16 },
				{ "Broadcast128_UInt32", MemorySize.Broadcast128_UInt32 },
				{ "Broadcast128_Int32", MemorySize.Broadcast128_Int32 },
				{ "Broadcast128_UInt52", MemorySize.Broadcast128_UInt52 },
				{ "Broadcast128_UInt64", MemorySize.Broadcast128_UInt64 },
				{ "Broadcast128_Int64", MemorySize.Broadcast128_Int64 },
				{ "Broadcast128_Float16", MemorySize.Broadcast128_Float16 },
				{ "Broadcast128_Float32", MemorySize.Broadcast128_Float32 },
				{ "Broadcast128_Float64", MemorySize.Broadcast128_Float64 },
				{ "Broadcast128_2xInt16", MemorySize.Broadcast128_2xInt16 },
				{ "Broadcast128_2xInt32", MemorySize.Broadcast128_2xInt32 },
				{ "Broadcast128_2xUInt32", MemorySize.Broadcast128_2xUInt32 },
				{ "Broadcast128_2xFloat16", MemorySize.Broadcast128_2xFloat16 },
				{ "Broadcast128_2xBFloat16", MemorySize.Broadcast128_2xBFloat16 },
				{ "Broadcast256_Int16", MemorySize.Broadcast256_Int16 },
				{ "Broadcast256_UInt16", MemorySize.Broadcast256_UInt16 },
				{ "Broadcast256_UInt32", MemorySize.Broadcast256_UInt32 },
				{ "Broadcast256_Int32", MemorySize.Broadcast256_Int32 },
				{ "Broadcast256_UInt52", MemorySize.Broadcast256_UInt52 },
				{ "Broadcast256_UInt64", MemorySize.Broadcast256_UInt64 },
				{ "Broadcast256_Int64", MemorySize.Broadcast256_Int64 },
				{ "Broadcast256_Float16", MemorySize.Broadcast256_Float16 },
				{ "Broadcast256_Float32", MemorySize.Broadcast256_Float32 },
				{ "Broadcast256_Float64", MemorySize.Broadcast256_Float64 },
				{ "Broadcast256_2xInt16", MemorySize.Broadcast256_2xInt16 },
				{ "Broadcast256_2xInt32", MemorySize.Broadcast256_2xInt32 },
				{ "Broadcast256_2xUInt32", MemorySize.Broadcast256_2xUInt32 },
				{ "Broadcast256_2xFloat16", MemorySize.Broadcast256_2xFloat16 },
				{ "Broadcast256_2xBFloat16", MemorySize.Broadcast256_2xBFloat16 },
				{ "Broadcast512_Int16", MemorySize.Broadcast512_Int16 },
				{ "Broadcast512_UInt16", MemorySize.Broadcast512_UInt16 },
				{ "Broadcast512_UInt32", MemorySize.Broadcast512_UInt32 },
				{ "Broadcast512_Int32", MemorySize.Broadcast512_Int32 },
				{ "Broadcast512_UInt52", MemorySize.Broadcast512_UInt52 },
				{ "Broadcast512_UInt64", MemorySize.Broadcast512_UInt64 },
				{ "Broadcast512_Int64", MemorySize.Broadcast512_Int64 },
				{ "Broadcast512_Float16", MemorySize.Broadcast512_Float16 },
				{ "Broadcast512_Float32", MemorySize.Broadcast512_Float32 },
				{ "Broadcast512_Float64", MemorySize.Broadcast512_Float64 },
				{ "Broadcast512_2xFloat16", MemorySize.Broadcast512_2xFloat16 },
				{ "Broadcast512_2xInt16", MemorySize.Broadcast512_2xInt16 },
				{ "Broadcast512_2xUInt32", MemorySize.Broadcast512_2xUInt32 },
				{ "Broadcast512_2xInt32", MemorySize.Broadcast512_2xInt32 },
				{ "Broadcast512_2xBFloat16", MemorySize.Broadcast512_2xBFloat16 },
			};
			// GENERATOR-END: MemorySizeHash
	}
}
