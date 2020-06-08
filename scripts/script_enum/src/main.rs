use heck::{CamelCase, TitleCase};
use regex::Regex;
use std::fs;

fn main() {
    let scripts = Script::from_harfbuzz();

    // TODO Could use `code` as descriminant to allow for zero-cost conversion.

    println!(
        "// This file was auto-generated using `scripts/enum_script.rs`.

use crate::sys;

/// All scripts supported by HarfBuzz
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum Script {{"
    );

    for script in scripts.iter() {
        println!("/// The {} script", script.rust_name().to_title_case());
        println!("{},", script.rust_name());
    }
    println!(
            "/// Getting the script failed or the script is invalid.
    Invalid,
}}


impl Script {{
    /// Get the corresponding `Script` from a `hb_script_t`.
    pub fn from_raw(raw: sys::hb_script_t) -> Self {{
        match raw {{"
    );

    for script in scripts.iter() {
        println!(
            "sys::{} => Script::{},",
            script.hb_name(),
            script.rust_name()
        );
    }

    println!(
        "        0 => Script::Invalid,
        _ => panic!(\"unrecognised script\"),
        }}
    }}

    /// Get the corresponding `hb_script_t` from a `Script`.
    pub fn as_raw(&self) -> sys::hb_script_t {{
        match self {{"
    );

    for script in scripts.iter() {
        println!(
            "Script::{} => sys::{},",
            script.rust_name(),
            script.hb_name(),
        );
    }
    println!(
        "       Script::Invalid => 0
        }}
    }}
}}"
    );
}

#[derive(Debug)]
pub struct Script {
    name: String,
    age: String,
    script_code_tag: [char; 4],
    script_code: u32,
}

impl Script {
    fn hb_name(&self) -> String {
        format!("HB_SCRIPT_{}", self.name)
    }

    fn rust_name(&self) -> String {
        self.name.to_camel_case()
    }

    fn from_harfbuzz() -> Vec<Self> {
        let scripts_source = fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../harfbuzz-sys/harfbuzz/src/hb-common.h"
        ))
        .unwrap();

        // Deduced by inspection of harfbuzz source.
        let script_regex = Regex::new(
            r"^\s*/\*([0-9.]+)\*/\s*HB_SCRIPT_(\w+)\s*=\s*HB_TAG\s*\('(\w)','(\w)','(\w)','(\w)'\),$",
        )
        .unwrap();

        let tag_part = |matches: &regex::Captures, idx: usize| {
            matches.get(idx).unwrap().as_str().chars().next().unwrap()
        };

        scripts_source
            .lines()
            .filter_map(|line| {
                script_regex.captures(line).map(|matches| {
                    let tag = [
                        tag_part(&matches, 3),
                        tag_part(&matches, 4),
                        tag_part(&matches, 5),
                        tag_part(&matches, 6),
                    ];
                    Script {
                        name: matches.get(2).unwrap().as_str().to_string(),
                        age: matches.get(1).unwrap().as_str().to_string(),
                        script_code_tag: tag,
                        script_code: tag_to_code(tag),
                    }
                })
            })
            .collect()
    }
}

fn tag_to_code(tag: [char; 4]) -> u32 {
    ((tag[0] as u32 & 0xff) << 24)
        + ((tag[1] as u32 & 0xff) << 16)
        + ((tag[2] as u32 & 0xff) << 8)
        + (tag[3] as u32 & 0xff)
}
