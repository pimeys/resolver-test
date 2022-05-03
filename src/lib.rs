#[cfg(not(any(feature = "a", feature = "b")))]
compile_error!("one of 'a' or 'b' features must be enabled");
