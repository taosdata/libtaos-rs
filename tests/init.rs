use libtaos::*;
use std::env::var;

#[cfg(test)]
#[allow(dead_code)]
pub fn init() {
    std::env::set_var("RUST_LOG", "libtaos=trace");
    env_logger::init();
}

#[allow(dead_code)]
pub fn var_or_default(env: &str, default: &str) -> String {
    var(env).unwrap_or(default.to_string())
}

#[allow(dead_code)]
pub fn taos() -> Result<Taos, Error> {
    TaosCfgBuilder::default()
        .ip(&var_or_default("TEST_TAOS_IP", "127.0.0.1"))
        .user(&var_or_default("TEST_TAOS_USER", "root"))
        .pass(&var_or_default("TEST_TAOS_PASS", "taosdata"))
        .db(&var_or_default("TEST_TAOS_DB", "log"))
        .port(
            var_or_default("TEST_TAOS_PORT", "6030")
                .parse::<u16>()
                .unwrap(),
        )
        .token(var("TEST_TAOS_TOKEN").ok())
        .build()
        .expect("TaosCfg builder error")
        .connect()
}
