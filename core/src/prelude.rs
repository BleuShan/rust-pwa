//! Core library prelude

#![allow(unused_imports)]
pub use color_eyre::{
    self,
    eyre::{
        format_err,
        Result,
        WrapErr,
    },
};
pub use dotenv;
pub use futures::{
    self,
    prelude::*,
};
pub use once_cell;
pub use serde::{
    self,
    Deserialize,
    Serialize,
};
pub use serde_json as json;
pub use thiserror::{
    self,
    Error,
};

pub use derive_more::{
    AsMut,
    AsRef,
    Deref,
    DerefMut,
    Display,
    From,
    FromStr,
    Index,
    IndexMut,
    Into,
    IntoIterator,
    TryInto,
};

pub use std::{
    backtrace::Backtrace,
    convert::{
        AsMut,
        AsRef,
        TryFrom,
    },
    error::Error as StdError,
    fmt::{
        self,
        Debug,
        Display,
    },
    io::{
        Error as IOError,
        ErrorKind as IOErrorKind,
    },
    ops::{
        Deref,
        DerefMut,
        Index,
        IndexMut,
    },
    result::Result as StdResult,
    str::FromStr,
};

/// An alias for the `()` type. Used to get a more uniform syntax.
pub type Unit = ();

/// An alias for Send + Sync
pub trait SendSync = Send + Sync;
