# gos-mirror

Mirror releases.grapheneos.org for air-gapped/enterprise deployments. Verify signatures, serve locally. Consumes OtaProvider trait.

## Build

```bash
nix build
nix run .#gos-mirror
cargo build
```

## Architecture

- Binary: `gos-mirror`
- Language: Rust (edition 2024, rust-version 1.91.0)
- License: MIT
- Nix: substrate `rust-tool-release-flake.nix` pattern
