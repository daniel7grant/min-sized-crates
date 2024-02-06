use log_lib_size::shave_all;

fn main() {
    // Lib mode: tracing setup is up to the user
    // for example:
    // simple_logger::SimpleLogger::new().init().unwrap();

    let _ = shave_all(5);
}
