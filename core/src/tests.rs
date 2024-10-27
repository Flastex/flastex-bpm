mod process_engine_tests;
mod xml_parser_tests;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // Initialize the logger globally for all tests
    let _ = env_logger::builder().is_test(true).try_init();
}
