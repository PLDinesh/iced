// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

package com.github.icedland.iced.x86.internal.info;

import com.github.icedland.iced.x86.internal.IcedConstants;

/** DO NOT USE: INTERNAL API */
public final class CpuidFeatureInternalData {
	private CpuidFeatureInternalData() {
	}

	public static final int[][] toCpuidFeatures = getCpuidFeatures();

	@SuppressWarnings("deprecation")
	private static int[][] getCpuidFeatures() {
		byte[] data = getCpuidFeaturesData();
		com.github.icedland.iced.x86.internal.DataReader reader = new com.github.icedland.iced.x86.internal.DataReader(data);
		reader.setIndex((IcedConstants.MAX_CPUID_FEATURE_INTERNAL_VALUES + 7) / 8);
		int[][] cpuidFeatures = new int[IcedConstants.MAX_CPUID_FEATURE_INTERNAL_VALUES][];
		for (int i = 0; i < cpuidFeatures.length; i++) {
			byte b = data[i / 8];
			int[] features = new int[((b >>> (i % 8)) & 1) + 1];
			for (int j = 0; j < features.length; j++)
				features[j] = reader.ReadByte();
			cpuidFeatures[i] = features;
		}
		if (reader.canRead())
			throw new UnsupportedOperationException();
		return cpuidFeatures;
	}

	// GENERATOR-BEGIN: Table
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	private static byte[] getCpuidFeaturesData() {
		return new byte[] {
			// Header
			(byte)0x00,
			(byte)0x00,
			(byte)0x01,
			(byte)0x80,
			(byte)0xFD,
			(byte)0x3F,
			(byte)0x00,
			(byte)0x00,
			(byte)0x10,
			(byte)0xE0,
			(byte)0x00,
			(byte)0x00,
			(byte)0x10,
			(byte)0x00,
			(byte)0x00,
			(byte)0x10,
			(byte)0x80,
			(byte)0x6D,
			(byte)0x00,
			(byte)0x00,
			(byte)0x00,
			(byte)0x08,
			(byte)0x08,

			(byte)0x00,// INTEL8086
			(byte)0x01,// INTEL8086_ONLY
			(byte)0x02,// INTEL186
			(byte)0x03,// INTEL286
			(byte)0x04,// INTEL286_ONLY
			(byte)0x05,// INTEL386
			(byte)0x06,// INTEL386_ONLY
			(byte)0x07,// INTEL386_A0_ONLY
			(byte)0x08,// INTEL486
			(byte)0x09,// INTEL486_A_ONLY
			(byte)0x0A,// UMOV
			(byte)0x0B,// IA64
			(byte)0x0C,// X64
			(byte)0x0D,// ADX
			(byte)0x0E,// AES
			(byte)0x0F,// AVX
			(byte)0x0E, (byte)0x0F,// AES_AND_AVX
			(byte)0x10,// AVX2
			(byte)0x11,// AVX512_4_FMAPS
			(byte)0x12,// AVX512_4_VNNIW
			(byte)0x14,// AVX512_BITALG
			(byte)0x15,// AVX512_IFMA
			(byte)0x16,// AVX512_VBMI
			(byte)0x17,// AVX512_VBMI2
			(byte)0x18,// AVX512_VNNI
			(byte)0x1A,// AVX512_VPOPCNTDQ
			(byte)0x1B,// AVX512_BW
			(byte)0x1C,// AVX512_CD
			(byte)0x1D,// AVX512_DQ
			(byte)0x1E,// AVX512_ER
			(byte)0x1F,// AVX512_F
			(byte)0x1F, (byte)0x13,// AVX512_F_AND_AVX512_BF16
			(byte)0x1F, (byte)0x19,// AVX512_F_AND_AVX512_VP2_INTERSECT
			(byte)0x20,// AVX512_PF
			(byte)0x21, (byte)0x13,// AVX512_VL_AND_AVX512_BF16
			(byte)0x21, (byte)0x14,// AVX512_VL_AND_AVX512_BITALG
			(byte)0x21, (byte)0x15,// AVX512_VL_AND_AVX512_IFMA
			(byte)0x21, (byte)0x16,// AVX512_VL_AND_AVX512_VBMI
			(byte)0x21, (byte)0x17,// AVX512_VL_AND_AVX512_VBMI2
			(byte)0x21, (byte)0x18,// AVX512_VL_AND_AVX512_VNNI
			(byte)0x21, (byte)0x19,// AVX512_VL_AND_AVX512_VP2_INTERSECT
			(byte)0x21, (byte)0x1A,// AVX512_VL_AND_AVX512_VPOPCNTDQ
			(byte)0x21, (byte)0x1B,// AVX512_VL_AND_AVX512_BW
			(byte)0x21, (byte)0x1C,// AVX512_VL_AND_AVX512_CD
			(byte)0x21, (byte)0x1D,// AVX512_VL_AND_AVX512_DQ
			(byte)0x21, (byte)0x1F,// AVX512_VL_AND_AVX512_F
			(byte)0x22,// BMI1
			(byte)0x23,// BMI2
			(byte)0x24,// CET_IBT
			(byte)0x25,// CET_SS
			(byte)0x26,// CL1_INVMB
			(byte)0x27,// CLDEMOTE
			(byte)0x28,// CLFLUSHOPT
			(byte)0x29,// CLFSH
			(byte)0x2A,// CLWB
			(byte)0x2B,// CLZERO
			(byte)0x2C,// CMOV
			(byte)0x2D,// CMPXCHG16_B
			(byte)0x2E,// CPUID
			(byte)0x2F,// CX8
			(byte)0x30,// D3NOW
			(byte)0x31,// D3_NOWEXT
			(byte)0x32,// OSS
			(byte)0x33,// ENQCMD
			(byte)0x34,// F16_C
			(byte)0x35,// FMA
			(byte)0x36,// FMA4
			(byte)0x37,// FPU
			(byte)0x37, (byte)0x2C,// FPU_AND_CMOV
			(byte)0x38,// FPU287
			(byte)0x39,// FPU287_XL_ONLY
			(byte)0x3A,// FPU387
			(byte)0x3B,// FPU387_SL_ONLY
			(byte)0x3C,// FSGSBASE
			(byte)0x3D,// FXSR
			(byte)0x3E,// CYRIX_D3_NOW
			(byte)0x3F,// GFNI
			(byte)0x0F, (byte)0x3F,// AVX_AND_GFNI
			(byte)0x1F, (byte)0x3F,// AVX512_F_AND_GFNI
			(byte)0x21, (byte)0x3F,// AVX512_VL_AND_GFNI
			(byte)0x41,// HLE_OR_RTM
			(byte)0x43,// INVPCID
			(byte)0x45,// LWP
			(byte)0x46,// LZCNT
			(byte)0x47,// MCOMMIT
			(byte)0x48,// MMX
			(byte)0x49,// MONITOR
			(byte)0x4A,// MONITORX
			(byte)0x4B,// MOVBE
			(byte)0x4C,// MOVDIR64_B
			(byte)0x4D,// MOVDIRI
			(byte)0x4E,// MPX
			(byte)0x4F,// MSR
			(byte)0x50,// MULTIBYTENOP
			(byte)0x51,// PADLOCK_ACE
			(byte)0x52,// PADLOCK_PHE
			(byte)0x53,// PADLOCK_PMM
			(byte)0x54,// PADLOCK_RNG
			(byte)0x55,// PAUSE
			(byte)0x56,// PCLMULQDQ
			(byte)0x56, (byte)0x0F,// PCLMULQDQ_AND_AVX
			(byte)0x57,// PCOMMIT
			(byte)0x58,// PCONFIG
			(byte)0x59,// PKU
			(byte)0x5A,// POPCNT
			(byte)0x5B,// PREFETCHW
			(byte)0x5C,// PREFETCHWT1
			(byte)0x5D,// PTWRITE
			(byte)0x5E,// RDPID
			(byte)0x5F,// RDPMC
			(byte)0x60,// RDPRU
			(byte)0x61,// RDRAND
			(byte)0x62,// RDSEED
			(byte)0x63,// RDTSCP
			(byte)0x64,// RTM
			(byte)0x65,// SEP
			(byte)0x66,// SGX1
			(byte)0x67,// SHA
			(byte)0x69,// SKINIT_OR_SVM
			(byte)0x6A,// SMAP
			(byte)0x6B,// SMX
			(byte)0x6C,// SSE
			(byte)0x6D,// SSE2
			(byte)0x6E,// SSE3
			(byte)0x37, (byte)0x6E,// FPU_AND_SSE3
			(byte)0x6F,// SSE4_1
			(byte)0x70,// SSE4_2
			(byte)0x71,// SSE4_A
			(byte)0x72,// SSSE3
			(byte)0x73,// SVM
			(byte)0x74,// SEV_ES
			(byte)0x75,// SYSCALL
			(byte)0x76,// TBM
			(byte)0x77,// TSC
			(byte)0x78,// VAES
			(byte)0x1F, (byte)0x78,// AVX512_F_AND_VAES
			(byte)0x21, (byte)0x78,// AVX512_VL_AND_VAES
			(byte)0x79,// VMX
			(byte)0x79, (byte)0x42,// VMX_AND_INVEPT
			(byte)0x79, (byte)0x44,// VMX_AND_INVVPID
			(byte)0x7A,// VPCLMULQDQ
			(byte)0x1F, (byte)0x7A,// AVX512_F_AND_VPCLMULQDQ
			(byte)0x21, (byte)0x7A,// AVX512_VL_AND_VPCLMULQDQ
			(byte)0x7B,// WAITPKG
			(byte)0x7C,// WBNOINVD
			(byte)0x7D,// XOP
			(byte)0x7E,// XSAVE
			(byte)0x7F,// XSAVEC
			(byte)0x80,// XSAVEOPT
			(byte)0x81,// XSAVES
			(byte)0x82,// SEV_SNP
			(byte)0x83,// SERIALIZE
			(byte)0x84,// TSXLDTRK
			(byte)0x85,// INVLPGB
			(byte)0x86,// AMX_BF16
			(byte)0x87,// AMX_TILE
			(byte)0x88,// AMX_INT8
			(byte)0x89,// CYRIX_FPU
			(byte)0x8A,// CYRIX_SMM
			(byte)0x8B,// CYRIX_SMINT
			(byte)0x8C,// CYRIX_SMINT_0_F7_E
			(byte)0x8D,// CYRIX_SHR
			(byte)0x8E,// CYRIX_DDI
			(byte)0x8F,// CYRIX_EMMI
			(byte)0x90,// CYRIX_DMI
			(byte)0x91,// CENTAUR_AIS
			(byte)0x92,// MOV_TR
			(byte)0x93,// SMM
			(byte)0x94,// TDX
			(byte)0x95,// KL
			(byte)0x96,// AESKLE
			(byte)0x96, (byte)0x97,// AESKLE_AND_WIDE_KL
			(byte)0x98,// UINTR
			(byte)0x99,// HRESET
			(byte)0x9A,// AVX_VNNI
			(byte)0x9B,// PADLOCK_GMI
			(byte)0x9C,// FRED
			(byte)0x9D,// LKGS
			(byte)0x9E,// AVX512_FP16
			(byte)0x21, (byte)0x9E,// AVX512_VL_AND_AVX512_FP16
			(byte)0x9F,// UDBG
			(byte)0xA0,// KNC
			(byte)0xA1,// PADLOCK_UNDOC
		};
	}
	// GENERATOR-END: Table
}
