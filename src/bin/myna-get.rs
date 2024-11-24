
use myna_utils::*;

use std::io::Write;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let secret = std::env::args().nth(1).unwrap();
    let mut myna = String::new();
    std::io::stdin().read_to_string(&mut myna)?;
    let myna = Myna::parse(myna.trim()).map_err(|e| std::io::Error::other(e))?;
    let db = MynaDb::new(secret.trim());
    let line = db.get_line(&myna);
    std::io::stdout().write_all(line.as_bytes())?;
    Ok(())
}
