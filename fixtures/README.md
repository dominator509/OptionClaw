# Fixtures

This directory holds synthetic, non-secret fixture data for tests and local development.

- `market/` will contain sample market snapshots.
- `config/` will contain test config files when needed.
- `broker/` will contain mocked provider responses when needed.
- `state/` will contain persistence fixtures and examples for schema-versioned local state.
- `market/sample_snapshot.json` and `llm/sample_advisory.json` are the canonical fixture pairs for the paper run-once service tests.
- `config/invalid_config.toml` exercises config parse failures in CLI tests.
- `orders/sample_order_intent.json` exercises the risk explain CLI contract test.
