use std::str::FromStr;
use std::time::Duration;

use crate::field::*;
use crate::*;
use crate::{Error, TaosError};
use itertools::Itertools;
#[cfg(not(feature = "rest"))]
use log::*;
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;
#[derive(Debug, Clone)]
pub struct Taos {
    client: reqwest::Client,
    endpoint: String,
    username: String,
    password: String,
}

// #[derive(Debug, Deserialize)]
// struct TaosQueryDataProxy {
//     status: String,
//     head: Vec<String>,
//     column_meta: Vec<ColumnMeta>,
//     data: Vec<Vec<serde_json::Value>>,
//     rows: usize,
// }

// #[derive(Debug, Deserialize)]
// struct TaosQueryError {
//     status: String,
//     code: TaosCode,
//     desc: String,
// }

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum TaosQueryResponse {
    Data {
        status: String,
        head: Vec<String>,
        column_meta: Vec<ColumnMeta>,
        data: Vec<Vec<serde_json::Value>>,
        rows: usize,
    },
    Error {
        status: String,
        code: i32,
        desc: String,
    },
}

fn value_to_field(from: &TaosQueryResponse, value: Value, meta: &ColumnMeta) -> Field {
    if value.is_null() {
        return Field::Null;
    }
    match meta.type_ {
        TaosDataType::Null => Field::Null,
        TaosDataType::TinyInt => Field::TinyInt(
            value
                .as_i64()
                .expect("the column declared as tinyint but not") as i8,
        ),
        TaosDataType::SmallInt => Field::SmallInt(
            value
                .as_i64()
                .expect("the column declared as smallint but not") as i16,
        ),
        TaosDataType::Int => {
            Field::Int(value.as_i64().expect("the column declared as int but not") as i32)
        }
        TaosDataType::BigInt => Field::BigInt(
            value
                .as_i64()
                .expect("the column declared as bigint but not") as i64,
        ),
        TaosDataType::UTinyInt => Field::UTinyInt(
            value
                .as_u64()
                .expect("the column declared as unsigned tinyint but not") as u8,
        ),
        TaosDataType::USmallInt => Field::USmallInt(
            value
                .as_u64()
                .expect("the column declared as unsigned smallint but not") as u16,
        ),
        TaosDataType::UInt => Field::UInt(
            value
                .as_u64()
                .expect("the column declared as unsigned int but not") as u32,
        ),
        TaosDataType::UBigInt => Field::UBigInt(
            value
                .as_u64()
                .expect("the column declared as unsigned bigint but not") as u64,
        ),
        TaosDataType::Timestamp => Field::Timestamp(if value.is_i64() {
            Timestamp {
                timestamp: value.as_i64().expect("timestamp i64"),
                precision: TimestampPrecision::Milli,
            }
        } else if let Some(s) = value.as_str() {
            Timestamp::from_str(s).expect("parse timestamp format error")
        } else {
            unimplemented!("unknown timestamp format")
        }),
        TaosDataType::Float => Field::Float(
            value
                .as_f64()
                .expect("the column declared as float but not") as f32,
        ),
        TaosDataType::Double => Field::Double(
            value
                .as_f64()
                .expect("the column declared as double but not"),
        ),
        TaosDataType::Binary => match value {
            Value::String(str) => Field::Binary(str.into()),
            v => unreachable!(
                "{}",
                &format!(
                    "the column declared as binary but not: {:?}, from {:?}",
                    v, from
                )
            ),
        },
        TaosDataType::NChar => match value {
            Value::String(str) => Field::NChar(str),
            _ => unreachable!("{}", "the column declared as binary but not"),
        },
        TaosDataType::Bool => Field::Bool(
            value
                .as_bool()
                .expect("the column declared as bool but not"),
        ),
        _ => unreachable!("unkown data type"),
    }
}

impl From<TaosQueryResponse> for Result<TaosQueryData, Error> {
    fn from(from: TaosQueryResponse) -> Result<TaosQueryData, Error> {
        let from_back = from.clone();
        match from {
            #[allow(unused_variables)]
            TaosQueryResponse::Data {
                status,
                head,
                column_meta,
                data,
                rows,
            } => {
                let rows = data
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .zip(column_meta.iter())
                            .map(|(value, meta)| value_to_field(&from_back, value, meta))
                            .collect_vec()
                    })
                    .collect_vec();
                Ok(TaosQueryData { column_meta, rows })
            }
            #[allow(unused_variables)]
            TaosQueryResponse::Error { status, code, desc } => {
                Err(Error::RawTaosError(TaosError {
                    code: code.into(),
                    err: desc.into(),
                }))
            }
        }
    }
}

impl Taos {
    pub fn new(
        endpoint: String,
        username: String,
        password: String,
        token: Option<String>,
    ) -> Self {
        if let Some(t) = token {
            let url = Url::parse_with_params(&endpoint, &[("token", t)]).unwrap();
            Self {
                client: reqwest::ClientBuilder::new()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .expect("build client with timeout error"),
                endpoint: url.to_string(),
                username,
                password,
            }
        } else {
            Self {
                client: reqwest::ClientBuilder::new()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .expect("build client with timeout error"),
                endpoint,
                username,
                password,
            }
        }
    }
    pub async fn create_table(&self, table: &str, options: Option<&str>) -> Result<(), Error> {
        self.query(&format!("create table {} {}", table, options.unwrap_or("")))
            .await
            .map(|_| ())
    }

    pub async fn create_table_if_not_exists(
        &self,
        table: &str,
        options: Option<&str>,
    ) -> Result<(), Error> {
        self.query(&format!(
            "create table if not exists {} {}",
            table,
            options.unwrap_or("")
        ))
        .await
        .map(|_| ())
    }

    pub async fn create_database(&self, database: &str) -> Result<(), Error> {
        self.exec(&format!("create database {}", database)).await
    }

    pub async fn create_database_if_exists(&self, database: &str) -> Result<(), Error> {
        self.exec(&format!("create database if not exists {}", database))
            .await
    }

    pub async fn use_database(&self, database: &str) -> Result<(), Error> {
        self.query(&format!("use {}", database)).await.map(|_| ())
    }

    pub async fn describe(&self, table: &str) -> Result<TaosDescribe, Error> {
        self.query(&format!("describe {}", table))
            .await
            .map(|res| TaosDescribe::from(res))
    }
    async fn raw_query(&self, sql: &str) -> Result<reqwest::Response, reqwest::Error> {
        assert!(sql.len() < 65480, "sql length should be less than 65480");
        match self
            .client
            .post(&self.endpoint)
            .basic_auth(&self.username, Some(&self.password))
            .body(sql.to_string())
            .send()
            .await
        {
            Ok(res) => Ok(res),
            Err(_) => {
                self.client
                    .post(&self.endpoint)
                    .basic_auth(&self.username, Some(&self.password))
                    .body(sql.to_string())
                    .send()
                    .await
            }
        }
    }
    pub async fn exec(&self, sql: &str) -> Result<(), Error> {
        let res = self.raw_query(sql).await?;
        let res: TaosQueryResponse = res.json().await?;
        match res {
            TaosQueryResponse::Data { .. } => Ok(()),
            TaosQueryResponse::Error { code, desc, .. } => Err(Error::RawTaosError(TaosError {
                code: code.into(),
                err: desc.into(),
            })),
        }
    }
    pub async fn query(&self, sql: &str) -> Result<TaosQueryData, Error> {
        let res = self.raw_query(sql).await?;
        let res: TaosQueryResponse = res.json().await?;
        res.into()
    }
}
