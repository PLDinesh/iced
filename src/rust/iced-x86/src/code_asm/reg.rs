// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

use crate::code_asm::op_state::CodeAsmOpState;
use crate::Register;

/// All 8-bit general purpose registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::gpr8::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegister8 {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegister8 {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegister8> for Register {
	#[inline]
	fn from(reg: __AsmRegister8) -> Self {
		reg.register
	}
}

/// All 16-bit general purpose registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::gpr16::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegister16 {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegister16 {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegister16> for Register {
	#[inline]
	fn from(reg: __AsmRegister16) -> Self {
		reg.register
	}
}

/// All 32-bit general purpose registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::gpr32::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
pub struct __AsmRegister32 {
	register: Register,
	state: CodeAsmOpState,
}

#[rustfmt::skip]
impl __AsmRegister32 {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register, state: CodeAsmOpState::new() }
	}
}

#[rustfmt::skip]
impl From<__AsmRegister32> for Register {
	#[inline]
	fn from(reg: __AsmRegister32) -> Self {
		reg.register
	}
}

/// All 64-bit general purpose registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::gpr64::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
pub struct __AsmRegister64 {
	register: Register,
	state: CodeAsmOpState,
}

#[rustfmt::skip]
impl __AsmRegister64 {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register, state: CodeAsmOpState::new() }
	}
}

#[rustfmt::skip]
impl From<__AsmRegister64> for Register {
	#[inline]
	fn from(reg: __AsmRegister64) -> Self {
		reg.register
	}
}

/// All segment registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::segment::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegisterSegment {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegisterSegment {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterSegment> for Register {
	#[inline]
	fn from(reg: __AsmRegisterSegment) -> Self {
		reg.register
	}
}

/// All control registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::cr::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegisterCr {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegisterCr {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterCr> for Register {
	#[inline]
	fn from(reg: __AsmRegisterCr) -> Self {
		reg.register
	}
}

/// All debug registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::dr::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegisterDr {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegisterDr {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterDr> for Register {
	#[inline]
	fn from(reg: __AsmRegisterDr) -> Self {
		reg.register
	}
}

/// All test registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::tr::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegisterTr {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegisterTr {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterTr> for Register {
	#[inline]
	fn from(reg: __AsmRegisterTr) -> Self {
		reg.register
	}
}

/// All FPU registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::st::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegisterSt {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegisterSt {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterSt> for Register {
	#[inline]
	fn from(reg: __AsmRegisterSt) -> Self {
		reg.register
	}
}

/// All MMX registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::mm::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegisterMm {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegisterMm {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterMm> for Register {
	#[inline]
	fn from(reg: __AsmRegisterMm) -> Self {
		reg.register
	}
}

/// All 128-bit vector registers (XMM).
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::xmm::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
pub struct __AsmRegisterXmm {
	register: Register,
	state: CodeAsmOpState,
}

#[rustfmt::skip]
impl __AsmRegisterXmm {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register, state: CodeAsmOpState::new() }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterXmm> for Register {
	#[inline]
	fn from(reg: __AsmRegisterXmm) -> Self {
		reg.register
	}
}

/// All 256-bit vector registers (YMM).
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::ymm::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
pub struct __AsmRegisterYmm {
	register: Register,
	state: CodeAsmOpState,
}

#[rustfmt::skip]
impl __AsmRegisterYmm {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register, state: CodeAsmOpState::new() }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterYmm> for Register {
	#[inline]
	fn from(reg: __AsmRegisterYmm) -> Self {
		reg.register
	}
}

/// All 512-bit vector registers (ZMM).
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::zmm::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
pub struct __AsmRegisterZmm {
	register: Register,
	state: CodeAsmOpState,
}

#[rustfmt::skip]
impl __AsmRegisterZmm {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register, state: CodeAsmOpState::new() }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterZmm> for Register {
	#[inline]
	fn from(reg: __AsmRegisterZmm) -> Self {
		reg.register
	}
}

/// All tile registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::tmm::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegisterTmm {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegisterTmm {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterTmm> for Register {
	#[inline]
	fn from(reg: __AsmRegisterTmm) -> Self {
		reg.register
	}
}

/// All opmask registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::k::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
pub struct __AsmRegisterK {
	register: Register,
	state: CodeAsmOpState,
}

#[rustfmt::skip]
impl __AsmRegisterK {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register, state: CodeAsmOpState::new() }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterK> for Register {
	#[inline]
	fn from(reg: __AsmRegisterK) -> Self {
		reg.register
	}
}

/// All bound registers.
///
/// This type is *not* part of the public API! It's an implementation detail.
/// The register identifiers, however, *are* part of the public API.
///
/// To use the registers, you must import everything from the module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::*;
/// ```
///
/// or import them from this module:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::*;
/// ```
///
/// or import only these registers:
///
/// ```
/// # #![allow(unused_imports)]
/// use iced_x86::code_asm::registers::bnd::*;
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip]
#[repr(transparent)]
pub struct __AsmRegisterBnd {
	register: Register,
}

#[rustfmt::skip]
impl __AsmRegisterBnd {
	pub(crate) const fn new(register: Register) -> Self {
		Self { register }
	}
}

#[rustfmt::skip]
impl From<__AsmRegisterBnd> for Register {
	#[inline]
	fn from(reg: __AsmRegisterBnd) -> Self {
		reg.register
	}
}
