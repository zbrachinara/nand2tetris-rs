#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($arg)*);
    }
}
#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprint!($($arg)*);
    }
}

#[macro_export]
macro_rules! time {
    ($expr:expr) => {{
        use ::std::time::Instant;

        let instant = Instant::now();
        let result = $expr;
        let duration = Instant::now().duration_since(instant);

        eprintln!(
            "Result of {} took {} µs",
            stringify!($expr),
            duration.as_micros()
        );

        result
    }};
}
