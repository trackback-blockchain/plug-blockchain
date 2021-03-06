// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

#[test]
fn decl_storage_ui() {
	// As trybuild is using `cargo check`, we don't need the real WASM binaries.
	std::env::set_var("BUILD_DUMMY_WASM_BINARY", "1");

	let t = trybuild::TestCases::new();
	t.compile_fail("tests/decl_storage_ui/*.rs");
}
