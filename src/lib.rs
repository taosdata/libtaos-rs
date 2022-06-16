#![cfg_attr(coverage_nightly, feature(no_coverage))]

use std::{
    borrow::Cow,
    fmt::{self, Display},
    sync::Once,
};

use derive_builder::Builder;
#[cfg(not(feature = "rest"))]
use itertools::Itertools;
use log::*;
use mdsn::{Dsn, DsnError, IntoDsn};
use thiserror::Error;

#[cfg(not(feature = "rest"))]
pub mod bindings;

#[cfg(not(feature = "rest"))]
mod util;
#[cfg(not(feature = "rest"))]
pub(crate) use util::*;

mod error;
mod timestamp;
pub use timestamp::*;

pub mod field;
#[cfg(feature = "rest")]
mod rest;
#[cfg(feature = "rest")]
pub use rest::*;

#[cfg(not(feature = "rest"))]
mod client;
#[cfg(not(feature = "rest"))]
pub use client::*;

#[cfg(all(not(feature = "rest"), feature = "stmt"))]
pub mod stmt;

#[cfg(all(not(feature = "rest"), feature = "schemaless"))]
pub mod schemaless;

pub use error::*;
pub use field::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot get TDengine connection")]
    ConnectionInvalid,
    #[error("parse dsn error: {0}")]
    DsnError(#[from] DsnError),
    #[error("Dsn error: {0} (dsn: {1})")]
    DsnParseError(String, Dsn),
    #[error("taos error: {0}")]
    RawTaosError(#[from] TaosError),
    #[cfg(feature = "rest")]
    #[error("rest error: {0}")]
    RestApiError(#[from] reqwest::Error),
}

#[derive(Error, Debug)]
pub struct TaosError {
    pub code: TaosCode,
    pub err: Cow<'static, str>,
}

impl Display for TaosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.err)
    }
}

#[derive(Builder, Debug)]
#[builder(default)]
pub struct TaosCfg {
    #[builder(setter(strip_option, into))]
    scheme: Option<String>,
    #[builder(setter(strip_option, into))]
    ip: Option<String>,
    #[builder(setter(strip_option, into))]
    user: Option<String>,
    #[builder(setter(strip_option, into))]
    pass: Option<String>,
    #[builder(setter(strip_option, into))]
    db: Option<String>,
    #[builder(setter(strip_option, into))]
    port: Option<u16>,
    #[builder(setter(into))]
    token: Option<String>,
    #[builder(setter(skip))]
    ping_once: Once,
}

impl Default for TaosCfg {
    fn default() -> Self {
        Self {
            scheme: None,
            ip: Default::default(),
            user: Default::default(),
            pass: Default::default(),
            db: Default::default(),
            port: Default::default(),
            token: Default::default(),
            ping_once: Once::new(),
        }
    }
}

impl TaosCfg {
    /// Create cfg from dsn.
    ///
    /// ```text
    /// taos://localhost:6030:
    /// ```
    pub fn from_dsn<T: IntoDsn>(dsn: T) -> Result<Self, Error> {
        let dsn = dsn.into_dsn()?;
        let scheme = match (
            dsn.driver.as_str(),
            dsn.protocol.as_ref().map(|s| s.as_str()),
        ) {
            ("taos", None) => None,
            ("taos", Some("http")) | ("taos", Some("https")) => {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "rest")] {
                        dsn.protocol
                    } else {
                        Err(Error::DsnParseError("http(s) protocol is not valid without feature rest".to_string(), dsn.clone()))?
                    }
                }
            }
            ("http", None) | ("https", None) => {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "rest")] {
                        Some(dsn.driver)
                    } else {
                        Err(Error::DsnParseError("http(s) protocol is not valid without feature rest".to_string(), dsn.clone()))?
                    }
                }
            },
            ("taos", Some(protocol)) => Err(Error::DsnParseError(
                format!("TaosCfg does not support protocol {protocol}"),
                dsn.clone(),
            ))?,
            (driver, _) => Err(Error::DsnParseError(
                format!("TaosCfg does not support driver {driver}"),
                dsn.clone(),
            ))?,
        };
        let port = dsn.addresses.first().and_then(|addr| addr.port);
        let token = dsn.params.get("token").map(|t| t.to_string());
        Ok(Self {
            scheme,
            ip: dsn
                .addresses
                .first()
                .and_then(|addr| addr.host.as_ref().map(Clone::clone)),
            user: dsn.username,
            pass: dsn.password,
            db: dsn.database,
            port,
            token,
            ping_once: Once::new(),
        })
    }

    #[cfg(feature = "rest")]
    pub fn connect(&self) -> Result<Taos, Error> {
        let user = self.user.as_deref().unwrap_or("root").to_string();
        let pass = self.pass.as_deref().unwrap_or("taosdata").to_string();
        let port = self
            .port
            .map(|p| if p == 6030 { 6041 } else { p })
            .unwrap_or(6041);
        let scheme = self.scheme.as_ref().map(|s| s.as_str()).unwrap_or("http");
        let taos = match self.db.as_ref() {
            Some(db) => match self.token.as_ref() {
                Some(token) => Taos::new(
                    format!(
                        "{}://{}:{}/rest/sql/{}?token={}",
                        scheme,
                        self.ip.as_deref().unwrap_or("localhost"),
                        port,
                        db,
                        token
                    ),
                    user,
                    pass,
                ),
                None => Taos::new(
                    format!(
                        "{}://{}:{}/rest/sql/{}",
                        scheme,
                        self.ip.as_deref().unwrap_or("localhost"),
                        port,
                        db
                    ),
                    user,
                    pass,
                ),
            },
            None => match self.token.as_ref() {
                Some(token) => Taos::new(
                    format!(
                        "{}://{}:{}/rest/sql?token={}",
                        scheme,
                        self.ip.as_deref().unwrap_or("localhost"),
                        port,
                        token
                    ),
                    user,
                    pass,
                ),
                None => Taos::new(
                    format!(
                        "{}://{}:{}/rest/sql",
                        scheme,
                        self.ip.as_deref().unwrap_or("localhost"),
                        port,
                    ),
                    user,
                    pass,
                ),
            },
        };
        let mut res = None;
        self.ping_once.call_once(|| {
            if let Err(err) = futures::executor::block_on(taos.exec("select server_version()")) {
                res = Some(err)
            }
        });
        if let Some(err) = res {
            Err(err)
        } else {
            Ok(taos)
        }
    }

    #[cfg(not(feature = "rest"))]
    pub fn connect(&self) -> Result<Taos, Error> {
        Taos::new(
            &self.ip,
            &self.user,
            &self.pass,
            &self.db,
            self.port.unwrap_or(0),
        )
    }
}

#[cfg(feature = "r2d2")]
pub type TaosPool = r2d2::Pool<TaosCfg>;

#[cfg(feature = "r2d2")]
impl r2d2::ManageConnection for TaosCfg {
    type Connection = Taos;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.connect()
    }

    fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

#[cfg(test)]
pub mod test;
