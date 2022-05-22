-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

---@meta
---@diagnostic disable unused-local

---A memory location used by an instruction
---
---@class UsedMemory
local UsedMemory = {}

---Effective segment register or `Register.None` if the segment register is ignored
---
---@return integer #A `Register` enum value
function UsedMemory:segment() end

---Base register or `Register.None` if none
---
---@return integer #A `Register` enum value
function UsedMemory:base() end

---Index register or `Register.None` if none
---
---@return integer #A `Register` enum value
function UsedMemory:index() end

---Index scale (1, 2, 4 or 8)
---
---@return integer
function UsedMemory:scale() end

---Displacement
---
---@return integer
function UsedMemory:displacement() end

---Size of location (a `MemorySize` enum value)
---
---@return integer #A `MemorySize` enum value
function UsedMemory:memory_size() end

---Memory access (an `OpAccess` enum value)
---
---@return integer #An `OpAccess` enum value
function UsedMemory:access() end

---Address size (a `CodeSize` enum value)
---
---@return integer #A `CodeSize` enum value
function UsedMemory:address_size() end

---VSIB size (`0`, `4` or `8`)
---
---@return integer
function UsedMemory:vsib_size() end

---Returns a copy of this instance.
---
---@return UsedMemory #A copy of this instance
function UsedMemory:copy() end

return UsedMemory
