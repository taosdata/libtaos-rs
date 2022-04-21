mod init;
use libtaos::schemaless::*;
use libtaos::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn sml() -> Result<(), Error> {
    init::init();
    let taos = init::taos().unwrap();

    let db = "rs_test_line";
    println!("test using {}", db);
    taos.exec(format!("drop database if exists {}", db)).await?;
    taos.exec(format!("create database if not exists {} keep 36500", db))
        .await?;
    taos.exec(format!("use {}", db)).await?;

    let lines = [
        "st,t1=abc,t2=def,t3=anything c1=3i64,c3=L\"pass\",c2=false,c4=4f64 1626006833639000000",
    ];
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
