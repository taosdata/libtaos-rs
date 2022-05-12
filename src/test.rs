use crate::*;

pub fn var_or_default(env: &str, default: &str) -> String {
    std::env::var(env).unwrap_or(default.to_string())
}

pub fn taos() -> Result<Taos, Error> {
    TaosCfgBuilder::default()
        .ip(var_or_default("TEST_TAOS_IP", "127.0.0.1"))
        .user(var_or_default("TEST_TAOS_USER", "root"))
        .pass(var_or_default("TEST_TAOS_PASS", "taosdata"))
        .db(var_or_default("TEST_TAOS_DB", ""))
        .port(
            var_or_default("TEST_TAOS_PORT", "6030")
                .parse::<u16>()
                .unwrap(),
        )
        .token(std::env::var("TEST_TAOS_TOKEN").ok())
        .build()
        .expect("TaosCfg builder error")
        .connect()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_builder() {
    let _ = TaosCfgBuilder::default()
        .build()
        .expect("cfg builder error")
        .connect()
        .expect("connect with default");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_builder2() {
    let _ = taos().unwrap();
}
