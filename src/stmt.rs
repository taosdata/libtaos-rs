use crate::bindings::*;
use crate::*;

use std::ffi::CStr;
use std::os::raw::c_void;

mod bind;
pub use bind::{BindParam, IntoBindParam};

pub trait IntoParams {
    fn into_params(self) -> Vec<BindParam>;
}

impl<F: IntoBindParam, T: IntoIterator<Item = F>> IntoParams for T {
    fn into_params(self) -> Vec<BindParam> {
        self.into_iter().map(|v| v.into_bind_param()).collect()
    }
}

pub struct Stmt {
    taos: *mut c_void,
    stmt: *mut c_void,
}

impl Stmt {
    fn err_or(&self, res: i32) -> Result<(), TaosError> {
        if res != 0 {
            let code: TaosCode = (res & 0x0000ffff).into();
            let err = unsafe { taos_stmt_errstr(self.stmt) };
            if !err.is_null() {
                let err = unsafe { CStr::from_ptr(err as _) }
                    .to_string_lossy()
                    .to_owned();
                trace!("stmt error: {:?}", err);
                return Err(TaosError { code, err });
            }
            return Err(TaosError {
                code,
                err: "unknown".into(),
            });
        }
        Ok(())
    }
    /// NOT a public method
    fn prepare(&mut self, sql: impl ToCString) -> Result<(), TaosError> {
        let res = unsafe { taos_stmt_prepare(self.stmt, sql.to_c_string().as_ptr(), 0) };
        self.err_or(res)
    }
    pub fn execute(&self) -> Result<(), TaosError> {
        unsafe {
            let res = taos_stmt_execute(self.stmt);
            let res = self.err_or(res);

            if let Err(TaosError { code, err }) = res {
                if code == TaosCode::RpcFqdnError {
                    let res = taos_query(self.taos, b"select server_version()\0" as *const u8 as _);
                    let errno = taos_errno(res);
                    if errno == 0 {
                        return Ok(());
                    } else {
                        Err(TaosError { code, err })
                    }
                } else {
                    Err(TaosError { code, err })
                }
            } else {
                Ok(())
            }
        }
    }

    /// To bind one row with params
    pub fn bind(&mut self, params: impl IntoParams) -> Result<(), TaosError> {
        let params = params.into_params();
        //assert_eq!(self.num_params(), params.len());
        unsafe {
            let res = taos_stmt_bind_param(self.stmt, params.as_ptr() as _);
            self.err_or(res)?;
            let res = taos_stmt_add_batch(self.stmt);
            self.err_or(res)?;
        }
        Ok(())
    }

    /// Bind params for one record.
    pub fn bind_inplace(&mut self, params: &[BindParam]) -> Result<(), TaosError> {
        unsafe {
            let res = taos_stmt_bind_param(self.stmt, params.as_ptr() as _);
            self.err_or(res)?;
            let res = taos_stmt_add_batch(self.stmt);
            self.err_or(res)
        }
    }

    pub fn bind_batch_at_col<T>(&mut self, _params: T) -> Result<(), TaosError>
    where
        T: IntoIterator,
        T::Item: IntoBindParam,
    {
        Ok(())
    }

    pub fn num_params(&self) -> usize {
        unsafe {
            let mut num = 0;
            taos_stmt_num_params(self.stmt, &mut num as _);
            num as _
        }
    }
    pub fn set_tbname_tags(
        &mut self,
        tbname: impl ToCString,
        tags: impl IntoParams,
    ) -> Result<(), TaosError> {
        let tags = tags.into_params();
        unsafe {
            let res = taos_stmt_set_tbname_tags(
                self.stmt,
                tbname.to_c_string().as_ptr(),
                tags.as_ptr() as _,
            );
            self.err_or(res)
        }
    }
    pub fn set_tbname(&mut self, tbname: impl ToCString) -> Result<(), TaosError> {
        unsafe {
            let res = taos_stmt_set_tbname(self.stmt, tbname.to_c_string().as_ptr());
            self.err_or(res)
        }
    }
    pub fn set_sub_tbname(&mut self, tbname: impl ToCString) -> Result<(), TaosError> {
        unsafe {
            let res = taos_stmt_set_sub_tbname(self.stmt, tbname.to_c_string().as_ptr());
            self.err_or(res)
        }
    }
    pub fn is_insert(&self) -> bool {
        unsafe {
            let mut res = 0;
            taos_stmt_is_insert(self.stmt, &mut res as _);
            res != 0
        }
    }
    fn close(&mut self) {
        unsafe {
            taos_stmt_close(self.stmt);
        }
    }
}

impl Drop for Stmt {
    fn drop(&mut self) {
        self.close();
    }
}

impl Taos {
    /// Create stmt with sql
    pub fn stmt(&self, sql: impl ToCString) -> Result<Stmt, TaosError> {
        let sql = sql.to_c_string();
        unsafe {
            let stmt = taos_stmt_init(self.as_raw());
            // let res = taos_stmt_prepare(stmt, sql.as_ptr(), 0);
            let mut stmt = Stmt {
                taos: self.as_raw(),
                stmt,
            };
            stmt.prepare(sql)?;
            Ok(stmt)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test::taos;
    use crate::*;
    use proc_test_catalog::test_catalogue;

    async fn stmt_test(db: &str, ty: &str, value: Field) -> Result<(), Error> {
        let taos = taos()?;
        println!("test {} using {}", ty, db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        taos.exec(format!(
            "create table if not exists tb0 (ts timestamp, n {})",
            ty
        ))
        .await?;
        let mut stmt = taos.stmt("insert into ? values(?,?)")?;
        stmt.set_tbname("tb0")?;
        assert!(stmt.is_insert());
        assert_eq!(stmt.num_params(), 2);
        let ts = Field::Timestamp(Timestamp::now());
        stmt.bind(vec![ts, value.clone()].iter())?;
        let _ = stmt.execute()?;
        let res = taos.query("select n from tb0").await?;
        assert_eq!(value, res.rows[0][0]);
        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }

    macro_rules! _test_column_null {
        ($ty:ty, $v:expr) => {
            paste::paste! {
                #[tokio::test]
                #[test_catalogue()]
                #[doc = "Test bind null to type " $ty]
                async fn [<null_ $ty:snake>]() -> Result<(), Error> {
                    let db = stdext::function_name!()
                        .replace("::{{closure}}", "")
                        .replace("::", "_");
                    stmt_test(&db, $v, Field::Null).await
                }
            }
        };
    }
    _test_column_null!(bool, "bool");
    _test_column_null!(tinyint, "tinyint");
    _test_column_null!(smallint, "smallint");
    _test_column_null!(int, "int");
    _test_column_null!(bigint, "bigint");
    _test_column_null!(utinyint, "tinyint unsigned");
    _test_column_null!(usmallint, "smallint unsigned");
    _test_column_null!(uint, "int unsigned");
    _test_column_null!(ubigint, "bigint unsigned");
    _test_column_null!(timestamp, "timestamp");
    _test_column_null!(float, "float");
    _test_column_null!(double, "double");
    _test_column_null!(binary, "binary(10)");
    _test_column_null!(nchar, "nchar(10)");
    #[should_panic]
    #[tokio::test]
    #[test_catalogue()]
    /// Test bind null to json tag, should panic because json is not supported in cols
    async fn null_json() -> () {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "json", Field::Null).await.unwrap()
    }

    async fn stmt_tag(db: &str, ty: &str, tag: Field) -> Result<(), Error> {
        let taos = taos()?;
        println!("test {} using {}", ty, db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        taos.exec(format!(
            "create table if not exists stb0 (ts timestamp, v int) tags (n {})",
            ty
        ))
        .await?;
        println!("start stmt");
        let mut stmt = taos.stmt("insert into ? using stb0 tags(?) values(?,?)")?;
        println!("set tags");
        stmt.set_tbname_tags("tb0", [&tag])?;
        assert!(stmt.is_insert());
        assert_eq!(stmt.num_params(), 2);
        let ts = Field::Timestamp(Timestamp::now());
        println!("bind stmt");
        stmt.bind(vec![ts, Field::Null].iter())?;
        println!("execute");
        stmt.execute()?;
        println!("execute stmt done");
        let res = taos.query("select n from tb0").await?;
        assert_eq!(tag, res.rows[0][0]);
        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }
    macro_rules! _test_tag_null {
        ($ty:ty, $v:expr) => {
            paste::paste! {
                #[tokio::test]
                #[test_catalogue()]
                #[doc = "Test bind null to type " $ty]
                async fn [<null_tag_ $ty:snake>]() -> Result<(), Error> {
                    let db = stdext::function_name!()
                        .replace("::{{closure}}", "")
                        .replace("::", "_")
                        .replace("libtaos_", "");
                    stmt_tag(&db, $v, Field::Null).await
                }
            }
        };
    }
    _test_tag_null!(bool, "bool");
    _test_tag_null!(tinyint, "tinyint");
    _test_tag_null!(smallint, "smallint");
    _test_tag_null!(int, "int");
    _test_tag_null!(bigint, "bigint");
    _test_tag_null!(utinyint, "tinyint unsigned");
    _test_tag_null!(usmallint, "smallint unsigned");
    _test_tag_null!(uint, "int unsigned");
    _test_tag_null!(ubigint, "bigint unsigned");
    _test_tag_null!(timestamp, "timestamp");
    _test_tag_null!(float, "float");
    _test_tag_null!(double, "double");
    _test_tag_null!(binary, "binary(10)");
    _test_tag_null!(nchar, "nchar(10)");
    // set null in json tag is currently abort taosd. see TD-12452.
    // TD-12452 is fixed by https://github.com/taosdata/TDengine/pull/9317
    _test_tag_null!(json, "json");
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with bool values.
    async fn bool() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "bool", Field::Bool(true)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with tiny int values.
    async fn tinyint() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "tinyint", Field::TinyInt(-0x7f)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with small int values.
    async fn smallint() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "smallint", Field::SmallInt(0x7fff)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with int values.
    async fn int() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "int", Field::Int(0x7fffffff)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with bigint values.
    async fn bigint() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "bigint", Field::BigInt(0x7fffffff_ffffffff)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with unsigned tinyint values.
    async fn utinyint() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "tinyint unsigned", Field::UTinyInt(0)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with unsigned smallint values.
    async fn usmallint() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "smallint unsigned", Field::USmallInt(1)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with unsigned int values.
    async fn uint() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "int unsigned", Field::UInt(1)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with unsigned bigint values.
    async fn ubigint() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "bigint unsigned", Field::UBigInt(1)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with float values.
    async fn float() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "float", Field::Float(1.0)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with double values.
    async fn double() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        stmt_test(&db, "double", Field::Double(1.0)).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with binary values.
    async fn binary() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        let v = Field::Binary("0123456789".into());
        stmt_test(&db, "binary(10)", v).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with nchar(unicode) values.
    async fn nchar() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        let v = Field::NChar("一二三四五六七八九十".into());
        stmt_test(&db, "nchar(10)", v).await
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with json values.
    async fn json() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        let v = Field::Json(serde_json::from_str("{\"tag1\":\"一二三四五六七八九十\"}").unwrap());

        let taos = taos()?;
        println!("test json using {}", db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        taos.exec("create stable if not exists stb0 (ts timestamp, n int) tags(j json)")
            .await?;
        let mut stmt = taos.stmt("insert into ? using stb0 tags(?) values(?,?)")?;
        println!("set tbname with tags");
        stmt.set_tbname_tags("tb0", [&v])?;
        println!("bind values");
        assert!(stmt.is_insert());
        assert_eq!(stmt.num_params(), 2);
        let ts = Field::Timestamp(Timestamp::now());
        stmt.bind(vec![ts, Field::Int(3)].iter())?;
        let _ = stmt.execute()?;
        let res = taos.query("select j from stb0").await?;
        let row = res.rows.iter().next().unwrap();
        assert_eq!(&v, row.iter().next().unwrap());
        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }

    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT inserting with all types of values.
    async fn all_types() -> Result<(), Error> {
        let taos = taos()?;
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        println!("{}", db);
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        taos.exec(
            "create table if not exists tb0 (ts timestamp,
             c1 tinyint, c2 smallint, c3 int, c4 bigint,
             c5 tinyint unsigned, c6 smallint unsigned, c7 int unsigned, c8 bigint unsigned,
             c9 float, c10 double, c11 binary(10), c12 nchar(10))",
        )
        .await?;
        let mut stmt = taos.stmt("insert into ? values(?,?,?,?,?,?,?,?,?,?,?,?,?)")?;
        stmt.set_tbname("tb0")?;
        assert!(stmt.is_insert());

        assert_eq!(stmt.num_params(), 13);
        let ts = Field::Timestamp(Timestamp::now());
        let c1 = Field::TinyInt(1);
        let c2 = Field::SmallInt(2);
        let c3 = Field::Int(3);
        let c4 = Field::BigInt(4);
        let c5 = Field::UTinyInt(5);
        let c6 = Field::USmallInt(6);
        let c7 = Field::UInt(7);
        let c8 = Field::UBigInt(8);
        let c9 = Field::Float(9.0);
        let c10 = Field::Double(9.0);
        let c11 = Field::Binary("binary".into());
        let c12 = Field::NChar("nchar".into());
        stmt.bind(vec![ts, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12].iter())?;
        let _ = stmt.execute()?;
        let res = taos.query("select count(*) as count from tb0").await?;
        println!("{:?}", res);
        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT set tbname with upper-case stable, see jira TD-12977
    async fn test_uppercase_tbname() -> Result<(), Error> {
        let db = "uppercase_test";
        let taos = taos()?;
        taos.exec(format!("drop database if exists {db}")).await?;
        taos.exec(format!("create database {db}")).await?;
        taos.exec(format!("use {db}")).await?;
        taos.exec(format!(
            "create stable STB(ts timestamp, n int) tags(b int)"
        ))
        .await?;
        let mut stmt = taos.stmt("insert into ? using STB tags(?) values(?, ?)")?;

        stmt.set_tbname_tags("tb0", [0i32])?;
        // stmt.bind(&[0i32])?;
        let values = vec![Field::Timestamp(Timestamp::now()), Field::Int(10)];
        stmt.bind(&values)?;

        assert!(stmt.is_insert());
        assert_eq!(stmt.num_params(), 2);

        let _ = stmt.execute()?;
        const LIMIT: i64 = 100;

        for i in 1..LIMIT {
            stmt.set_tbname_tags(format!("tb{}", i), &[2i32])?;
            stmt.bind(&values)?;
        }
        let _ = stmt.execute()?;
        let res = taos.query("select count(*) as count from stb").await?;
        assert_eq!(res.rows[0][0], Field::BigInt(LIMIT));
        taos.exec(format!("drop database {}", db)).await?;

        Ok(())
    }
    #[tokio::test]
    #[test_catalogue()]
    /// Test STMT API insertion with tags
    async fn test_stmt_tags() -> Result<(), Error> {
        let db = stdext::function_name!()
            .replace("::{{closure}}", "")
            .replace("::", "_");
        println!("{:?}", db);
        let taos = taos()?;
        taos.exec(format!("drop database if exists {}", db)).await?;
        taos.exec(format!("create database if not exists {} keep 36500", db))
            .await?;
        taos.exec(format!("use {}", db)).await?;
        taos.exec("create table if not exists stb (ts timestamp, n int) tags(b int)")
            .await?;

        let mut stmt = taos.stmt("insert into ? using stb tags(?) values(?, ?)")?;

        stmt.set_tbname_tags("tb0", [0i32])?;
        // stmt.bind(&[0i32])?;
        let values = vec![Field::Timestamp(Timestamp::now()), Field::Int(10)];
        stmt.bind(&values)?;

        assert!(stmt.is_insert());
        assert_eq!(stmt.num_params(), 2);

        let _ = stmt.execute()?;
        const LIMIT: i64 = 100;

        for i in 1..LIMIT {
            stmt.set_tbname_tags(format!("tb{}", i), &[2i32])?;
            stmt.bind(&values)?;
        }
        let _ = stmt.execute()?;
        let res = taos.query("select count(*) as count from stb").await?;
        assert_eq!(res.rows[0][0], Field::BigInt(LIMIT));
        taos.exec(format!("drop database {}", db)).await?;
        Ok(())
    }
}
