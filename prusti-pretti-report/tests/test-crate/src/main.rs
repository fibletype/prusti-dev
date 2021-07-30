#![allow(dead_code)]

#[cfg(feature = "prusti")]
use prusti_contracts::*;

fn main() {
	let _foo = max(42, 11);
	// println!("foo: {}", foo); // unsupported
	// assert_eq!(42, max(11, 42)); // unsupported
}

#[ensures(result >= a && result >= b)]
#[ensures(result == a || result == b)]
fn max(a: i32, b: i32) -> i32 {
	if a > b {
		a
	} else {
		b
	}
}


#[pure]
#[ensures(result == (slice.len() == 0))]
pub fn is_empty<T>(slice: &[T]) -> bool {
	slice.len() == 0
	// slice.is_empty()
	// true
}


#[ensures(*x == old(*x) + 1)]
pub fn inc(x: &mut u32) { *x += 1; }
