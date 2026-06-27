use std::path::PathBuf;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        AccountSnapshot, CalendarDate, OptionContract, OptionKind, OrderIntent, OrderSide,
        OrderType, Price,
    },
    errors::{AppError, InputError},
    execution::{ExecutionMode, ExecutionProvider, ExecutionReport, ExecutionStatus},
};

#[derive(Debug, Clone)]
pub struct AlpacaCredentials {
    api_key: String,
    api_secret: String,
}

impl AlpacaCredentials {
    pub fn from_env() -> Result<Self, AppError> {
        let api_key = read_secret_env("OPTIONCLAW_ALPACA_API_KEY")?;
        let api_secret = read_secret_env("OPTIONCLAW_ALPACA_API_SECRET")?;
        Ok(Self {
            api_key,
            api_secret,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AlpacaClient {
    base_url: String,
    credentials: AlpacaCredentials,
    http: Client,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpacaAccountStatus {
    pub account_id: String,
    pub status: String,
    pub options_approved_level: u8,
    pub options_trading_level: u8,
    pub trading_blocked: bool,
    pub equity: Price,
    pub buying_power: Option<Price>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpacaOrderReport {
    pub order_id: String,
    pub status: String,
    pub filled_quantity: u32,
    pub fill_price: Option<Price>,
}

#[derive(Debug, Deserialize)]
struct AlpacaAccountResponse {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    account_number: Option<String>,
    status: String,
    #[serde(default)]
    options_approved_level: Option<u8>,
    #[serde(default)]
    options_trading_level: Option<u8>,
    #[serde(default)]
    trading_blocked: bool,
    equity: String,
    #[serde(default)]
    buying_power: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AlpacaOrderResponse {
    id: String,
    status: String,
    #[serde(default)]
    filled_qty: Option<String>,
    #[serde(default)]
    filled_avg_price: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AlpacaContractResponse {
    symbol: String,
    underlying_symbol: String,
    expiration_date: String,
    strike_price: String,
    #[serde(rename = "type")]
    option_type: String,
}

#[derive(Debug, Serialize)]
pub struct AlpacaOrderRequest<'a> {
    pub symbol: &'a str,
    pub qty: String,
    pub side: &'a str,
    #[serde(rename = "type")]
    pub order_type: &'a str,
    pub time_in_force: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
}

impl AlpacaClient {
    pub fn new(base_url: impl Into<String>, credentials: AlpacaCredentials) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            credentials,
            http: Client::new(),
        }
    }

    pub fn account_status(&self) -> Result<AlpacaAccountStatus, AppError> {
        let response: AlpacaAccountResponse = self
            .http
            .get(format!("{}/v2/account", self.base_url))
            .headers(self.auth_headers()?)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|source| self.http_error("account status", source))?
            .json()
            .map_err(|source| self.http_error("account status json", source))?;
        response.to_status()
    }

    pub fn account_snapshot(&self) -> Result<AccountSnapshot, AppError> {
        let status = self.account_status()?;
        Ok(AccountSnapshot::new(
            status.account_id,
            status.equity,
            status.buying_power,
            0,
            Vec::new(),
        )?)
    }

    pub fn option_contract(
        &self,
        symbol: &str,
        as_of: CalendarDate,
    ) -> Result<OptionContract, AppError> {
        let response: AlpacaContractResponse = self
            .http
            .get(format!("{}/v2/options/contracts/{}", self.base_url, symbol))
            .headers(self.auth_headers()?)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|source| self.http_error("option contract", source))?
            .json()
            .map_err(|source| self.http_error("option contract json", source))?;
        response.to_contract(as_of)
    }

    pub fn submit_order(&self, intent: &OrderIntent) -> Result<AlpacaOrderReport, AppError> {
        let body = self.preview_order(intent)?;
        let response: AlpacaOrderResponse = self
            .http
            .post(format!("{}/v2/orders", self.base_url))
            .headers(self.auth_headers()?)
            .json(&body)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|source| self.http_error("submit order", source))?
            .json()
            .map_err(|source| self.http_error("submit order json", source))?;
        response.to_report()
    }

    pub fn cancel_order(&self, order_id: &str) -> Result<(), AppError> {
        self.http
            .delete(format!("{}/v2/orders/{}", self.base_url, order_id))
            .headers(self.auth_headers()?)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|source| self.http_error("cancel order", source))?;
        Ok(())
    }

    pub fn order_status(&self, order_id: &str) -> Result<AlpacaOrderReport, AppError> {
        let response: AlpacaOrderResponse = self
            .http
            .get(format!("{}/v2/orders/{}", self.base_url, order_id))
            .headers(self.auth_headers()?)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|source| self.http_error("order status", source))?
            .json()
            .map_err(|source| self.http_error("order status json", source))?;
        response.to_report()
    }

    pub fn preview_order<'a>(
        &self,
        intent: &'a OrderIntent,
    ) -> Result<AlpacaOrderRequest<'a>, AppError> {
        if intent.side != OrderSide::Buy {
            return Err(invalid(
                "alpaca-order",
                "first live release allows long calls and puts only",
            ));
        }
        let side = "buy";
        let order_type = match intent.order_type {
            OrderType::Market => "market",
            OrderType::Limit => "limit",
        };
        let limit_price = intent.limit_price.map(format_price);
        Ok(AlpacaOrderRequest {
            symbol: &intent.contract.symbol,
            qty: intent.quantity.get().to_string(),
            side,
            order_type,
            time_in_force: "day",
            limit_price,
        })
    }

    fn auth_headers(&self) -> Result<reqwest::header::HeaderMap, AppError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "APCA-API-KEY-ID",
            reqwest::header::HeaderValue::from_str(&self.credentials.api_key).map_err(
                |source| invalid("alpaca-auth", format!("invalid API key header: {source}")),
            )?,
        );
        headers.insert(
            "APCA-API-SECRET-KEY",
            reqwest::header::HeaderValue::from_str(&self.credentials.api_secret).map_err(
                |source| {
                    invalid(
                        "alpaca-auth",
                        format!("invalid API secret header: {source}"),
                    )
                },
            )?,
        );
        Ok(headers)
    }

    fn http_error(&self, operation: &'static str, source: reqwest::Error) -> AppError {
        invalid(
            "alpaca-http",
            format!("Alpaca {operation} failed without exposing credentials: {source}"),
        )
    }
}

impl ExecutionProvider for AlpacaClient {
    fn execute_live(&self, intent: &OrderIntent) -> Result<ExecutionReport, AppError> {
        let report = self.submit_order(intent)?;
        Ok(ExecutionReport {
            mode: ExecutionMode::Live,
            status: ExecutionStatus::LiveSubmitted,
            intent_id: intent.id.clone(),
            filled_quantity: report.filled_quantity,
            fill_price: report.fill_price,
        })
    }
}

impl AlpacaAccountResponse {
    fn to_status(&self) -> Result<AlpacaAccountStatus, AppError> {
        Ok(AlpacaAccountStatus {
            account_id: self
                .id
                .clone()
                .or_else(|| self.account_number.clone())
                .unwrap_or_else(|| "alpaca-account".to_string()),
            status: self.status.clone(),
            options_approved_level: self.options_approved_level.unwrap_or(0),
            options_trading_level: self.options_trading_level.unwrap_or(0),
            trading_blocked: self.trading_blocked,
            equity: parse_decimal_micros(&self.equity)?,
            buying_power: self
                .buying_power
                .as_deref()
                .map(parse_decimal_micros)
                .transpose()?,
        })
    }
}

impl AlpacaContractResponse {
    fn to_contract(&self, as_of: CalendarDate) -> Result<OptionContract, AppError> {
        let expiration = parse_date(&self.expiration_date)?;
        let kind = match self.option_type.to_ascii_lowercase().as_str() {
            "call" => OptionKind::Call,
            "put" => OptionKind::Put,
            _ => return Err(invalid("alpaca-contract", "unknown option contract type")),
        };
        Ok(OptionContract::new(
            self.symbol.clone(),
            self.underlying_symbol.clone(),
            expiration,
            parse_decimal_micros(&self.strike_price)?,
            kind,
            None::<String>,
            as_of,
        )?)
    }
}

impl AlpacaOrderResponse {
    fn to_report(&self) -> Result<AlpacaOrderReport, AppError> {
        Ok(AlpacaOrderReport {
            order_id: self.id.clone(),
            status: self.status.clone(),
            filled_quantity: self
                .filled_qty
                .as_deref()
                .unwrap_or("0")
                .parse::<u32>()
                .unwrap_or(0),
            fill_price: self
                .filled_avg_price
                .as_deref()
                .map(parse_decimal_micros)
                .transpose()?,
        })
    }
}

fn read_secret_env(name: &str) -> Result<String, AppError> {
    let value = std::env::var(name).unwrap_or_default();
    if value.trim().is_empty() {
        return Err(AppError::from(
            crate::errors::SecurityError::SecretMissing {
                name: name.to_string(),
            },
        ));
    }
    Ok(value)
}

fn parse_decimal_micros(value: &str) -> Result<Price, AppError> {
    let trimmed = value.trim();
    let Some((dollars, cents)) = trimmed.split_once('.') else {
        let units = trimmed.parse::<i64>().map_err(|source| {
            invalid(
                "alpaca-decimal",
                format!("invalid decimal amount: {source}"),
            )
        })?;
        return Ok(Price::from_micros(units * Price::SCALE)?);
    };
    let dollars = dollars.parse::<i64>().map_err(|source| {
        invalid(
            "alpaca-decimal",
            format!("invalid decimal amount: {source}"),
        )
    })?;
    let mut fraction = cents.to_string();
    fraction.truncate(6);
    while fraction.len() < 6 {
        fraction.push('0');
    }
    let micros = fraction.parse::<i64>().map_err(|source| {
        invalid(
            "alpaca-decimal",
            format!("invalid decimal amount: {source}"),
        )
    })?;
    Ok(Price::from_micros(dollars * Price::SCALE + micros)?)
}

fn parse_date(value: &str) -> Result<CalendarDate, AppError> {
    let parts: Vec<_> = value.split('-').collect();
    if parts.len() != 3 {
        return Err(invalid("alpaca-date", "expected yyyy-mm-dd"));
    }
    Ok(CalendarDate::new(
        parts[0]
            .parse()
            .map_err(|source| invalid("alpaca-date", format!("invalid year: {source}")))?,
        parts[1]
            .parse()
            .map_err(|source| invalid("alpaca-date", format!("invalid month: {source}")))?,
        parts[2]
            .parse()
            .map_err(|source| invalid("alpaca-date", format!("invalid day: {source}")))?,
    )?)
}

fn format_price(price: Price) -> String {
    format!(
        "{}.{:06}",
        price.micros() / Price::SCALE,
        price.micros() % Price::SCALE
    )
}

fn invalid(path: impl Into<String>, detail: impl Into<String>) -> AppError {
    AppError::from(InputError::Invalid {
        path: PathBuf::from(path.into()),
        detail: detail.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ContractQuantity, OrderIntentSpec, TradingMode};
    use std::time::{Duration, UNIX_EPOCH};

    fn credentials() -> AlpacaCredentials {
        AlpacaCredentials {
            api_key: "key".to_string(),
            api_secret: "secret".to_string(),
        }
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
    fn decimal_parser_converts_to_micros() {
        assert_eq!(
            parse_decimal_micros("12.34").expect("valid").micros(),
            12_340_000
        );
    }

    #[test]
    fn preview_order_rejects_short_options() {
        let client = AlpacaClient::new("http://127.0.0.1", credentials());
        let err = client
            .preview_order(&intent(OrderSide::Sell))
            .expect_err("sell side should fail");
        assert!(format!("{err}").contains("long calls and puts only"));
    }
}
