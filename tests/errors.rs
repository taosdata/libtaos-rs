mod init;

use crate::TaosCode;
use libtaos::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn invalid_database_name() -> () {
    let taos = init::taos().unwrap();
    let res = taos
        .query("insert into a_long_in_valid_database_name.table1 values(0, 1)")
        .await;
    assert!(res.is_err());

    let err = res.unwrap_err();
    match err {
        Error::RawTaosError(TaosError { code, err }) => {
            println!("{}", err);
            assert_eq!(code, TaosCode::MndDbNotSelected);
        }
        _ => {
            unreachable!();
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn invalid_table_name() -> () {
    let taos = init::taos().unwrap();
    let res = taos
        .query("insert into log.a_long_in_valid_database_name values(0, 1)")
        .await;
    assert!(res.is_err());

    let err = res.unwrap_err();
    match err {
        Error::RawTaosError(TaosError { code, err }) => {
            println!("{}", err);
            assert!(code.mnd_invalid_table_name());
        }
        _ => {
            unreachable!();
        }
    }
}
