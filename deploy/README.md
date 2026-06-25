# OptionClaw Deploy

This directory documents a manual release layout for operator-owned local hardware,
Raspberry Pi, or a small VPS. It intentionally stays paper-mode first and does
not include a systemd unit because the repository does not yet contain evidence
for one.

## Release Layout

Recommended paths:

- Binary: `/opt/optionclaw/bin/optionclaw`
- Config: `/opt/optionclaw/config/production.toml`
- Data: `/opt/optionclaw/var/dev`
- Releases: `/opt/optionclaw/releases/<version>/optionclaw`

The config path matters because OptionClaw derives its local data directory from
the config location. Keeping the config under `/opt/optionclaw/config/` preserves
the `/opt/optionclaw/var/dev` data layout.

## Release Steps

1. Build the artifact with `./scripts/build.sh`.
2. Copy `target/release/optionclaw` to `/opt/optionclaw/releases/<version>/optionclaw`.
3. Make the release binary executable.
4. Copy `config/production.example.toml` to `/opt/optionclaw/config/production.toml`.
5. Run `optionclaw check-config --config /opt/optionclaw/config/production.toml`.
6. Run `optionclaw health --config /opt/optionclaw/config/production.toml`.
7. Start the release binary with the operator's preferred supervisor or shell.

## Paper-Mode Smoke

After deployment, verify:

```sh
optionclaw --help
optionclaw check-config --config /opt/optionclaw/config/production.toml
optionclaw health --config /opt/optionclaw/config/production.toml
```

Do not enable live mode in this release path. Live execution remains gated by
later ExecPlans and explicit operator approval.
