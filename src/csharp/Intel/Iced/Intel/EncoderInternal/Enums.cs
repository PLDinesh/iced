// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if ENCODER
using System;

namespace Iced.Intel.EncoderInternal {
	// GENERATOR-BEGIN: DisplSize
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum DisplSize {
		None,
		Size1,
		Size2,
		Size4,
		Size8,
		RipRelSize4_Target32,
		RipRelSize4_Target64,
	}
	// GENERATOR-END: DisplSize

	// GENERATOR-BEGIN: ImmSize
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum ImmSize {
		None,
		Size1,
		Size2,
		Size4,
		Size8,
		/// <summary><c>ENTER xxxx,yy</c></summary>
		Size2_1,
		/// <summary><c>EXTRQ/INSERTQ xx,yy</c></summary>
		Size1_1,
		/// <summary><c>CALL16 FAR x:y</c></summary>
		Size2_2,
		/// <summary><c>CALL32 FAR x:y</c></summary>
		Size4_2,
		RipRelSize1_Target16,
		RipRelSize1_Target32,
		RipRelSize1_Target64,
		RipRelSize2_Target16,
		RipRelSize2_Target32,
		RipRelSize2_Target64,
		RipRelSize4_Target32,
		RipRelSize4_Target64,
		SizeIbReg,
		Size1OpCode,
	}
	// GENERATOR-END: ImmSize

	// GENERATOR-BEGIN: EncoderFlags
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	[Flags]
	enum EncoderFlags : uint {
		None = 0x00000000,
		B = 0x00000001,
		X = 0x00000002,
		R = 0x00000004,
		W = 0x00000008,
		ModRM = 0x00000010,
		Sib = 0x00000020,
		REX = 0x00000040,
		P66 = 0x00000080,
		P67 = 0x00000100,
		/// <summary><c>EVEX.R&apos;</c></summary>
		R2 = 0x00000200,
		Broadcast = 0x00000400,
		HighLegacy8BitRegs = 0x00000800,
		Displ = 0x00001000,
		PF0 = 0x00002000,
		RegIsMemory = 0x00004000,
		MustUseSib = 0x00008000,
		VvvvvShift = 0x0000001B,
		VvvvvMask = 0x0000001F,
	}
	// GENERATOR-END: EncoderFlags

	// GENERATOR-BEGIN: LegacyOpCodeTable
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum LegacyOpCodeTable {
		MAP0,
		MAP0F,
		MAP0F38,
		MAP0F3A,
	}
	// GENERATOR-END: LegacyOpCodeTable

#if !NO_VEX
	// GENERATOR-BEGIN: VexOpCodeTable
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum VexOpCodeTable {
		MAP0F = 1,
		MAP0F38,
		MAP0F3A,
	}
	// GENERATOR-END: VexOpCodeTable
#endif

#if !NO_XOP
	// GENERATOR-BEGIN: XopOpCodeTable
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum XopOpCodeTable {
		MAP8,
		MAP9,
		MAP10,
	}
	// GENERATOR-END: XopOpCodeTable
#endif

#if !NO_EVEX
	// GENERATOR-BEGIN: EvexOpCodeTable
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum EvexOpCodeTable {
		MAP0F = 1,
		MAP0F38,
		MAP0F3A,
		MAP5 = 5,
		MAP6,
	}
	// GENERATOR-END: EvexOpCodeTable
#endif

	// GENERATOR-BEGIN: EncFlags1
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	[Flags]
	enum EncFlags1 : uint {
		None = 0x00000000,
		Legacy_OpMask = 0x0000007F,
		Legacy_Op0Shift = 0x00000000,
		Legacy_Op1Shift = 0x00000007,
		Legacy_Op2Shift = 0x0000000E,
		Legacy_Op3Shift = 0x00000015,
		VEX_OpMask = 0x0000003F,
		VEX_Op0Shift = 0x00000000,
		VEX_Op1Shift = 0x00000006,
		VEX_Op2Shift = 0x0000000C,
		VEX_Op3Shift = 0x00000012,
		VEX_Op4Shift = 0x00000018,
		XOP_OpMask = 0x0000001F,
		XOP_Op0Shift = 0x00000000,
		XOP_Op1Shift = 0x00000005,
		XOP_Op2Shift = 0x0000000A,
		XOP_Op3Shift = 0x0000000F,
		EVEX_OpMask = 0x0000001F,
		EVEX_Op0Shift = 0x00000000,
		EVEX_Op1Shift = 0x00000005,
		EVEX_Op2Shift = 0x0000000A,
		EVEX_Op3Shift = 0x0000000F,
		IgnoresRoundingControl = 0x40000000,
		AmdLockRegBit = 0x80000000,
	}
	// GENERATOR-END: EncFlags1

	// GENERATOR-BEGIN: EncFlags2
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	[Flags]
	enum EncFlags2 : uint {
		None = 0x00000000,
		OpCodeShift = 0x00000000,
		OpCodeIs2Bytes = 0x00010000,
		TableShift = 0x00000011,
		TableMask = 0x00000007,
		MandatoryPrefixShift = 0x00000014,
		MandatoryPrefixMask = 0x00000003,
		WBitShift = 0x00000016,
		WBitMask = 0x00000003,
		LBitShift = 0x00000018,
		LBitMask = 0x00000007,
		GroupIndexShift = 0x0000001B,
		GroupIndexMask = 0x00000007,
		HasMandatoryPrefix = 0x40000000,
		HasGroupIndex = 0x80000000,
	}
	// GENERATOR-END: EncFlags2

	// GENERATOR-BEGIN: EncFlags3
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	[Flags]
	enum EncFlags3 : uint {
		None = 0x00000000,
		EncodingShift = 0x00000000,
		EncodingMask = 0x00000007,
		OperandSizeShift = 0x00000003,
		OperandSizeMask = 0x00000003,
		AddressSizeShift = 0x00000005,
		AddressSizeMask = 0x00000003,
		TupleTypeShift = 0x00000007,
		TupleTypeMask = 0x0000001F,
		DefaultOpSize64 = 0x00001000,
		HasRmGroupIndex = 0x00002000,
		IntelForceOpSize64 = 0x00004000,
		Fwait = 0x00008000,
		Bit16or32 = 0x00010000,
		Bit64 = 0x00020000,
		Lock = 0x00040000,
		Xacquire = 0x00080000,
		Xrelease = 0x00100000,
		Rep = 0x00200000,
		Repne = 0x00400000,
		Bnd = 0x00800000,
		HintTaken = 0x01000000,
		Notrack = 0x02000000,
		Broadcast = 0x04000000,
		RoundingControl = 0x08000000,
		SuppressAllExceptions = 0x10000000,
		OpMaskRegister = 0x20000000,
		ZeroingMasking = 0x40000000,
		RequireOpMaskRegister = 0x80000000,
	}
	// GENERATOR-END: EncFlags3

#if !NO_VEX || !NO_EVEX || !NO_XOP
	// GENERATOR-BEGIN: WBit
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum WBit : uint {
		W0,
		W1,
		WIG,
		WIG32,
	}
	// GENERATOR-END: WBit
#endif

#if !NO_VEX || !NO_EVEX || !NO_XOP
	// GENERATOR-BEGIN: LBit
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum LBit : uint {
		L0,
		L1,
		LIG,
		LZ,
		L128,
		L256,
		L512,
	}
	// GENERATOR-END: LBit
#endif
}
#endif
