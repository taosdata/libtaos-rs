use crate::*;

pub fn var_or_default(env: &str, default: &str) -> String {
    std::env::var(env).unwrap_or(default.to_string())
}

pub fn taos() -> Result<Taos, Error> {
    TaosCfgBuilder::default()
        .ip(var_or_default("TEST_TAOS_IP", "127.0.0.1"))
        .user(var_or_default("TEST_TAOS_USER", "root"))
        .pass(var_or_default("TEST_TAOS_PASS", "taosdata"))
        .db(var_or_default("TEST_TAOS_DB", "log"))
        .port(
            var_or_default("TEST_TAOS_PORT", "6030")
                .parse::<u16>()
                .unwrap(),
        )
        .build()
        .expect("ToasCfg builder error")
        .connect()
}

#[test]
fn test_builder() {
    let _ = TaosCfgBuilder::default()
        .build()
        .expect("cfg builder error")
        .connect()
        .expect("connect with default");
}

#[test]
fn test_builder2() {
    let _ = taos().unwrap();
}
