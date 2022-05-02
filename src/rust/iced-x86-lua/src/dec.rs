// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::instr::Instruction;
use mlua::prelude::*;
use std::slice;

struct Decoder {
	// The decoder holds a reference to this data
	#[allow(dead_code)]
	data: Vec<u8>,
	decoder: iced_x86::Decoder<'static>,
}

impl Decoder {
	fn new(bitness: u32, lua_data: LuaString<'_>, options: u32, ip: u64) -> mlua::Result<Self> {
		let data = lua_data.as_bytes().to_vec();
		let decoder_data: &'static [u8] = unsafe { slice::from_raw_parts(data.as_ptr(), data.len()) };

		let decoder = iced_x86::Decoder::try_with_ip(bitness, decoder_data, ip, options).map_err(mlua::Error::external)?;
		Ok(Decoder { data, decoder })
	}

	fn decode(&mut self) -> Instruction {
		let mut instr = Instruction::new();
		self.decode_out(&mut instr);
		instr
	}

	fn decode_out(&mut self, instr: &mut Instruction) {
		self.decoder.decode_out(&mut instr.instr);
	}
}

impl LuaUserData for Decoder {}

#[mlua::lua_module]
fn iced_x86_priv_dec(lua: &Lua) -> LuaResult<LuaTable<'_>> {
	let exports = lua.create_table()?;
	lua_ctor!(lua, exports = fn decoder_new(bitness: u32, data: LuaString<'_>, options: u32, ip: u64,) {
		Ok(Decoder::new(bitness, data, options, ip))
	});
	lua_getter!(lua, exports = fn decoder_ip(this: Decoder) { Ok(this.decoder.ip()) });
	lua_setter!(lua, exports = fn decoder_set_ip(this: Decoder, value: u64) {
		this.decoder.set_ip(value);
		Ok(())
	});
	lua_getter!(lua, exports = fn decoder_bitness(this: Decoder) { Ok(this.decoder.bitness()) });
	lua_getter!(lua, exports = fn decoder_max_position(this: Decoder) { Ok(this.decoder.max_position()) });
	lua_getter!(lua, exports = fn decoder_position(this: Decoder) { Ok(this.decoder.position()) });
	lua_setter!(lua, exports = fn decoder_set_position(this: Decoder, value: usize) {
		this.decoder.set_position(value).map_err(mlua::Error::external)
	});
	lua_getter!(lua, exports = fn decoder_can_decode(this: Decoder) { Ok(this.decoder.can_decode()) });
	lua_getter!(lua, exports = fn decoder_last_error(this: Decoder) { Ok(this.decoder.last_error() as u32) });
	lua_method_mut!(lua, exports = fn decoder_decode(this: Decoder,) {
		Ok(this.decode())
	});
	lua_method_mut!(lua, exports = fn decoder_decode_out(this: Decoder, instr: LuaAnyUserData<'_>,) {
		let instr = &mut *instr.borrow_mut::<Instruction>()?;
		this.decode_out(instr);
		Ok(())
	});
	Ok(exports)
}
