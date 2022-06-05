// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

package com.github.icedland.iced.x86.internal;

public final class TupleTypeTable {
	static final byte[] tupleTypeData = new byte[] {
		// GENERATOR-BEGIN: TupleTypeTable
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		// TupleType.N1
		(byte)0x01,// N
		(byte)0x01,// Nbcst
		// TupleType.N2
		(byte)0x02,// N
		(byte)0x02,// Nbcst
		// TupleType.N4
		(byte)0x04,// N
		(byte)0x04,// Nbcst
		// TupleType.N8
		(byte)0x08,// N
		(byte)0x08,// Nbcst
		// TupleType.N16
		(byte)0x10,// N
		(byte)0x10,// Nbcst
		// TupleType.N32
		(byte)0x20,// N
		(byte)0x20,// Nbcst
		// TupleType.N64
		(byte)0x40,// N
		(byte)0x40,// Nbcst
		// TupleType.N8B4
		(byte)0x08,// N
		(byte)0x04,// Nbcst
		// TupleType.N16B4
		(byte)0x10,// N
		(byte)0x04,// Nbcst
		// TupleType.N32B4
		(byte)0x20,// N
		(byte)0x04,// Nbcst
		// TupleType.N64B4
		(byte)0x40,// N
		(byte)0x04,// Nbcst
		// TupleType.N16B8
		(byte)0x10,// N
		(byte)0x08,// Nbcst
		// TupleType.N32B8
		(byte)0x20,// N
		(byte)0x08,// Nbcst
		// TupleType.N64B8
		(byte)0x40,// N
		(byte)0x08,// Nbcst
		// TupleType.N4B2
		(byte)0x04,// N
		(byte)0x02,// Nbcst
		// TupleType.N8B2
		(byte)0x08,// N
		(byte)0x02,// Nbcst
		// TupleType.N16B2
		(byte)0x10,// N
		(byte)0x02,// Nbcst
		// TupleType.N32B2
		(byte)0x20,// N
		(byte)0x02,// Nbcst
		// TupleType.N64B2
		(byte)0x40,// N
		(byte)0x02,// Nbcst
		// GENERATOR-END: TupleTypeTable
	};

	private TupleTypeTable() {
	}

	public static int getDisp8N(int tupleType, boolean bcst) {
		int index = (tupleType << 1) | (bcst ? 1 : 0);
		assert index >= 0 && index < tupleTypeData.length : tupleType;
		return tupleTypeData[index] & 0xFF;
	}
}
