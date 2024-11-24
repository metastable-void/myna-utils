use std::{fmt::Display, str::FromStr};

use sha2::Sha256;
use hmac::{Hmac, Mac};

#[derive(Debug, PartialEq, Eq)]
pub enum MynaError {
    ParseError(String),
    InvalidInput(String),
}

impl Display for MynaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MynaError::ParseError(s) => write!(f, "Parse error: {}", s),
            MynaError::InvalidInput(s) => write!(f, "Invalid input: {}", s),
        }
    }
}

impl std::error::Error for MynaError {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Myna {
    digits: [u8; 11],
}

const Q_N: [u8; 11] = [6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];

impl Myna {
    pub const ZERO: Myna = Myna { digits: [0; 11] };

    fn check_digit(&self) -> u8 {
        let mut sum = 0u16;
        for i in 0..11 {
            sum += self.digits[i] as u16 * Q_N[i] as u16;
        }
        let remainder = (sum % 11u16) as u8;
        if remainder == 0 || remainder == 1 {
            0
        } else {
            11 - remainder
        }
    }

    pub fn parse(input: &str) -> Result<Myna, MynaError> {
        let input = input.trim().as_bytes();
        if input.len() != 12 {
            return Err(MynaError::InvalidInput("Input must be 12 characters long".to_string()));
        }

        let mut bytes = [0; 12];
        for i in 0..12 {
            bytes[i] = match input[i] {
                b'0'..=b'9' => input[i] - b'0',
                _ => return Err(MynaError::ParseError("Invalid character".to_string())),
            };
        }

        let myna = Myna {
            digits: [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10]],
        };

        if myna.check_digit() != bytes[11] {
            return Err(MynaError::ParseError("Invalid check digit".to_string()));
        }

        Ok(myna)
    }

    pub fn increment(&mut self) {
        let mut i = 10;
        loop {
            if self.digits[i] == 9 {
                self.digits[i] = 0;
                if i == 0 {
                    break;
                }
                i -= 1;
            } else {
                self.digits[i] += 1;
                break;
            }
        }
    }
}

impl FromStr for Myna {
    type Err = MynaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Myna::parse(s)
    }
}

impl Display for Myna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zero = b'0';
        for i in 0..11 {
            write!(f, "{}", (self.digits[i] + zero) as char)?;
        }
        write!(f, "{}", self.check_digit())?;
        Ok(())
    }
}

impl Default for Myna {
    fn default() -> Self {
        Myna {
            digits: [0; 11],
        }
    }
}

pub struct MynaIter {
    myna: Myna,
}

impl MynaIter {
    pub fn new() -> MynaIter {
        MynaIter { myna: Myna::default() }
    }
}

impl Iterator for MynaIter {
    type Item = Myna;

    fn next(&mut self) -> Option<Self::Item> {
        let myna = self.myna;
        self.myna.increment();
        if self.myna == Myna::ZERO {
            return None;
        }
        Some(myna)
    }
}

type HmacSha256 = Hmac<Sha256>;

pub struct MynaDb {
    secret: Vec<u8>,
}

impl MynaDb {
    pub fn new(secret: &str) -> MynaDb {
        MynaDb {
            secret: secret.as_bytes().to_vec(),
        }
    }

    fn hash(&self, myna: &Myna) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(&self.secret).unwrap();
        mac.update(&myna.to_string().as_bytes());
        mac.finalize().into_bytes().to_vec()
    }

    fn uuid(&self, myna: &Myna) -> String {
        let hash = self.hash(myna);
        let mut uuid = [0u8; 16];
        uuid.copy_from_slice(&hash[..16]);

        // Set the version to 4 (random)
        uuid[6] = (uuid[6] & 0x0F) | 0x40;
        // Set the variant to DCE 1.1
        uuid[8] = (uuid[8] & 0x3F) | 0x80;

        let uuid = uuid.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
        format!(
            "{}-{}-{}-{}-{}",
            &uuid[0..8],
            &uuid[8..12],
            &uuid[12..16],
            &uuid[16..20],
            &uuid[20..32]
        )
    }

    pub fn get_line(&self, myna: &Myna) -> String {
        format!("{} {}\n", myna, self.uuid(myna))
    }
}

