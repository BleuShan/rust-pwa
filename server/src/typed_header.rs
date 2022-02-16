use std::default;

use crate::prelude::*;
use once_cell::sync::Lazy;
static ACCEPT_ENCODING_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?P<kind>\w+)(?:;q=(?P<quality>.+))?"#).unwrap());

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("\"{0}\" is invalid value for AcceptEncodingKind")]
    InvalidAcceptEncodingKind(String),
    #[error("\"{0}\" is invalid value for QualityValue")]
    InvalidQualityValue(String),
    #[error("\"{0}\" is invalid value for AcceptEncoding")]
    InvalidAcceptEncodingValue(String),
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
            _ => Err(ParseError::InvalidAcceptEncodingKind(s.to_owned())),
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

impl FromStr for QualityValue {
    type Err = ParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let value =
            f64::from_str(s).map_err(|_val| ParseError::InvalidQualityValue(s.to_owned()))?;

        Ok(Self(Some(value)))
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

impl FromStr for AcceptEncoding {
    type Err = ParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if let Some(captures) = ACCEPT_ENCODING_REGEX.captures(s) {
            let maybe_kind = captures
                .name("kind")
                .map(|matched| AcceptEncodingKind::from_str(matched.as_str()).ok())
                .flatten();
            let maybe_quality = captures
                .name("quality")
                .map(|matched| QualityValue::from_str(matched.as_str()).ok())
                .flatten();
            return match (maybe_kind, maybe_quality) {
                (Some(kind), Some(quality)) => Ok(Self { kind, quality }),
                (Some(kind), None) => Ok(Self {
                    kind,
                    quality: Default::default(),
                }),
                _ => Err(ParseError::InvalidAcceptEncodingValue(s.to_owned())),
            };
        }
        Err(ParseError::InvalidAcceptEncodingValue(s.to_owned()))
    }
}
