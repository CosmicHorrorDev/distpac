use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bytes(f32);

impl From<f32> for Bytes {
    fn from(amount: f32) -> Self {
        Self(amount)
    }
}

impl FromStr for Bytes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split_whitespace();

        let amount: f32 = pieces
            .next()
            .ok_or(Self::Err::InvalidByteFormat)?
            .parse()
            .map_err(|_| Self::Err::InvalidByteFormat)?;
        let modifier = match pieces.next().unwrap_or("B") {
            "B" => 1.0,
            "KB" => 1_024.0,
            "MB" => 1_024.0 * 1_024.0,
            "GB" => 1_024.0 * 1_024.0,
            _ => return Err(Self::Err::InvalidByteFormat),
        };

        Ok(Self(amount * modifier))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() -> Result<(), Error> {
        let bytes: Bytes = "786.8 MB".parse()?;
        assert_eq!(bytes, Bytes(786.8 * 1_024.0 * 1_024.0));

        Ok(())
    }
}
