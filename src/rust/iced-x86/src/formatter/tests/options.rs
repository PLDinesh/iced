/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::super::*;
use super::super::test_utils::get_formatter_unit_tests_dir;
use super::options_test_case_parser::OptionsTestParser;
use super::opts_info::*;
use super::{filter_removed_code_tests, opts_infos};
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use hashbrown::HashSet;
#[cfg(feature = "std")]
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

fn read_lines(filename: PathBuf) -> Vec<String> {
	let display_filename = filename.display().to_string();
	let file = File::open(filename).unwrap_or_else(|_| panic!("Couldn't open file {}", display_filename));
	BufReader::new(file)
		.lines()
		.map(|r| r.unwrap_or_else(|e| panic!(e.to_string())))
		.filter(|line| !line.is_empty() && !line.starts_with('#'))
		.collect()
}

fn read_infos<'a>(
	dir: &str, file_part: &str, options_file: &str, tmp_infos: &'a mut Vec<OptionsInstructionInfo>,
) -> Vec<(&'a OptionsInstructionInfo, String)> {
	let mut ignored: HashSet<u32>;
	let mut opts_filename = get_formatter_unit_tests_dir();
	opts_filename.push(dir);
	opts_filename.push(format!("{}.txt", options_file));
	ignored = HashSet::new();
	tmp_infos.extend(OptionsTestParser::new(opts_filename.as_path(), &mut ignored).into_iter());
	filter_infos(dir, file_part, tmp_infos, &ignored)
}

fn filter_infos<'a, 'b>(
	dir: &str, file_part: &str, all_infos: &'a [OptionsInstructionInfo], ignored: &'b HashSet<u32>,
) -> Vec<(&'a OptionsInstructionInfo, String)> {
	let mut filename = get_formatter_unit_tests_dir();
	filename.push(dir);
	filename.push(format!("{}.txt", file_part));
	let display_filename = filename.display().to_string();
	let lines = filter_removed_code_tests(read_lines(filename), ignored);
	if lines.len() != all_infos.len() {
		panic!("lines.len() ({}) != all_infos.len() ({}), file: {}", lines.len(), all_infos.len(), display_filename);
	}
	all_infos.iter().zip(lines.into_iter()).map(|a| (a.0, a.1)).collect()
}

#[cfg(any(feature = "gas", feature = "intel", feature = "masm", feature = "nasm"))]
pub(in super::super) fn test_format_file_common(dir: &str, file_part: &str, fmt_factory: fn() -> Box<dyn Formatter>) {
	let (all_infos, ignored): (&[OptionsInstructionInfo], &HashSet<u32>) = {
		let infos = &*opts_infos::COMMON_INFOS;
		(&infos.0, &infos.1)
	};
	let infos = filter_infos(dir, file_part, all_infos, ignored);
	test_format(infos, fmt_factory);
}

#[cfg(any(feature = "gas", feature = "intel", feature = "masm", feature = "nasm"))]
pub(in super::super) fn test_format_file_all(dir: &str, file_part: &str, fmt_factory: fn() -> Box<dyn Formatter>) {
	let (all_infos, ignored): (&[OptionsInstructionInfo], &HashSet<u32>) = {
		let infos = &*opts_infos::ALL_INFOS;
		(&infos.0, &infos.1)
	};
	let infos = filter_infos(dir, file_part, all_infos, ignored);
	test_format(infos, fmt_factory);
}

#[cfg(any(feature = "gas", feature = "intel", feature = "masm", feature = "nasm"))]
pub(in super::super) fn test_format_file(dir: &str, file_part: &str, options_file: &str, fmt_factory: fn() -> Box<dyn Formatter>) {
	let mut tmp_infos: Vec<OptionsInstructionInfo> = Vec::new();
	let infos = read_infos(dir, file_part, options_file, &mut tmp_infos);
	test_format(infos, fmt_factory);
}

#[cfg(any(feature = "gas", feature = "intel", feature = "masm", feature = "nasm"))]
fn test_format(infos: Vec<(&OptionsInstructionInfo, String)>, fmt_factory: fn() -> Box<dyn Formatter>) {
	for &(tc, ref formatted_string) in &infos {
		let mut formatter = fmt_factory();
		tc.initialize_options(formatter.options_mut());
		super::simple_format_test(tc.bitness, &tc.hex_bytes, tc.code, tc.decoder_options, formatted_string.as_str(), formatter.as_mut(), |decoder| {
			tc.initialize_decoder(decoder)
		});
	}
}

#[cfg(feature = "fast_fmt")]
pub(in super::super) fn test_format_file_common_fast(dir: &str, file_part: &str, fmt_factory: fn() -> Box<FastFormatter>) {
	let (all_infos, ignored): (&[OptionsInstructionInfo], &HashSet<u32>) = {
		let infos = &*opts_infos::COMMON_INFOS;
		(&infos.0, &infos.1)
	};
	let infos = filter_infos(dir, file_part, all_infos, ignored);
	test_format_fast(infos, fmt_factory);
}

#[cfg(feature = "fast_fmt")]
pub(in super::super) fn test_format_file_fast(dir: &str, file_part: &str, options_file: &str, fmt_factory: fn() -> Box<FastFormatter>) {
	let mut tmp_infos: Vec<OptionsInstructionInfo> = Vec::new();
	let infos = read_infos(dir, file_part, options_file, &mut tmp_infos);
	test_format_fast(infos, fmt_factory);
}

#[cfg(feature = "fast_fmt")]
fn test_format_fast(infos: Vec<(&OptionsInstructionInfo, String)>, fmt_factory: fn() -> Box<FastFormatter>) {
	for &(tc, ref formatted_string) in &infos {
		let mut formatter = fmt_factory();
		tc.initialize_options_fast(formatter.options_mut());
		super::simple_format_test_fast(
			tc.bitness,
			&tc.hex_bytes,
			tc.code,
			tc.decoder_options,
			formatted_string.as_str(),
			formatter.as_mut(),
			|decoder| tc.initialize_decoder(decoder),
		);
	}
}
