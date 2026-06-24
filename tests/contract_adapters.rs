use optionclaw::{
    domain::{
        CalendarDate, ContractQuantity, MarketSnapshot, OptionContract, OptionKind, OrderIntent,
        OrderIntentSpec, OrderSide, OrderType, PercentBps, Price, Signal, SignalSource,
        TradingMode,
    },
    execution::{
        DisabledExecutionProvider, ExecutionMode, ExecutionProvider, ExecutionStatus,
        FixturePaperExecutor, PaperExecutor,
    },
    llm::{AdvisoryContext, AdvisoryResult, FixtureLlmAdvisor, LlmAdvisor},
    market_data::{FixtureMarketDataProvider, MarketDataProvider, MarketDataRequest},
};
use std::time::{Duration, UNIX_EPOCH};

fn contract() -> OptionContract {
    OptionContract::new(
        "AAPL260201C00100000",
        "AAPL",
        CalendarDate::new(2026, 2, 1).expect("valid date"),
        Price::from_micros(1_000_000).expect("valid price"),
        OptionKind::Call,
        None::<String>,
        CalendarDate::new(2026, 1, 1).expect("valid date"),
    )
    .expect("valid contract")
}

fn market_snapshot() -> MarketSnapshot {
    MarketSnapshot::new(
        contract(),
        UNIX_EPOCH + Duration::from_secs(1704067201),
        Price::from_micros(19_000_000).expect("valid price"),
        Some(Price::from_micros(4_500).expect("valid price")),
        Some(Price::from_micros(5_000).expect("valid price")),
        Some(Price::from_micros(4_800).expect("valid price")),
        Some(2_500),
    )
    .expect("valid snapshot")
}

fn order_intent() -> OrderIntent {
    OrderIntent::new(OrderIntentSpec {
        id: "intent-1".to_string(),
        mode: TradingMode::Paper,
        contract: contract(),
        side: OrderSide::Buy,
        quantity: ContractQuantity::new(1).expect("valid quantity"),
        order_type: OrderType::Limit,
        limit_price: Some(Price::from_micros(5_000).expect("valid price")),
        estimated_max_loss: Price::from_micros(5_000).expect("valid price"),
        strategy_id: "strategy-1".to_string(),
        risk_context_id: "risk-1".to_string(),
        created_at: UNIX_EPOCH + Duration::from_secs(1704067201),
    })
    .expect("valid order intent")
}

#[test]
fn market_provider_returns_domain_snapshot() {
    let provider = FixtureMarketDataProvider::new(market_snapshot());
    let snapshot = provider
        .snapshot(MarketDataRequest {
            contract: contract(),
            as_of: CalendarDate::new(2026, 1, 1).expect("valid date"),
        })
        .expect("market snapshot should resolve");
    assert_eq!(snapshot.contract.symbol, "AAPL260201C00100000");
}

#[test]
fn llm_provider_returns_advisory() {
    let provider = FixtureLlmAdvisor::new(AdvisoryResult {
        score: 75,
        confidence: PercentBps::from_bps(8_500).expect("valid confidence"),
        explanation: "fixture advisory".to_string(),
    });
    let advisory = provider
        .advise(AdvisoryContext {
            market: market_snapshot(),
            signal: Signal::new(
                SignalSource::Model,
                75,
                PercentBps::from_bps(8_500).expect("valid confidence"),
                UNIX_EPOCH + Duration::from_secs(1704067201),
                "fixture advisory",
            )
            .expect("valid signal"),
        })
        .expect("advisory should resolve");
    assert_eq!(advisory.score, 75);
    assert_eq!(advisory.confidence.bps(), 8_500);
}

#[test]
fn paper_executor_returns_filled_report() {
    let executor = FixturePaperExecutor::new();
    let report = executor
        .execute(&order_intent())
        .expect("paper execute should succeed");
    assert_eq!(report.mode, ExecutionMode::Paper);
    assert_eq!(report.status, ExecutionStatus::PaperFilled);
    assert_eq!(report.filled_quantity, 1);
}

#[test]
fn live_provider_stays_disabled() {
    let provider = DisabledExecutionProvider::new();
    let report = provider
        .execute_live(&order_intent())
        .expect("live provider should return disabled report");
    assert_eq!(report.mode, ExecutionMode::Live);
    assert_eq!(report.status, ExecutionStatus::LiveDisabled);
}
