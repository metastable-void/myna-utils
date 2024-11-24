
use myna_utils::*;

use std::io::Write;

fn main() {
    let secret = std::env::args().nth(1).unwrap();
    let db = MynaDb::new(secret.trim());
    for myna in MynaIter::new() {
        let line = db.get_line(&myna);
        std::io::stdout().write_all(line.as_bytes()).unwrap();
    }
}
