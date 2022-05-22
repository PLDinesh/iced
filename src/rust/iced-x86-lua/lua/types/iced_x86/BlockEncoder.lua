-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

---@meta
---@diagnostic disable unused-local

---Encodes instructions
---
---`Encoder` can only encode one instruction at a time. This class can encode any number of
---instructions and can also fix short branches if the target is too far away.
---
---It will fail if there's an instruction with an RIP-relative operand (`[rip+123h]`) and the target is too far away.
---A workaround is to use a new base RIP of the encoded instructions that is close (+/-2GB) to the original location.
---
---# Examples
---
---```lua
---from iced_x86 import *
---
---data = b"\x86\x64\x32\x16\xF0\xF2\x83\x00\x5A\x62\xC1\xFE\xCB\x6F\xD3"
---decoder = Decoder(64, data, ip=0x1234_5678)
---
---instrs = [instr for instr in decoder]
---
---encoder = BlockEncoder(64)
---# Add an instruction
---encoder.add(instrs[0])
---# Add more instructions
---encoder.add_many(instrs[1:])
---try:
---    # Encode all added instructions and get the raw bytes
---    raw_data = encoder.encode(0x3456_789A)
---except ValueError as ex:
---    print("Could not encode all instructions")
---    raise
---
---# It has no IP-relative instructions (eg. branches or [rip+xxx] ops)
---# so the result should be identical to the original code.
---assert data == raw_data
---```
---
---@class BlockEncoder
local BlockEncoder = {}

---Encodes all instructions and returns the raw bytes
---
---Error if one or more instructions couldn't be encoded.
---
---@param bitness integer #16, 32 or 64
---@param instructions Instruction[] #Instructions to encode
---@param rip integer #Base IP of all encoded instructions
---@param options integer #(optional, default = `BlockEncoderOptions.None`) Options
---@return table
function BlockEncoder.encode(bitness, instructions, rip, options) end

return BlockEncoder
