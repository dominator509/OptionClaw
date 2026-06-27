use std::{
    sync::Mutex,
    time::{Duration, UNIX_EPOCH},
};

use httpmock::prelude::*;
use optionclaw::{
    alpaca::{AlpacaClient, AlpacaCredentials},
    domain::{
        CalendarDate, ContractQuantity, OptionContract, OptionKind, OrderIntent, OrderIntentSpec,
        OrderSide, OrderType, Price, TradingMode,
    },
};
use serde_json::json;

static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvGuard;

impl EnvGuard {
    fn set() -> Self {
        std::env::set_var("OPTIONCLAW_ALPACA_API_KEY", "contract-key");
        std::env::set_var("OPTIONCLAW_ALPACA_API_SECRET", "contract-secret");
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        std::env::remove_var("OPTIONCLAW_ALPACA_API_KEY");
        std::env::remove_var("OPTIONCLAW_ALPACA_API_SECRET");
    }
}

fn client(server: &MockServer) -> (EnvGuard, AlpacaClient) {
    let guard = EnvGuard::set();
    let credentials = AlpacaCredentials::from_env().expect("test credentials should load");
    (guard, AlpacaClient::new(server.base_url(), credentials))
}

fn intent(side: OrderSide) -> OrderIntent {
    let as_of = CalendarDate::new(2026, 1, 1).expect("valid date");
    let contract = OptionContract::new(
        "AAPL260201C00100000",
        "AAPL",
        CalendarDate::new(2026, 2, 1).expect("valid date"),
        Price::from_micros(100_000_000).expect("valid price"),
        OptionKind::Call,
        None::<String>,
        as_of,
    )
    .expect("valid contract");
    OrderIntent::new(OrderIntentSpec {
        id: "intent-1".to_string(),
        mode: TradingMode::Live,
        contract,
        side,
        quantity: ContractQuantity::new(1).expect("valid quantity"),
        order_type: OrderType::Limit,
        limit_price: Some(Price::from_micros(1_250_000).expect("valid price")),
        estimated_max_loss: Price::from_micros(1_250_000).expect("valid price"),
        strategy_id: "aggressive-growth-v1".to_string(),
        risk_context_id: "aggressive-growth-risk-v1".to_string(),
        created_at: UNIX_EPOCH + Duration::from_secs(1),
    })
    .expect("valid intent")
}

#[test]
fn account_status_maps_options_capability_and_auth_headers() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v2/account")
            .header("APCA-API-KEY-ID", "contract-key")
            .header("APCA-API-SECRET-KEY", "contract-secret");
        then.status(200).json_body(json!({
            "id": "acct-123",
            "status": "ACTIVE",
            "options_approved_level": 2,
            "options_trading_level": 2,
            "trading_blocked": false,
            "equity": "100000.00",
            "buying_power": "50000.00"
        }));
    });
    let (_env, client) = client(&server);

    let status = client.account_status().expect("account should map");

    assert_eq!(status.account_id, "acct-123");
    assert_eq!(status.options_approved_level, 2);
    assert_eq!(status.options_trading_level, 2);
    assert_eq!(status.equity.micros(), 100_000_000_000);
    mock.assert();
}

#[test]
fn option_contract_maps_alpaca_contract_response() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v2/options/contracts/AAPL260201C00100000");
        then.status(200).json_body(json!({
            "symbol": "AAPL260201C00100000",
            "underlying_symbol": "AAPL",
            "expiration_date": "2026-02-01",
            "strike_price": "100.00",
            "type": "call"
        }));
    });
    let (_env, client) = client(&server);

    let contract = client
        .option_contract(
            "AAPL260201C00100000",
            CalendarDate::new(2026, 1, 1).expect("valid date"),
        )
        .expect("contract should map");

    assert_eq!(contract.symbol, "AAPL260201C00100000");
    assert_eq!(contract.kind, OptionKind::Call);
    assert_eq!(contract.strike.micros(), 100_000_000);
    mock.assert();
}

#[test]
fn preview_and_submit_order_map_limit_day_buy() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let server = MockServer::start();
    let submit = server.mock(|when, then| {
        when.method(POST).path("/v2/orders");
        then.status(200).json_body(json!({
            "id": "alpaca-order-1",
            "status": "accepted",
            "filled_qty": "0",
            "filled_avg_price": null
        }));
    });
    let (_env, client) = client(&server);
    let intent = intent(OrderSide::Buy);

    let preview = client.preview_order(&intent).expect("preview should map");
    assert_eq!(preview.symbol, "AAPL260201C00100000");
    assert_eq!(preview.qty, "1");
    assert_eq!(preview.side, "buy");
    assert_eq!(preview.order_type, "limit");
    assert_eq!(preview.time_in_force, "day");
    assert_eq!(preview.limit_price.as_deref(), Some("1.250000"));

    let report = client.submit_order(&intent).expect("submit should map");

    assert_eq!(report.order_id, "alpaca-order-1");
    assert_eq!(report.status, "accepted");
    submit.assert_calls(1);
}

#[test]
fn cancel_and_status_mapping_use_order_endpoints() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let server = MockServer::start();
    let cancel = server.mock(|when, then| {
        when.method(DELETE).path("/v2/orders/alpaca-order-1");
        then.status(204);
    });
    let status = server.mock(|when, then| {
        when.method(GET).path("/v2/orders/alpaca-order-1");
        then.status(200).json_body(json!({
            "id": "alpaca-order-1",
            "status": "partially_filled",
            "filled_qty": "1",
            "filled_avg_price": "1.25"
        }));
    });
    let (_env, client) = client(&server);

    client
        .cancel_order("alpaca-order-1")
        .expect("cancel should map");
    let report = client
        .order_status("alpaca-order-1")
        .expect("status should map");

    assert_eq!(report.status, "partially_filled");
    assert_eq!(report.filled_quantity, 1);
    assert_eq!(report.fill_price.expect("fill price").micros(), 1_250_000);
    cancel.assert_calls(1);
    status.assert_calls(1);
}

#[test]
fn auth_rate_limit_and_rejection_errors_do_not_expose_secrets() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/v2/account");
        then.status(401)
            .json_body(json!({"message": "unauthorized"}));
    });
    server.mock(|when, then| {
        when.method(GET).path("/v2/orders/rate-limited");
        then.status(429).json_body(json!({"message": "rate limit"}));
    });
    server.mock(|when, then| {
        when.method(POST).path("/v2/orders");
        then.status(422).json_body(json!({"message": "rejected"}));
    });
    let (_env, client) = client(&server);

    for error in [
        client.account_status().expect_err("auth should fail"),
        client
            .order_status("rate-limited")
            .expect_err("rate limit should fail"),
        client
            .submit_order(&intent(OrderSide::Buy))
            .expect_err("rejection should fail"),
    ] {
        let rendered = error.to_string();
        assert!(rendered.contains("Alpaca"));
        assert!(!rendered.contains("contract-key"));
        assert!(!rendered.contains("contract-secret"));
    }
}
