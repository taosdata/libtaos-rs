mod init;

use libtaos::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn describe_non_exist_table() -> () {
    init::init();
    let taos = init::taos().unwrap();
    let res = taos.describe("log.a_long_in_valid_database_name").await;
    assert!(res.is_err());
    dbg!(&res);

    let err = res.unwrap_err();
    match err {
        Error::RawTaosError(TaosError { code, err }) => {
            println!("{}", err);
            assert_eq!(code, TaosCode::MndInvalidTableName);
        }

        _ => {
            unreachable!();
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn describe() -> () {
    let taos = init::taos().unwrap();
    let res = taos.describe("log.log").await;
    assert!(res.is_ok());
    let res = res.unwrap();
    let _ = dbg!(res.names());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn query() -> Result<(), Error> {
    let taos = init::taos()?;
    let _res = taos.query("select * from log.log limit 10").await?;
    // assert!(res.is_ok());
    Ok(())
}
