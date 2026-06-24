use std::{
    collections::BTreeMap,
    sync::{Mutex, OnceLock},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricEvent {
    name: String,
    labels: BTreeMap<String, String>,
}

impl MetricEvent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            labels: BTreeMap::new(),
        }
    }

    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    pub fn command_success(command: impl Into<String>) -> Self {
        Self::new("command_success").with_label("command", command)
    }

    pub fn command_failure(command: impl Into<String>, error_code: impl Into<String>) -> Self {
        Self::new("command_failure")
            .with_label("command", command)
            .with_label("error_code", error_code)
    }

    pub fn config_validation(success: bool, mode: impl Into<String>) -> Self {
        Self::new("config_validation")
            .with_label("result", if success { "success" } else { "failure" })
            .with_label("mode", mode)
    }

    pub fn risk_decision(accepted: bool) -> Self {
        Self::new("risk_decision")
            .with_label("result", if accepted { "accepted" } else { "rejected" })
    }

    pub fn paper_execution(executed: bool) -> Self {
        Self::new("paper_execution")
            .with_label("result", if executed { "executed" } else { "rejected" })
    }

    pub fn adapter_result(
        provider: impl Into<String>,
        operation: impl Into<String>,
        success: bool,
    ) -> Self {
        Self::new("adapter_result")
            .with_label("provider", provider)
            .with_label("operation", operation)
            .with_label("result", if success { "success" } else { "failure" })
    }

    pub fn audit_append(success: bool) -> Self {
        Self::new("audit_append").with_label("result", if success { "success" } else { "failure" })
    }

    pub fn health_status(
        config_ready: bool,
        data_ready: bool,
        audit_ready: bool,
        secrets_store_ready: bool,
        providers_ready: bool,
        kill_switch_active: bool,
    ) -> Self {
        Self::new("health_status")
            .with_label("config_ready", config_ready.to_string())
            .with_label("data_ready", data_ready.to_string())
            .with_label("audit_ready", audit_ready.to_string())
            .with_label("secrets_store_ready", secrets_store_ready.to_string())
            .with_label("providers_ready", providers_ready.to_string())
            .with_label("kill_switch_active", kill_switch_active.to_string())
    }
}

pub type MetricSnapshot = BTreeMap<String, u64>;

static METRICS: OnceLock<Mutex<MetricSnapshot>> = OnceLock::new();

pub fn record_metric(event: MetricEvent) {
    let mut guard = metric_registry()
        .lock()
        .expect("metric registry should be usable");
    *guard.entry(event.key()).or_insert(0) += 1;
}

pub fn snapshot_metrics() -> MetricSnapshot {
    metric_registry()
        .lock()
        .expect("metric registry should be usable")
        .clone()
}

pub fn reset_metrics_for_test() {
    metric_registry()
        .lock()
        .expect("metric registry should be usable")
        .clear();
}

impl MetricEvent {
    pub fn key(&self) -> String {
        if self.labels.is_empty() {
            return self.name.clone();
        }

        let labels = self
            .labels
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(",");
        format!("{}{{{labels}}}", self.name)
    }
}

fn metric_registry() -> &'static Mutex<MetricSnapshot> {
    METRICS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_metrics_with_stable_keys() {
        reset_metrics_for_test();

        record_metric(MetricEvent::command_success("health"));
        record_metric(MetricEvent::command_success("health"));
        record_metric(MetricEvent::command_failure("health", "E1001"));

        let snapshot = snapshot_metrics();
        assert_eq!(
            snapshot.get("command_success{command=health}"),
            Some(&2_u64)
        );
        assert_eq!(
            snapshot.get("command_failure{command=health,error_code=E1001}"),
            Some(&1_u64)
        );
    }

    #[test]
    fn health_status_metric_includes_expected_labels() {
        let event = MetricEvent::health_status(true, false, true, true, false, false);
        assert!(event.key().contains("health_status"));
        assert!(event.key().contains("kill_switch_active=false"));
    }
}
