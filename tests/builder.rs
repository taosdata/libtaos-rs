#[cfg(all(feature = "r2d2", not(feature = "rest")))]
// #[test()]
#[tokio::test(flavor = "current_thread")]
async fn builder() {
    use libtaos::*;
    use stdext::prelude::DurationExt;
    let cfg = TaosCfgBuilder::default().build().unwrap();

    use std::time::Duration;
    let _pool = r2d2::Pool::builder()
        // .max_size(5000) // 最大连接数
        .max_lifetime(Some(Duration::from_minutes(100))) // 每个连接的最长存活时间
        // .min_idle(Some(1)) // 最小空闲连接数
        .connection_timeout(Duration::from_minutes(2)) // 连接超时时间
        .build(cfg)
        .expect("");
    let taos = _pool.get().expect("");
    taos.exec("select server_version()").await.unwrap();

}
