// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use scale_info::{TypeInfo, Registry, meta_type};
use scale::Encode;
use std::io::Write;

/// Module defining the types from which the scale-info is generated.
///
/// Run `cargo run -p scale-info-test-suite` in order to generate the SCALE encoded representation
/// of these types, from which compatible types can be generated by the macro in the tests below.
mod source_types {
    use super::*;

    #[allow(unused)]
    #[derive(TypeInfo, Encode)]
    pub struct Combined (S, Parent);

    #[allow(unused)]
    #[derive(TypeInfo, Encode)]
    pub struct S {
        pub a: bool,
        pub b: u32,
    }

    #[allow(unused)]
    #[derive(TypeInfo, Encode)]
    struct Parent {
        a: bool,
        b: Child,
    }

    #[allow(unused)]
    #[derive(TypeInfo, Encode)]
    struct Child {
        a: i32,
    }
}

/// Writes the SCALE encoded type registry to the `encoded` dir for testing the type gen macro.
fn main() {
    let mut registry = Registry::new();
    registry.register_type(&meta_type::<source_types::Combined>());

    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
    let root = std::path::Path::new(&root);
    let path = root.join("encoded/types.scale");

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(&registry.encode()).unwrap();
}
