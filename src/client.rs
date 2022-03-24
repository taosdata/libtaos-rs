use crate::bindings::*;
use crate::*;

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::error::*;
use crate::field::*;

#[cfg(feature = "cleanup")]
lazy_static::lazy_static! {
    static ref TAOS_INIT_LOCK: std::sync::Mutex<u32> = std::sync::Mutex::new(0);
}

#[derive(Debug)]
pub struct Taos {
    conn: *mut TAOS,
}

unsafe impl Send for Taos {}
unsafe impl Sync for Taos {}

impl Taos {
    pub fn new<'a>(
        ip: impl Into<NullableCStr<'a>>,
        user: impl Into<NullableCStr<'a>>,
        pass: impl Into<NullableCStr<'a>>,
        db: impl Into<NullableCStr<'a>>,
        port: u16,
    ) -> Result<Self, Error> {
        let (ip, user, pass, db) = (ip.into(), user.into(), pass.into(), db.into());
        #[cfg(feature = "cleanup")]
        // Call taos_init at first connection.
        {
            let mut n = TAOS_INIT_LOCK.lock().unwrap();
            if *n == 0 {
                unsafe { taos_init() };
            }
            *n += 1;
        }
        unsafe {
            taos_options(
                TSDB_OPTION_TSDB_OPTION_CHARSET,
                "UTF-8".to_c_string().as_ptr() as _,
            );
        }
        unsafe {
            let conn =
                taos_connect(ip.as_ptr(), user.as_ptr(), pass.as_ptr(), db.as_ptr(), port).as_mut();
            match conn {
                None => Err(Error::ConnectionInvalid),
                Some(conn) => Ok(Taos { conn: conn as _ }),
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
        self.query(&format!("create database {}", database))
            .await
            .map(|_| ())
    }

    pub async fn create_database_if_exists(&self, database: &str) -> Result<(), Error> {
        self.query(&format!("create database if not exists {}", database))
            .await
            .map(|_| ())
    }

    pub async fn use_database(&self, database: &str) -> Result<(), Error> {
        self.query(&format!("use {}", database)).await.map(|_| ())
    }

    pub async fn describe(&self, table: &str) -> Result<TaosDescribe, Error> {
        self.query(&format!("describe {}", table))
            .await
            .map(TaosDescribe::from)
    }

    pub async fn exec(&self, sql: impl ToCString) -> Result<(), Error> {
        self.raw_query(sql).map(|_| ())
    }
    pub fn raw_query(&self, s: impl ToCString) -> Result<CTaosResult, Error> {
        let cstr = s.to_c_string();
        let res = CTaosResult::new(unsafe { taos_query(self.conn, cstr.as_ptr()) })?;
        Ok(res)
    }
    pub async fn query(&self, s: &str) -> Result<TaosQueryData, Error> {
        let res = self.raw_query(s)?;
        Ok(res.fetch_fields())
    }

    /// Warmup table metadata cache with a list of table name, separated by comma.
    ///
    /// ```ignore
    /// let tables = CString::new("table1,table2");
    /// taos.load_table_info(&tables).unwrap();
    /// ```
    ///
    pub fn load_table_info(&self, cstr: impl AsRef<CStr>) -> Result<(), Error> {
        unsafe {
            let code = taos_load_table_info(self.conn, cstr.as_ref().as_ptr());
            let code: TaosCode = (code & 0x0000ffff).into();
            if !code.success() {
                Err(TaosError {
                    code,
                    err: Cow::from("load table info error"),
                }
                .into())
            } else {
                Ok(())
            }
        }
    }

    pub fn as_raw(&self) -> *mut TAOS {
        self.conn
    }
}

impl Drop for Taos {
    fn drop(&mut self) {
        // reduce connection count and call clean_up after the last connection closed.
        unsafe {
            taos_close(self.conn);
        }
        #[cfg(feature = "cleanup")]
        {
            let mut n = TAOS_INIT_LOCK.lock().unwrap();
            *n -= 1;
            if *n == 0 {
                unsafe { taos_cleanup() };
            }
        }
    }
}

#[derive(Debug)]
pub struct CTaosResult {
    res: *mut bindings::TAOS_RES,
}

impl CTaosResult {
    pub fn as_raw_mut_ptr(&mut self) -> *mut bindings::TAOS_RES {
        self.res
    }

    pub fn error_code(&self) -> TaosCode {
        (unsafe { taos_errno(self.res as _) } & 0x0000ffff).into()
    }

    pub fn error_string(&self) -> String {
        let err = unsafe { taos_errstr(self.res as _) };
        unsafe {
            CStr::from_ptr(err as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn new(res: *mut TAOS_RES) -> Result<Self, TaosError> {
        let res = Self { res };
        let code = res.error_code();

        if !code.success() {
            let err = res.error_string();
            Err(TaosError {
                code,
                err: Cow::from(err),
            })
        } else {
            Ok(res)
        }
    }

    pub fn affected_rows(&self) -> i32 {
        unsafe { taos_affected_rows(self.res) }
    }

    pub fn fetch_fields(&self) -> TaosQueryData {
        let fields = unsafe { taos_fetch_fields(self.res) };

        let fcount = unsafe { taos_field_count(self.res) };
        let mut rows = Vec::new();
        let fields = (0..fcount)
            .into_iter()
            .map(|i| {
                let field = &unsafe { *fields.offset(i as _) };
                let name = unsafe { CStr::from_ptr(&field.name as _) }
                    .to_string_lossy()
                    .into_owned();
                ColumnMeta {
                    name,
                    type_: field.type_.into(),
                    bytes: field.bytes,
                }
            })
            .collect_vec();

        while let Some(taos_row) = unsafe { taos_fetch_row(self.res).as_ref() } {
            let lengths =
                unsafe { std::slice::from_raw_parts(taos_fetch_lengths(self.res), fcount as _) };
            let row = unsafe { std::slice::from_raw_parts(taos_row, fcount as usize) }
                .iter()
                .zip(fields.iter())
                .zip(lengths.iter())
                .map(|((ptr, meta), length)| unsafe {
                    if ptr.is_null() {
                        return Field::Null;
                    }
                    //let ptr = taos_row.offset(i as _);
                    match meta.type_ {
                        TaosDataType::Null => Field::Null,
                        TaosDataType::Bool => Field::Bool(*(*ptr as *mut i8) != 0),
                        TaosDataType::TinyInt => Field::TinyInt(*(*ptr as *mut i8)),
                        TaosDataType::SmallInt => Field::SmallInt(*(*ptr as *mut i16)),
                        TaosDataType::Int => Field::Int(*(*ptr as *mut i32)),
                        TaosDataType::BigInt => Field::BigInt(*(*ptr as *mut i64)),
                        TaosDataType::UTinyInt => Field::UTinyInt(*(*ptr as *mut u8)),
                        TaosDataType::USmallInt => Field::USmallInt(*(*ptr as *mut u16)),
                        TaosDataType::UInt => Field::UInt(*(*ptr as *mut u32)),
                        TaosDataType::UBigInt => Field::UBigInt(*(*ptr as *mut u64)),
                        TaosDataType::Timestamp => Field::Timestamp(Timestamp::new(
                            *(*ptr as *mut i64),
                            taos_result_precision(self.res),
                        )),
                        TaosDataType::Float => Field::Float(*(*ptr as *mut f32)),
                        TaosDataType::Double => Field::Double(*(*ptr as *mut f64)),
                        TaosDataType::Binary => Field::Binary({
                            std::slice::from_raw_parts((*ptr) as *mut u8, *length as _).into()
                        }),
                        TaosDataType::NChar => {
                            let slice = std::slice::from_raw_parts((*ptr) as *mut u8, *length as _);
                            let s = String::from_utf8_lossy(slice).to_string();
                            Field::NChar(s)
                        }
                        TaosDataType::Json => {
                            let slice = std::slice::from_raw_parts((*ptr) as *mut u8, *length as _);
                            serde_json::from_slice(slice)
                                .ok()
                                .map(Field::Json)
                                .unwrap_or(Field::Null)
                        }
                        _ => {
                            unreachable!("unexpected data type, please contact the author to fix!")
                        }
                    }
                })
                .collect_vec();
            // std::mem::forget(lengths);
            rows.push(row);
        }
        TaosQueryData {
            column_meta: fields,
            rows,
        }
    }
}
impl Drop for CTaosResult {
    fn drop(&mut self) {
        unsafe {
            taos_free_result(self.res);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test::taos;
    use crate::*;
    use proc_test_catalog::test_catalogue;

    #[tokio::test]
    #[test_catalogue]
    /// Test TS-781 Bug for binary
    async fn ts781_binary() -> Result<(), Error> {
        let taos = taos()?;
        let db = "rs_ts781_binary";
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        taos.exec("create stable stb1 (ts timestamp, name binary(10)) tags(n int)")
            .await?;
        taos.exec("insert into tb1 using stb1 tags(1) values(now, 'db02');")
            .await?;
        taos.exec("insert into tb2 using stb1 tags(2) values(now, 'db3');")
            .await?;
        let res = taos.query("select distinct(name) from stb1;").await?;
        assert_eq!(res.rows[0][0], Field::Binary("db02".into()));
        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }

    #[tokio::test]
    #[test_catalogue]
    /// Test TS-781 Bug for nchar
    async fn ts781_nchar() -> Result<(), Error> {
        let taos = taos()?;
        let db = "rs_ts781_nchar";
        println!("test using {}", db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        taos.exec("create stable stb1 (ts timestamp, name nchar(10)) tags(n int)")
            .await?;
        taos.exec("insert into tb1 using stb1 tags(1) values(now, 'db02');")
            .await?;
        taos.exec("insert into tb2 using stb1 tags(2) values(now, 'db3');")
            .await?;
        let res = taos.query("select distinct(name) from stb1;").await?;
        assert_eq!(res.rows[0][0], Field::NChar("db02".into()));
        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }

    #[tokio::test]
    #[test_catalogue]
    /// Test json tag format
    async fn json_tag() -> Result<(), Error> {
        let taos = taos()?;
        let db = "rs_test_json_tag";
        println!("test using {}", db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;

        macro_rules! exec_ok {
            ($sql:expr) => {
                paste::paste! {
                    taos.exec($sql).await?;
                }
            };
        }
        macro_rules! exec_err {
            ($sql:expr) => {
                paste::paste! {
                    assert!(taos.exec($sql).await.is_err());
                }
            };
        }
        macro_rules! query_assert_rows {
            ($sql:expr, $rows:expr) => {
                paste::paste! {
                    let res = taos.query($sql).await?;
                    assert_eq!(res.rows.len(), $rows);
                }
            };
        }
        macro_rules! query_assert_eq_in {
            ($sql:expr, $i:expr, $j:expr, $v:expr) => {
                paste::paste! {
                    let res = taos.query($sql).await?;
                    assert_eq!(res.rows[$i][$j], $v);
                }
            };
        }
        println!("# STEP 1 prepare data & validate json string");
        taos.exec("create table if not exists jsons1(ts timestamp, dataInt int, dataBool bool, dataStr nchar(50), dataStrBin binary(150)) tags(jtag json);").await?;
        taos.exec("insert into jsons1_1 using jsons1 tags('{\"tag1\":\"fff\",\"tag2\":5, \"tag3\":true}') values(1591060618000, 1, false, 'json1', '涛思数据') (1591060608000, 23, true, '涛思数据', 'json')").await?;
        taos.exec("insert into jsons1_2 using jsons1 tags('{\"tag1\":5,\"tag2\":\"beijing\"}') values (1591060628000, 2, true, 'json2', 'sss')").await?;
        taos.exec("insert into jsons1_3 using jsons1 tags('{\"tag1\":false,\"tag2\":\"beijing\"}') values (1591060668000, 3, false, 'json3', 'efwe')").await?;
        taos.exec("insert into jsons1_4 using jsons1 tags('{\"tag1\":null,\"tag2\":\"shanghai\",\"tag3\":\"hello\"}') values (1591060728000, 4, true, 'json4', '323sd')").await?;
        taos.exec("insert into jsons1_5 using jsons1 tags('{\"tag1\":1.232, \"tag2\":null}') values(1591060928000, 1, false, '涛思数据', 'ewe')").await?;
        taos.exec("insert into jsons1_6 using jsons1 tags('{\"tag1\":11,\"tag2\":\"\",\"tag2\":null}') values(1591061628000, 11, false, '涛思数据','')").await?;
        taos.exec("insert into jsons1_7 using jsons1 tags('{\"tag1\":\"涛思数据\",\"tag2\":\"\",\"tag3\":null}') values(1591062628000, 2, NULL, '涛思数据', 'dws')").await?;

        println!("## test duplicate key using the first one. elimate empty key");
        taos.exec("CREATE TABLE if not exists jsons1_8 using jsons1 tags('{\"tag1\":null, \"tag1\":true, \"tag1\":45, \"1tag$\":2, \" \":90}')").await?;
        println!("## test empty json string, save as jtag is NULL");
        taos.exec("insert into jsons1_9 using jsons1 tags('\t') values (1591062328000, 24, NULL, '涛思数据', '2sdw')").await?;
        taos.exec("CREATE TABLE if not exists jsons1_10 using jsons1 tags('')")
            .await?;
        taos.exec("CREATE TABLE if not exists jsons1_11 using jsons1 tags(' ')")
            .await?;
        taos.exec("CREATE TABLE if not exists jsons1_12 using jsons1 tags('{}')")
            .await?;
        taos.exec("CREATE TABLE if not exists jsons1_13 using jsons1 tags('null')")
            .await?;

        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('\"efwewf\"')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('3333')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('33.33')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('false')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('[1,true]')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('{222}')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('{\"fe\"}')");

        println!("## test invalidate json key, key must can be printed assic char=");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('{\"tag1\":[1,true]}')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('{\"tag1\":{}}')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('{\"。loc\":\"fff\"}')");
        exec_err!("CREATE TABLE if not exists jsons1_14 using jsons1 tags('{\"\t\":\"fff\"}')");
        exec_err!(
            "CREATE TABLE if not exists jsons1_14 using jsons1 tags('{\"涛思数据\":\"fff\"}')"
        );

        println!("#  STEP 2 alter table json tag");
        exec_err!("ALTER STABLE jsons1 add tag tag2 nchar(20)");
        exec_err!("ALTER STABLE jsons1 drop tag jtag");
        exec_err!("ALTER TABLE jsons1_1 SET TAG jtag=4");
        exec_ok!(
            "ALTER TABLE jsons1_1 SET TAG jtag='{\"tag1\":\"female\",\"tag2\":35,\"tag3\":true}'"
        );

        println!("#  STEP 3 query table");
        println!("## test error syntax");
        exec_err!("select * from jsons1 where jtag->tag1='beijing'");
        exec_err!("select * from jsons1 where jtag->'location'");
        exec_err!("select * from jsons1 where jtag->''");
        exec_err!("select * from jsons1 where jtag->''=9");
        exec_err!("select -> from jsons1");
        exec_err!("select * from jsons1 where contains");
        exec_err!("select * from jsons1 where jtag->");
        exec_err!("select jtag->location from jsons1");
        exec_err!("select jtag contains location from jsons1");
        exec_err!("select * from jsons1 where jtag contains location");
        exec_err!("select * from jsons1 where jtag contains''");
        exec_err!("select * from jsons1 where jtag contains 'location'='beijing'");

        println!("## test select normal column");
        query_assert_rows!("select dataint from jsons1", 9);

        println!("## test select json tag");
        query_assert_rows!("select * from jsons1", 9);
        query_assert_rows!("select jtag from jsons1", 13);
        query_assert_rows!("select jtag from jsons1 where jtag is null", 5);
        query_assert_rows!("select jtag from jsons1 where jtag is not null", 8);
        query_assert_rows!("select jtag from jsons1_8", 1);
        query_assert_rows!("select jtag from jsons1_1", 1);

        query_assert_eq_in!(
            "select jtag from jsons1_9",
            0,
            0,
            Field::Null // Field::Json(serde_json::Value::Null)
        );
        println!("## test select json tag->'key', value is string");
        query_assert_eq_in!(
            "select jtag->'tag1' from jsons1_1",
            0,
            0,
            Field::Json(serde_json::Value::String("female".into()))
        );
        query_assert_eq_in!(
            "select jtag->'tag2' from jsons1_6",
            0,
            0,
            Field::Json(serde_json::Value::String("".into()))
        );
        println!("### test select json tag->'key', value is int");
        query_assert_eq_in!(
            "select jtag->'tag2' from jsons1_1",
            0,
            0,
            Field::Json(serde_json::Value::Number(35i64.into()))
        );
        println!("### test select json tag->'key', value is bool");
        query_assert_eq_in!(
            "select jtag->'tag3' from jsons1_1",
            0,
            0,
            Field::Json(serde_json::Value::Bool(true))
        );

        println!("### test select json tag->'key', value is null");
        query_assert_eq_in!(
            "select jtag->'tag1' from jsons1_4",
            0,
            0,
            Field::Json(serde_json::Value::Null)
        );
        println!("### test select json tag->'key', value is double");
        query_assert_eq_in!(
            "select jtag->'tag1' from jsons1_5",
            0,
            0,
            Field::Json(serde_json::Value::Number(
                serde_json::value::Number::from_f64(1.232000000_f64).unwrap()
            ))
        );

        println!("### test select json tag->'key', key is not exist");
        query_assert_eq_in!("select jtag->'tag10' from jsons1_4", 0, 0, Field::Null);

        println!("## test where with json tag");
        exec_err!("select * from jsons1_1 where jtag is not null");
        exec_err!("select * from jsons1 where jtag='{\"tag1\":11,\"tag2\":\"\"}'");
        exec_err!("select * from jsons1 where jtag->'tag1'={}");

        println!("### where json value is string");
        query_assert_rows!("select * from jsons1 where jtag->'tag2'='beijing'", 2);
        query_assert_rows!(
            "select dataint,tbname,jtag->'tag1',jtag from jsons1 where jtag->'tag2'='beijing'",
            2
        );
        query_assert_rows!("select * from jsons1 where jtag->'tag1'='beijing'", 0);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'='涛思数据'", 1);
        query_assert_rows!("select * from jsons1 where jtag->'tag2'>'beijing'", 1);
        query_assert_rows!("select * from jsons1 where jtag->'tag2'>='beijing'", 3);
        query_assert_rows!("select * from jsons1 where jtag->'tag2'<'beijing'", 2);
        query_assert_rows!("select * from jsons1 where jtag->'tag2'<='beijing'", 4);
        query_assert_rows!("select * from jsons1 where jtag->'tag2'!='beijing'", 3);
        query_assert_rows!("select * from jsons1 where jtag->'tag2'=''", 2);

        println!("### where json value is int");
        query_assert_rows!("select * from jsons1 where jtag->'tag1'=5", 1);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'=10", 0);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'<54", 3);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'<=11", 3);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'>4", 2);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'>=5", 2);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'!=5", 2);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'!=55", 3);

        println!("### where json value is double");
        query_assert_rows!("select * from jsons1 where jtag->'tag1'=1.232", 1);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'<1.232", 0);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'<=1.232", 1);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'>1.23", 3);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'>=1.232", 3);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'!=1.232", 2);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'!=3.232", 3);
        exec_err!("select * from jsons1 where jtag->'tag1'/0=3");
        exec_err!("select * from jsons1 where jtag->'tag1'/5=1");

        println!("### where json value is bool");
        query_assert_rows!("select * from jsons1 where jtag->'tag1'=true", 0);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'=false", 1);
        query_assert_rows!("select * from jsons1 where jtag->'tag1'!=false", 0);
        exec_err!("select * from jsons1 where jtag->'tag1'>false");

        println!("### where json value is null");
        query_assert_rows!("select * from jsons1 where jtag->'tag1'=null", 1);

        println!("### where json is null");
        query_assert_rows!("select * from jsons1 where jtag is null", 1);
        query_assert_rows!("select * from jsons1 where jtag is not null", 8);

        println!("### where json key is null");
        query_assert_rows!("select * from jsons1 where jtag->'tag_no_exist'=3", 0);

        println!("### where json value is not exist");
        query_assert_rows!("select * from jsons1 where jtag->'tag1' is null", 1);
        query_assert_rows!("select * from jsons1 where jtag->'tag4' is null", 9);
        query_assert_rows!("select * from jsons1 where jtag->'tag3' is not null", 4);

        println!("### test contains");
        query_assert_rows!("select * from jsons1 where jtag contains 'tag1'", 8);
        query_assert_rows!("select * from jsons1 where jtag contains 'tag3'", 4);
        query_assert_rows!("select * from jsons1 where jtag contains 'tag_no_exist'", 0);

        println!("### test json tag in where condition with and/or");
        query_assert_rows!(
            "select * from jsons1 where jtag->'tag1'=false and jtag->'tag2'='beijing'",
            1
        );
        query_assert_rows!(
            "select * from jsons1 where jtag->'tag1'=false or jtag->'tag2'='beijing'",
            2
        );
        query_assert_rows!(
            "select * from jsons1 where jtag->'tag1'=false and jtag->'tag2'='shanghai'",
            0
        );
        query_assert_rows!(
            "select * from jsons1 where jtag->'tag1'=13 or jtag->'tag2'>35",
            0
        );
        query_assert_rows!(
            "select * from jsons1 where jtag->'tag1' is not null and jtag contains 'tag3'",
            4
        );
        query_assert_rows!(
            "select * from jsons1 where jtag->'tag1'='female' and jtag contains 'tag3'",
            2
        );

        println!("### test with tbname/normal column");
        query_assert_rows!("select * from jsons1 where tbname = 'jsons1_1'", 2);
        query_assert_rows!(
            "select * from jsons1 where tbname = 'jsons1_1' and jtag contains 'tag3'",
            2
        );
        query_assert_rows!(
            "select * from jsons1 where tbname = 'jsons1_1' and jtag contains 'tag3' and dataint=3",
            0
        );
        query_assert_rows!("select * from jsons1 where tbname = 'jsons1_1' and jtag contains 'tag3' and dataint=23", 1);

        println!("### test where condition like");
        query_assert_rows!(
            "select *,tbname from jsons1 where jtag->'tag2' like 'bei%'",
            2
        );
        query_assert_rows!("select *,tbname from jsons1 where jtag->'tag1' like 'fe%' and jtag->'tag2' is not null", 2);

        println!("### test where condition in  no support in");
        exec_err!("select * from jsons1 where jtag->'tag1' in ('beijing')");

        println!("### test where condition match");
        query_assert_rows!("select * from jsons1 where jtag->'tag1' match 'ma'", 2);
        query_assert_rows!("select * from jsons1 where jtag->'tag1' match 'ma$'", 0);
        query_assert_rows!("select * from jsons1 where jtag->'tag2' match 'jing$'", 2);
        query_assert_rows!("select * from jsons1 where jtag->'tag1' match '收到'", 0);

        println!("### test distinct");
        exec_ok!("insert into jsons1_14 using jsons1 tags('{\"tag1\":\"涛思数据\",\"tag2\":\"\",\"tag3\":null}') values(1591062628000, 2, NULL, '涛思数据', 'dws')");
        query_assert_rows!("select distinct jtag->'tag1' from jsons1", 8);
        query_assert_rows!("select distinct jtag from jsons1", 9);

        println!("### test dumplicate key with normal colomn");
        exec_ok!("INSERT INTO jsons1_15 using jsons1 tags('{\"tbname\":\"tt\",\"databool\":true,\"datastr\":\"涛思数据\"}') values(1591060828000, 4, false, 'jjsf', \"涛思数据\")");
        query_assert_rows!("select *,tbname,jtag from jsons1 where jtag->'datastr' match '涛思' and datastr match 'js'", 1);
        query_assert_rows!("select tbname,jtag->'tbname' from jsons1 where jtag->'tbname'='tt' and tbname='jsons1_14'", 0);

        println!("## test join");
        exec_ok!("create table if not exists jsons2(ts timestamp, dataInt int, dataBool bool, dataStr nchar(50), dataStrBin binary(150)) tags(jtag json)");
        exec_ok!("insert into jsons2_1 using jsons2 tags('{\"tag1\":\"fff\",\"tag2\":5, \"tag3\":true}') values(1591060618000, 2, false, 'json2', '你是2')");
        exec_ok!("insert into jsons2_2 using jsons2 tags('{\"tag1\":5,\"tag2\":null}') values (1591060628000, 2, true, 'json2', 'sss')");
        exec_ok!("create table if not exists jsons3(ts timestamp, dataInt int, dataBool bool, dataStr nchar(50), dataStrBin binary(150)) tags(jtag json)");
        exec_ok!("insert into jsons3_1 using jsons3 tags('{\"tag1\":\"fff\",\"tag2\":5, \"tag3\":true}') values(1591060618000, 3, false, 'json3', '你是3')");
        exec_ok!("insert into jsons3_2 using jsons3 tags('{\"tag1\":5,\"tag2\":\"beijing\"}') values (1591060638000, 2, true, 'json3', 'sss')");

        query_assert_rows!("select 'sss',33,a.jtag->'tag3' from jsons2 a,jsons3 b where a.ts=b.ts and a.jtag->'tag1'=b.jtag->'tag1'", 1);
        query_assert_rows!("select 'sss',33,a.jtag->'tag3' from jsons2 a,jsons3 b where a.ts=b.ts and a.jtag->'tag1'=b.jtag->'tag1'", 1);

        println!("## test group by & order by  json tag");

        query_assert_rows!(
            "select count(*) from jsons1 group by jtag->'tag1' order by jtag->'tag1' desc",
            8
        );
        query_assert_eq_in!(
            "select count(*) from jsons1 group by jtag->'tag1' order by jtag->'tag1' desc",
            7,
            1,
            Field::Null
        );
        query_assert_rows!(
            "select count(*) from jsons1 group by jtag->'tag1' order by jtag->'tag1' asc",
            8
        );
        query_assert_eq_in!(
            "select count(*) from jsons1 group by jtag->'tag1' order by jtag->'tag1' asc",
            0,
            1,
            Field::Null
        );

        println!("## test stddev with group by json tag");
        query_assert_rows!(
            "select stddev(dataint) from jsons1 group by jtag->'tag1'",
            8
        );
        query_assert_rows!(
            "select stddev(dataint) from jsons1 group by jsons1.jtag->'tag1'",
            8
        );

        println!("## test top/bottom with group by json tag");
        query_assert_rows!(
            "select top(dataint,100) from jsons1 group by jtag->'tag1'",
            11
        );

        println!("## subquery with json tag");
        query_assert_rows!("select * from (select jtag, dataint from jsons1)", 11);
        query_assert_rows!(
            "select jtag->'tag1' from (select jtag->'tag1', dataint from jsons1)",
            11
        );
        query_assert_rows!("select ts,tbname,jtag->'tag1' from (select jtag->'tag1',tbname,ts from jsons1 order by ts)", 11);

        exec_ok!(format!("drop database {}", db));
        Ok(())
    }
}
