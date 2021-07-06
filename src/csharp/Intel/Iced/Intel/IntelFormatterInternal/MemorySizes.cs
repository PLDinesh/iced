// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if INTEL
using System;
using Iced.Intel.FormatterInternal;

namespace Iced.Intel.IntelFormatterInternal {
	static class MemorySizes {
		public readonly struct Info {
			public readonly FormatterString[] keywords;
			public readonly FormatterString bcstTo;
			public Info(FormatterString[] keywords, FormatterString bcstTo) {
				this.keywords = keywords;
				this.bcstTo = bcstTo;
			}
		}
		public static readonly Info[] AllMemorySizes = GetMemorySizes();

		static Info[] GetMemorySizes() {
			// GENERATOR-BEGIN: ConstData
			// ⚠️This was generated by GENERATOR!🦹‍♂️
			const int BroadcastToKindShift = 5;
			const int MemoryKeywordsMask = 31;
			var @byte = new FormatterString("byte");
			var ptr = new FormatterString("ptr");
			var byte_ptr = new[] { @byte, ptr };
			var dword = new FormatterString("dword");
			var dword_ptr = new[] { dword, ptr };
			var fpuenv14 = new FormatterString("fpuenv14");
			var fpuenv14_ptr = new[] { fpuenv14, ptr };
			var fpuenv28 = new FormatterString("fpuenv28");
			var fpuenv28_ptr = new[] { fpuenv28, ptr };
			var fpustate108 = new FormatterString("fpustate108");
			var fpustate108_ptr = new[] { fpustate108, ptr };
			var fpustate94 = new FormatterString("fpustate94");
			var fpustate94_ptr = new[] { fpustate94, ptr };
			var fword = new FormatterString("fword");
			var fword_ptr = new[] { fword, ptr };
			var qword = new FormatterString("qword");
			var qword_ptr = new[] { qword, ptr };
			var tbyte = new FormatterString("tbyte");
			var tbyte_ptr = new[] { tbyte, ptr };
			var word = new FormatterString("word");
			var word_ptr = new[] { word, ptr };
			var xmmword = new FormatterString("xmmword");
			var xmmword_ptr = new[] { xmmword, ptr };
			var ymmword = new FormatterString("ymmword");
			var ymmword_ptr = new[] { ymmword, ptr };
			var zmmword = new FormatterString("zmmword");
			var zmmword_ptr = new[] { zmmword, ptr };
			var mem384 = new FormatterString("mem384");
			var mem384_ptr = new[] { mem384, ptr };
			var empty = new FormatterString("");
			var b1to2 = new FormatterString("1to2");
			var b1to4 = new FormatterString("1to4");
			var b1to8 = new FormatterString("1to8");
			var b1to16 = new FormatterString("1to16");
			var b1to32 = new FormatterString("1to32");
			// GENERATOR-END: ConstData

			var infos = new Info[IcedConstants.MemorySizeEnumCount];
#if HAS_SPAN
			ReadOnlySpan<byte>
#else
			byte[]
#endif
			data = new byte[IcedConstants.MemorySizeEnumCount] {
				// GENERATOR-BEGIN: MemorySizes
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				0x00,
				0x01,
				0x0A,
				0x02,
				0x08,
				0x08,
				0x0B,
				0x0C,
				0x0D,
				0x01,
				0x0A,
				0x02,
				0x08,
				0x0B,
				0x0C,
				0x0D,
				0x02,
				0x07,
				0x09,
				0x0A,
				0x02,
				0x08,
				0x02,
				0x08,
				0x08,
				0x0B,
				0x07,
				0x07,
				0x0A,
				0x02,
				0x08,
				0x09,
				0x0B,
				0x0A,
				0x03,
				0x04,
				0x06,
				0x05,
				0x00,
				0x00,
				0x00,
				0x00,
				0x09,
				0x0D,
				0x00,
				0x09,
				0x0E,
				0x0D,
				0x0A,
				0x0A,
				0x02,
				0x02,
				0x02,
				0x02,
				0x02,
				0x02,
				0x08,
				0x08,
				0x08,
				0x08,
				0x08,
				0x08,
				0x08,
				0x08,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0C,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x0D,
				0x2A,
				0x22,
				0x22,
				0x4A,
				0x22,
				0x6A,
				0x6A,
				0x42,
				0x42,
				0x28,
				0x28,
				0x28,
				0x6A,
				0x42,
				0x28,
				0x42,
				0x28,
				0x28,
				0x42,
				0x42,
				0x8A,
				0x8A,
				0x62,
				0x62,
				0x48,
				0x48,
				0x48,
				0x8A,
				0x62,
				0x48,
				0x62,
				0x48,
				0x48,
				0x62,
				0x62,
				0xAA,
				0xAA,
				0x82,
				0x82,
				0x68,
				0x68,
				0x68,
				0xAA,
				0x82,
				0x68,
				0x82,
				0x82,
				0x68,
				0x68,
				0x82,
				// GENERATOR-END: MemorySizes
			};

			for (int i = 0; i < infos.Length; i++) {
				var d = data[i];
				var keywords = (d & MemoryKeywordsMask) switch {
					// GENERATOR-BEGIN: MemoryKeywordsSwitch
					// ⚠️This was generated by GENERATOR!🦹‍♂️
					0x00 => Array2.Empty<FormatterString>(),
					0x01 => byte_ptr,
					0x02 => dword_ptr,
					0x03 => fpuenv14_ptr,
					0x04 => fpuenv28_ptr,
					0x05 => fpustate108_ptr,
					0x06 => fpustate94_ptr,
					0x07 => fword_ptr,
					0x08 => qword_ptr,
					0x09 => tbyte_ptr,
					0x0A => word_ptr,
					0x0B => xmmword_ptr,
					0x0C => ymmword_ptr,
					0x0D => zmmword_ptr,
					0x0E => mem384_ptr,
					// GENERATOR-END: MemoryKeywordsSwitch
					_ => throw new InvalidOperationException(),
				};
				var bcstTo = (d >> BroadcastToKindShift) switch {
					// GENERATOR-BEGIN: BroadcastToKindSwitch
					// ⚠️This was generated by GENERATOR!🦹‍♂️
					0x00 => empty,
					0x01 => b1to2,
					0x02 => b1to4,
					0x03 => b1to8,
					0x04 => b1to16,
					0x05 => b1to32,
					// GENERATOR-END: BroadcastToKindSwitch
					_ => throw new InvalidOperationException(),
				};
				infos[i] = new Info(keywords, bcstTo);
			}

			return infos;
		}
	}
}
#endif
