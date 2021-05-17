use crate::*;

#[test]
fn should_pass_the_as_ref_through() {
    let masked = "".to_owned().masked();
    let _as_str_ref: &str = masked.as_ref();
    let _as_string_ref: &String = masked.as_ref();

    struct Config;

    let masked = Config.masked();
    let _config_ref: &Config = masked.as_ref();

    let masked = std::sync::Arc::new(Config).masked();
    let _config_ref: &Config = masked.as_ref();
}
