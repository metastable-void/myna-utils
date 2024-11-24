
use myna_utils::*;

fn main() {
    for myna in MynaIter::new() {
        println!("{}", myna);
    }
}
