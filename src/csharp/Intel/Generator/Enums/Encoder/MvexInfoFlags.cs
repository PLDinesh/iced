// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

namespace Generator.Enums.Encoder {
	[Enum("MvexInfoFlags", Flags = true, NoInitialize = true)]
	enum MvexInfoFlags {
		None					= 0,
		NDD						= 0x01,
		NDS						= 0x02,
		// Eviction hint {eh} is supported if it has a memory operand
		EvictionHint			= 0x04,
		// imm8 rounding control is supported
		ImmRoundingControl		= 0x08,
	}
}
