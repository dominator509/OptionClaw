use std::path::PathBuf;

use optionclaw::config::{AppConfig, TradingMode};

#[test]
fn loads_example_config_with_paper_mode() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("config/example.toml");

    let config = AppConfig::load_from_path(&path).expect("example config should parse");

    assert_eq!(config.trading_mode, TradingMode::Paper);
}
