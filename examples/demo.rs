use libtaos::*;

use std::env::var;

use itertools::Itertools;

pub fn init() {
    env_logger::init();
}

fn var_or_default(env: &str, default: &str) -> String {
    var(env).unwrap_or(default.to_string())
}
fn taos_connect() -> Result<Taos, Error> {
    TaosCfgBuilder::default()
        .ip(&var_or_default("TEST_TAOS_IP", "127.0.0.1"))
        .user(&var_or_default("TEST_TAOS_USER", "root"))
        .pass(&var_or_default("TEST_TAOS_PASS", "taosdata"))
        // .db(&var_or_default("TEST_TAOS_DB", "log"))
        .port(
            var_or_default("TEST_TAOS_PORT", "6030")
                .parse::<u16>()
                .unwrap(),
        )
        .build()
        .expect("TaosCfg builder error")
        .connect()
}

#[tokio::test]
async fn cc() -> Result<(), Error> {
    let cfg: TaosCfg = TaosCfgBuilder::default()
        .ip("127.0.0.1")
        .user("root")
        .pass("taosdata")
        .db("log") // do not set if not require a default database.
        .port(6030u16)
        .build()
        .expect("TaosCfg builder error");
    let pool = r2d2::Pool::builder()
        .max_size(10000) // 最大连接数
        .build(cfg)
        .unwrap();
    let conn = pool.get().unwrap();
    // let conn = cfg.connect().unwrap();
    conn.exec("create database if not exists demo").await?;
    conn.exec("use demo").await?;
    conn.exec("create table if not exists tb1 (ts timestamp, v int)")
        .await?;
    conn.exec("insert into tb1 values(now, 1)").await?;

    let rows = conn.query("select * from tb1").await?;
    for row in rows.rows {
        println!("{}", row.into_iter().join(","));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init();
    let taos = taos_connect()?;

    assert_eq!(
        taos.query("drop database if exists demo").await.is_ok(),
        true
    );
    assert_eq!(taos.query("create database demo").await.is_ok(), true);
    assert_eq!(
        taos.query("create table demo.m1 (ts timestamp, speed int unsigned)")
            .await
            .is_ok(),
        true
    );

    taos.query(format!("insert into demo.m1 values (now-1s, NULL)").as_str())
        .await?;
    for i in 0..10i32 {
        assert_eq!(
            taos.query(format!("insert into demo.m1 values (now+{}s, {})", i, i).as_str())
                .await
                .is_ok(),
            true
        );
    }
    let rows = taos.query("select * from demo.m1").await?;

    println!(
        "{}",
        rows.column_meta.into_iter().map(|col| col.name).join(",")
    );
    // TODO: add Iterator for TaosQueryData
    for row in rows.rows {
        println!("{}", row.into_iter().join(","));
    }
    Ok(())
}
