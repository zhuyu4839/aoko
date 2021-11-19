use crate::no_std::ext::AnyExt1;
use std::{prelude::v1::*, io::stdin, ops::BitXorAssign, time::{Instant, Duration}};

/// Reads a line of input from the standard input stream.
///
/// Returns a `Some` value when content is not empty.
///
/// # Examples
///
/// ```no_run
/// use aoko::standard::fun::*;
/// 
/// // If you ensure the input is not empty, or want to panic if it is empty, use:
/// read_line().unwrap();
///
/// // If you want to return a default value when the input is empty, use:
/// read_line().unwrap_or(String::from("default_value"));
///
/// // If you just want empty value is "", use:
/// read_line().unwrap_or_default();
/// ```
pub fn read_line() -> Option<String> {
    String::new().also_mut(|s| stdin().read_line(s))
        .trim_end().to_string()
        .let_owned(|s| if s.len() > 0 { Some(s) } else { None })
}

/// Breaks loop when command line input is empty. (Press `Enter` to exit.)
///
/// Usually used in command line program as `.exe` file on Windows to prevent it from exiting directly.
pub fn wait_enter() {
    while read_line() != None {}
}

/// Swaps two variables' value.
/// 
/// # Examples
/// ```
/// use aoko::standard::fun::*;
/// 
/// let a = &mut 1024; let b = &mut 2048;
/// swap_xor(a, b);
/// assert_eq!((2048, 1024), (*a, *b));
/// ```
pub fn swap_xor<T>(a: &mut T, b: &mut T) where T: BitXorAssign<T> + Copy {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

/// Executes the given closure block and returns the duration of elapsed time interval.
pub fn measure_time<R>(f: impl FnOnce() -> R) -> Duration {
    Instant::now().also_ref(|_| f()).elapsed()
}

/// Executes the given closure block,
/// returns the result of the closure execution and the duration of elapsed time interval.
pub fn measure_time_with_value<R>(f: impl FnOnce() -> R) -> (R, Duration) {
    Instant::now().let_owned(|s| (f(), s.elapsed()))
}

/// Takes a `&str` time unit as a parameter,
/// returns conversion function.
pub fn time_conversion(u: &str) -> impl FnOnce(Duration) -> u128 {
    match u {
        "nanos" => |elapsed: Duration| elapsed.as_nanos(),
        "micros" => |elapsed: Duration| elapsed.as_micros(),
        "millis" => |elapsed: Duration| elapsed.as_millis(),
        "secs" => |elapsed: Duration| elapsed.as_secs() as u128,
        _ => panic!("unsupported unit")
    }
}

/// Takes a `String` time unit as a parameter,
/// returns conversion function and the unit.
pub fn time_conversion_with_unit(u: String) -> (impl FnOnce(Duration) -> u128, String) {
    time_conversion(&u).let_owned(|f| (f, u))
}
