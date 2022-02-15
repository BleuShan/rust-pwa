use crate::prelude::*;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("\"{0}\" is invalid value for from_str")]
    FromStr(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AcceptEncodingKind {
    Wildcard,
    Identity,
    Gzip,
    Compress,
    Deflate,
    Br,
}

impl FromStr for AcceptEncodingKind {
    type Err = ParseError;
    fn from_str(s: &str) -> StdResult<Self, <Self as std::str::FromStr>::Err> {
        match s.to_lowercase().as_str() {
            "*" => Ok(Self::Wildcard),
            "identity" => Ok(Self::Identity),
            "gzip" => Ok(Self::Gzip),
            "compress" => Ok(Self::Compress),
            "deflate" => Ok(Self::Deflate),
            "br" => Ok(Self::Br),
            _ => Err(ParseError::FromStr(s.to_owned())),
        }
    }
}

impl Default for AcceptEncodingKind {
    fn default() -> Self {
        Self::Wildcard
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct QualityValue(Option<f64>);
impl PartialEq for QualityValue {
    fn eq(&self, other: &Self) -> bool {
        let val = self.0.unwrap_or_default();
        let other_val = other.0.unwrap_or_default();
        val == other_val
    }
}

impl PartialOrd for QualityValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let val = self.0.unwrap_or_default();
        let other_val = other.0.unwrap_or_default();
        Some(val.total_cmp(&other_val))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AcceptEncoding {
    kind: AcceptEncodingKind,
    quality: QualityValue,
}

impl PartialOrd for AcceptEncoding {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quality.partial_cmp(&other.quality)
    }
}
