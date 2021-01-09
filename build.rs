#[cfg(not(any(feature = "blocking", feature = "default")))]
compile_error!("No features enabled, please enable one of `default` (non-blocking) or `blocking` in order to compile and run properly");

#[cfg(all(feature = "blocking", feature = "default"))]
compile_error!("Both features `default` (non-blocking) and `blocking` enabled, please disable one in order to compile and run properly, \
if you're trying to use the blocking client, just enable the `blocking` feature and set `default-features` to `false`");


fn main() {}