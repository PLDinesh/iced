// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

namespace Iced.Intel {
	/// <summary>Instruction operand kind</summary>
	public enum OpKind {
		/// <summary>A register (<see cref="Iced.Intel.Register"/>).<br/>
		/// <br/>
		/// This operand kind uses <see cref="Instruction.Op0Register"/>, <see cref="Instruction.Op1Register"/>, <see cref="Instruction.Op2Register"/>, <see cref="Instruction.Op3Register"/> or <see cref="Instruction.Op4Register"/> depending on operand number. See also <see cref="Instruction.GetOpRegister"/>.</summary>
		Register = 0,
		/// <summary>Near 16-bit branch. This operand kind uses <see cref="Instruction.NearBranch16"/></summary>
		NearBranch16 = 1,
		/// <summary>Near 32-bit branch. This operand kind uses <see cref="Instruction.NearBranch32"/></summary>
		NearBranch32 = 2,
		/// <summary>Near 64-bit branch. This operand kind uses <see cref="Instruction.NearBranch64"/></summary>
		NearBranch64 = 3,
		/// <summary>Far 16-bit branch. This operand kind uses <see cref="Instruction.FarBranch16"/> and <see cref="Instruction.FarBranchSelector"/></summary>
		FarBranch16 = 4,
		/// <summary>Far 32-bit branch. This operand kind uses <see cref="Instruction.FarBranch32"/> and <see cref="Instruction.FarBranchSelector"/></summary>
		FarBranch32 = 5,
		/// <summary>8-bit constant. This operand kind uses <see cref="Instruction.Immediate8"/></summary>
		Immediate8 = 6,
		/// <summary>8-bit constant used by the <c>ENTER</c>, <c>EXTRQ</c>, <c>INSERTQ</c> instructions. This operand kind uses <see cref="Instruction.Immediate8_2nd"/></summary>
		Immediate8_2nd = 7,
		/// <summary>16-bit constant. This operand kind uses <see cref="Instruction.Immediate16"/></summary>
		Immediate16 = 8,
		/// <summary>32-bit constant. This operand kind uses <see cref="Instruction.Immediate32"/></summary>
		Immediate32 = 9,
		/// <summary>64-bit constant. This operand kind uses <see cref="Instruction.Immediate64"/></summary>
		Immediate64 = 10,
		/// <summary>An 8-bit value sign extended to 16 bits. This operand kind uses <see cref="Instruction.Immediate8to16"/></summary>
		Immediate8to16 = 11,
		/// <summary>An 8-bit value sign extended to 32 bits. This operand kind uses <see cref="Instruction.Immediate8to32"/></summary>
		Immediate8to32 = 12,
		/// <summary>An 8-bit value sign extended to 64 bits. This operand kind uses <see cref="Instruction.Immediate8to64"/></summary>
		Immediate8to64 = 13,
		/// <summary>A 32-bit value sign extended to 64 bits. This operand kind uses <see cref="Instruction.Immediate32to64"/></summary>
		Immediate32to64 = 14,
		/// <summary><c>seg:[SI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/>, <see cref="Instruction.MemorySegment"/>, <see cref="Instruction.SegmentPrefix"/></summary>
		MemorySegSI = 15,
		/// <summary><c>seg:[ESI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/>, <see cref="Instruction.MemorySegment"/>, <see cref="Instruction.SegmentPrefix"/></summary>
		MemorySegESI = 16,
		/// <summary><c>seg:[RSI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/>, <see cref="Instruction.MemorySegment"/>, <see cref="Instruction.SegmentPrefix"/></summary>
		MemorySegRSI = 17,
		/// <summary><c>seg:[DI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/>, <see cref="Instruction.MemorySegment"/>, <see cref="Instruction.SegmentPrefix"/></summary>
		MemorySegDI = 18,
		/// <summary><c>seg:[EDI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/>, <see cref="Instruction.MemorySegment"/>, <see cref="Instruction.SegmentPrefix"/></summary>
		MemorySegEDI = 19,
		/// <summary><c>seg:[RDI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/>, <see cref="Instruction.MemorySegment"/>, <see cref="Instruction.SegmentPrefix"/></summary>
		MemorySegRDI = 20,
		/// <summary><c>ES:[DI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/></summary>
		MemoryESDI = 21,
		/// <summary><c>ES:[EDI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/></summary>
		MemoryESEDI = 22,
		/// <summary><c>ES:[RDI]</c>. This operand kind uses <see cref="Instruction.MemorySize"/></summary>
		MemoryESRDI = 23,
		/// <summary>Memory operand.<br/>
		/// <br/>
		/// This operand kind uses <see cref="Instruction.MemoryDisplSize"/>, <see cref="Instruction.MemorySize"/>, <see cref="Instruction.MemoryIndexScale"/>, <see cref="Instruction.MemoryDisplacement64"/>, <see cref="Instruction.MemoryBase"/>, <see cref="Instruction.MemoryIndex"/>, <see cref="Instruction.MemorySegment"/>, <see cref="Instruction.SegmentPrefix"/></summary>
		Memory = 24,
	}
}
