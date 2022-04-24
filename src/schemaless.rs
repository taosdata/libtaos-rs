use crate::bindings::taos_schemaless_insert;
use crate::*;
use crate::{CTaosResult, Taos};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TSDB_SML_PROTOCOL_TYPE {
    Unknown = 0,
    Line,
    Telnet,
    Json,
}
pub const TSDB_SML_UNKNOWN_PROTOCOL: TSDB_SML_PROTOCOL_TYPE = TSDB_SML_PROTOCOL_TYPE::Unknown;
pub const TSDB_SML_LINE_PROTOCOL: TSDB_SML_PROTOCOL_TYPE = TSDB_SML_PROTOCOL_TYPE::Line;
pub const TSDB_SML_TELNET_PROTOCOL: TSDB_SML_PROTOCOL_TYPE = TSDB_SML_PROTOCOL_TYPE::Telnet;
pub const TSDB_SML_JSON_PROTOCOL: TSDB_SML_PROTOCOL_TYPE = TSDB_SML_PROTOCOL_TYPE::Json;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TSDB_SML_TIMESTAMP_TYPE {
    NonConfigured = 0,
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

pub const TSDB_SML_TIMESTAMP_NOT_CONFIGURED: TSDB_SML_TIMESTAMP_TYPE =
    TSDB_SML_TIMESTAMP_TYPE::NonConfigured;
pub const TSDB_SML_TIMESTAMP_HOURS: TSDB_SML_TIMESTAMP_TYPE = TSDB_SML_TIMESTAMP_TYPE::Hours;
pub const TSDB_SML_TIMESTAMP_MINUTES: TSDB_SML_TIMESTAMP_TYPE = TSDB_SML_TIMESTAMP_TYPE::Minutes;
pub const TSDB_SML_TIMESTAMP_SECONDS: TSDB_SML_TIMESTAMP_TYPE = TSDB_SML_TIMESTAMP_TYPE::Seconds;
pub const TSDB_SML_TIMESTAMP_MILLISECONDS: TSDB_SML_TIMESTAMP_TYPE =
    TSDB_SML_TIMESTAMP_TYPE::Milliseconds;
pub const TSDB_SML_TIMESTAMP_MICROSECONDS: TSDB_SML_TIMESTAMP_TYPE =
    TSDB_SML_TIMESTAMP_TYPE::Microseconds;
pub const TSDB_SML_TIMESTAMP_NANOSECONDS: TSDB_SML_TIMESTAMP_TYPE =
    TSDB_SML_TIMESTAMP_TYPE::Nanoseconds;

impl Taos {
    /// Schemaless insert with different protocol and timestamp precision.
    ///
    /// - InfluxDB line protocol
    ///
    ///     ```ignore,rust
    ///     let lines = ["st,t1=abc,t2=def,t3=anything c1=3i64,c3=L\"pass\",c2=false,c4=4f64 1626006833639000000"];
    ///     taos.schemaless_insert(&lines, TSDB_SML_LINE_PROTOCOL, TSDB_SML_TIMESTAMP_NANOSECONDS)?;
    ///     ```
    ///
    /// - OpenTSDB telnet protocol
    ///
    ///     ```ignore,rust
    ///     let lines = ["sys.if.bytes.out 1479496100 1.3E3 host=web01 interface=eth0"];
    ///     taos.schemaless_insert(&lines, TSDB_SML_TELNET_PROTOCOL, TSDB_SML_TIMESTAMP_NOT_CONFIGURED)?;
    ///     ```
    ///
    /// - or OpenTSDB json protocol.
    ///
    ///     ```ignore,rust
    ///     let lines = [r#"
    ///         {
    ///             "metric":   "st",
    ///             "timestamp":        1626006833,
    ///             "value":    10,
    ///             "tags":     {
    ///                 "t1":   true,
    ///                 "t2":   false,
    ///                 "t3":   10,
    ///                 "t4":   "123_abc_.!@#$%^&*:;,./?|+-=()[]{}<>"
    ///             }
    ///         }"#];
    ///     taos.schemaless_insert(&lines, TSDB_SML_JSON_PROTOCOL, TSDB_SML_TIMESTAMP_NOT_CONFIGURED)?;
    ///     ```
    ///
    pub fn schemaless_insert(
        &self,
        lines: &[impl ToCString],
        protocol: TSDB_SML_PROTOCOL_TYPE,
        precision: TSDB_SML_TIMESTAMP_TYPE,
    ) -> Result<i32, TaosError> {
        let lines: Vec<_> = lines.iter().map(|line| line.to_c_string()).collect();
        let mut lines = lines.iter().map(|line| line.as_ptr()).collect_vec();
        let lines = lines.as_mut_slice();
        unsafe {
            let res = taos_schemaless_insert(
                self.as_raw(),
                lines.as_mut_ptr() as *mut *mut _,
                lines.len() as _,
                protocol as _,
                precision as _,
            );

            let res = CTaosResult::new(res)?;

            Ok(res.affected_rows())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::schemaless::*;
    use crate::test::taos;

    #[tokio::test]
    /// Test schemaless insert with InfluxDB line protocol
    async fn line_insert() -> Result<(), Error> {
        let taos = taos()?;

        let db = "rs_test_line";
        println!("test using {}", db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;

        let lines = ["st,t1=abc,t2=def,t3=anything c1=3i64,c3=L\"pass\",c2=false,c4=4f64 1626006833639000000"];
        let res = taos.schemaless_insert(
            &lines,
            TSDB_SML_LINE_PROTOCOL,
            TSDB_SML_TIMESTAMP_NOT_CONFIGURED,
        )?;
        assert_eq!(res, 1);

        let res = taos.query("select * from st").await?;
        println!("{res:?}");

        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }

    #[tokio::test]
    /// Test schemaless insert with OpenTSDB telnet protocol
    async fn telnet_insert() -> Result<(), Error> {
        let taos = taos()?;

        let db = "rs_test_sml_telnet";
        println!("test using {}", db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        let lines = [
            "sys.if.bytes.out 1479496100 1.3E3 host=web01 interface=eth0",
            "sys.if.bytes.out 1479496200 1.4E3 host=web02 interface=eth1",
            "sys.if.bytes.out 1479496300 2.1E3 host=web03 interface=eth2",
            "sys.if.bytes.out 1479496400 3.5E3 host=web04 interface=eth3",
        ];

        let res =
            taos.schemaless_insert(&lines, TSDB_SML_TELNET_PROTOCOL, TSDB_SML_TIMESTAMP_SECONDS)?;
        assert_eq!(res, 4);

        let res = taos.query("select * from `sys.if.bytes.out`").await?;
        println!("{res:?}");

        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }

    #[tokio::test]
    /// Test schemaless insert with OpenTSDB json protocol
    async fn json_insert() -> Result<(), Error> {
        let taos = taos()?;

        let db = "rs_test_sml_json";
        println!("test using {}", db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        let lines = [r#"
        {
            "metric":   "st",
            "timestamp":        1626006833,
            "value":    10,
            "tags":     {
                "t1":   true,
                "t2":   false,
                "t3":   10,
                "t4":   "123_abc_.!@#$%^&*:;,./?|+-=()[]{}<>"
            }
        }"#];
        let _: serde_json::Value = serde_json::from_str(lines[0]).unwrap();

        let res =
            taos.schemaless_insert(&lines, TSDB_SML_JSON_PROTOCOL, TSDB_SML_TIMESTAMP_SECONDS)?;
        assert_eq!(res, 1);

        let res = taos.query("select * from st").await?;
        println!("{res:?}");

        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }
}
