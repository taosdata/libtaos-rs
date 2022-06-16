mod init;

use libtaos::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn double_quote() -> Result<(), Error> {
    //init::init();
    let taos = init::taos().unwrap();
    let _ = taos
        .exec("create database if not exists test_rust_double_quote_tag")
        .await
        .unwrap();
    let _ = taos.use_database("test_rust_double_quote_tag").await?;
    let _ = taos.exec("drop stable if exists stb1").await?;
    let _ = taos
        .exec("create stable if not exists stb1 (ts timestamp, t double) tags (tag1 binary(100))")
        .await?;
    let _ = taos
        .exec("create table tb1 using stb1 tags(\"abc\\\"def\")")
        .await?;
    let _ = taos.exec("insert into tb1 values(now, 1.0)").await;
    let res = taos.query("select * from stb1").await?;
    dbg!(&res);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn query() -> Result<(), Error> {
    //init::init();
    let taos = init::taos().unwrap();
    let q = taos.query("select * from log.logs limit 10").await?;

    let cols = &q.column_meta;
    for col in cols {
        println!(
            "name: {}, type: {:?}, bytes: {}",
            col.name, col.type_, col.bytes
        );
    }
    for (i, row) in q.rows.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            println!("cell({}, {}) data: {}", i, j, cell);
        }
    }
    Ok(())
}
