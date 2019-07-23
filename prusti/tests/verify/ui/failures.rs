#![allow(dead_code)]
extern crate prusti_contracts;

#[requires="false"]
fn failing_precondition(x: i32) {
    let mut x = x;
    while x < 10 {
        x += 1;
    }
}

#[ensures="false"]
fn failing_postcondition(x: i32) {
    let mut x = x;
    while x < 10 {
        x += 1;
    }
}

fn failing_loop_invariant(x: i32) {
    let mut x = x;
    #[invariant="false"]
    while x < 10 {
        x += 1;
    }
}

fn main(){
    failing_precondition(42);
}
