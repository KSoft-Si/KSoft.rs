#[cfg(not(any(feature = "blocking", feature = "default")))]
compile_error!("No features enabled, please enable one of `default` (non-blocking) or `blocking` in order to compile and run properly");

fn main() {}