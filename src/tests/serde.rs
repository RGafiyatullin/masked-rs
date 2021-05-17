use crate::*;

#[test]
fn should_serde() {
    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
    struct Config {
        host: String,
        port: u16,
        user: String,
        pass: Masked<String>,
    }

    let config = Config {
        host: "pg".to_owned(),
        port: 5432,
        user: "app".to_owned(),
        pass: "the-password".to_owned().into(),
    };

    let serialized = ::serde_json::to_value(config.clone()).unwrap();
    let deserialized: Config = ::serde_json::from_value(serialized).unwrap();

    assert_eq!(config, deserialized);
}
