// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

mod process_engine_tests;
mod xml_parser_tests;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // Initialize the logger globally for all tests
    let _ = env_logger::builder().is_test(true).try_init();
}
