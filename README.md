# Solidity Language Server

[![Crates.io](https://img.shields.io/crates/v/solidity-language-server)](https://crates.io/crates/solidity-language-server)
[![Tests](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml)
[![Release](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/mmsaki/solidity-language-server)](https://github.com/mmsaki/solidity-language-server/releases/latest)
[![Telegram](https://img.shields.io/badge/Telegram-Join%20Chat-blue?logo=telegram)](https://t.me/+R1lW7xWJ55tlYzcx)

The fastest Solidity language server — go-to-definition, references, rename, completions, hover, and more. See [benchmarks](https://github.com/mmsaki/solidity-lsp-benchmarks).

## Install

```sh
cargo install solidity-language-server
```

Or download a pre-built binary from the [latest release](https://github.com/mmsaki/solidity-language-server/releases/latest).

## Features

- **Go to Definition** / **Go to Declaration** — jump to any symbol across files
- **Find References** — all usages of a symbol across the project
- **Rename** — project-wide symbol rename with prepare support
- **Hover** — signatures, NatSpec docs, function/error/event selectors, `@inheritdoc` resolution
- **Completions** — scope-aware with two modes (fast cache vs full recomputation)
- **Document Links** — clickable imports, type names, function calls
- **Document Symbols** / **Workspace Symbols** — outline and search
- **Formatting** — via `forge fmt`
- **Diagnostics** — from `solc` and `forge lint`
- **Signature Help** — parameter info on function calls, event emits, and mapping access
- **Inlay Hints** — parameter names and gas estimates

See [FEATURES.md](FEATURES.md) for the full LSP feature set and roadmap.

## Benchmarks

p95 latency. `-` = unsupported, empty, error, or crash. ⚡ = fastest.

| Server | Version |
|--------|---------|
| [mmsaki](https://github.com/mmsaki/solidity-language-server) | `0.1.24` |
| [solc](https://docs.soliditylang.org) | `0.8.26` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | `0.8.25` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | `0.0.187` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | `0.5.4` |

### Shop.sol (272 lines) — [full results](benchmarks/shop/README.md)

| Method | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|--------|--------|------|-----------------|----------------|----------|
| initialize | ⚡ 12.2ms | 122.5ms | 846.0ms | 576.9ms | 93.0ms |
| diagnostic | 81.8ms | ⚡ 2.7ms | 506.3ms | 771.4ms | 275.8ms |
| definition | ⚡ 3.2ms | - | - | 70.0ms | 21.1ms |
| declaration | ⚡ 1.4ms | - | - | - | - |
| hover | ⚡ 4.4ms | - | - | 67.2ms | - |
| references | 2.2ms | - | ⚡ 1.8ms | 71.2ms | 21.0ms |
| completion | ⚡ 0.2ms | - | - | - | 20.2ms |
| signatureHelp | ⚡ 2.7ms | - | - | - | - |
| rename | 4.0ms | - | ⚡ 1.9ms | - | 22.0ms |
| documentSymbol | ⚡ 1.2ms | - | 16.0ms | 6.8ms | - |
| formatting | ⚡ 12.7ms | - | 175.7ms | - | - |
| inlayHint | ⚡ 1.9ms | - | - | - | - |
| semanticTokens/full | ⚡ 1.6ms | - | 14.3ms | - | - |
| semanticTokens/range | ⚡ 1.0ms | - | - | - | - |
| semanticTokens/delta | ⚡ 1.5ms | - | - | - | - |
| workspace/symbol | ⚡ 1.1ms | - | - | - | - |

**Scorecard: mmsaki 14/18 ⚡**

### Pool.sol (v4-core, 428 lines) — [full results](benchmarks/pool/README.md)

| Method | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|--------|--------|------|-----------------|----------------|----------|
| initialize | ⚡ 12.9ms | 181.2ms | 1.0s | 872.8ms | 99.1ms |
| diagnostic | 810.6ms | ⚡ 171.1ms | 1.0s | crash | 284.0ms |
| definition | ⚡ 11.1ms | - | - | crash | 2.3ms |
| declaration | ⚡ 9.1ms | - | - | crash | - |
| hover | ⚡ 23.1ms | - | - | crash | - |
| references | ⚡ 19.4ms | - | - | crash | - |
| completion | ⚡ 0.4ms | - | 166.9ms | crash | 2.4ms |
| signatureHelp | ⚡ 11.7ms | - | - | crash | - |
| rename | ⚡ 21.2ms | - | 1.7ms | crash | 0.5ms |
| documentSymbol | ⚡ 2.4ms | - | 35.5ms | crash | - |
| documentLink | ⚡ 83.2ms | - | - | crash | - |
| formatting | - | - | ⚡ 144.4ms | crash | - |
| inlayHint | ⚡ 2.8ms | - | - | crash | - |
| semanticTokens/full | ⚡ 3.7ms | - | 45.4ms | crash | - |
| semanticTokens/range | ⚡ 2.3ms | - | - | crash | - |
| semanticTokens/delta | ⚡ 3.6ms | - | - | crash | - |
| workspace/symbol | ⚡ 2.7ms | - | - | crash | - |

**Scorecard: mmsaki 15/18 ⚡**

### PoolManager.t.sol (v4-core, 1262 lines, viaIR) — [full results](benchmarks/poolmanager-t/README.md)

| Method | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|--------|--------|------|-----------------|----------------|----------|
| initialize | **12.5ms** | 124.6ms | 873.6ms | 552.3ms | 113.2ms |
| diagnostic | 2.2s | **155.5ms** | 2.7s | crash | 204.4ms |
| definition | **138.9ms** | - | - | crash | - |
| declaration | **133.2ms** | - | - | crash | - |
| hover | **242.6ms** | - | - | crash | - |
| references | **137.9ms** | - | - | crash | - |
| completion | **4.8s** | - | - | crash | - |
| signatureHelp | **33.2ms** | - | - | crash | - |
| rename | **270.5ms** | - | 0.8ms | crash | 0.3ms |
| documentSymbol | **6.2ms** | - | 90.0ms | crash | - |
| documentLink | **2.7ms** | - | - | crash | - |
| formatting | **21.0ms** | - | 1.0s | crash | - |
| inlayHint | **32.8ms** | - | - | crash | - |
| semanticTokens/full | **10.2ms** | - | 89.9ms | crash | - |
| semanticTokens/range | **6.8ms** | - | - | crash | - |
| semanticTokens/delta | **12.1ms** | - | - | crash | - |
| workspace/symbol | **6.1ms** | - | - | crash | - |

**Scorecard: mmsaki 17/18 wins**

## Docs

- [FEATURES.md](FEATURES.md) — full LSP feature set and roadmap
- [CONTRIBUTING.md](CONTRIBUTING.md) — development setup, project structure, and how to contribute
- [CHANGELOG.md](CHANGELOG.md) — release history

## Neovim

```lua
return {
  name = "Solidity Language Server",
  cmd = { "solidity-language-server" },
  root_dir = vim.fs.root(0, { "foundry.toml", ".git" }),
  filetypes = { "solidity" },
  root_markers = { "foundry.toml", ".git" },
  on_attach = function(_, _)
    vim.api.nvim_create_autocmd("BufWritePost", {
      pattern = { "*.sol" },
      callback = function()
        vim.lsp.buf.format()
      end,
    })
  end,
}
```

## Settings

Settings are passed via `initializationOptions` or `didChangeConfiguration`. All settings are optional — defaults are shown below.

```lua
-- Neovim: lsp/forge_lsp.lua
settings = {
  ["solidity-language-server"] = {
    inlayHints = {
      -- Show parameter name hints on function/event/struct calls.
      parameters = true,
      -- Show gas cost hints on functions annotated with
      -- `/// @custom:lsp-enable gas-estimates`.
      gasEstimates = true,
    },
    lint = {
      -- Master toggle for forge lint diagnostics.
      enabled = true,
      -- Filter lints by severity. Empty = all severities.
      -- Values: "high", "med", "low", "info", "gas", "code-size"
      severity = {},
      -- Run only specific lint rules by ID. Empty = all rules.
      -- Values: "incorrect-shift", "unchecked-call", "erc20-unchecked-transfer",
      --   "divide-before-multiply", "unsafe-typecast", "pascal-case-struct",
      --   "mixed-case-function", "mixed-case-variable", "screaming-snake-case-const",
      --   "screaming-snake-case-immutable", "unused-import", "unaliased-plain-import",
      --   "named-struct-fields", "unsafe-cheatcode", "asm-keccak256", "custom-errors",
      --   "unwrapped-modifier-logic"
      only = {},
      -- Suppress specific lint rule IDs from diagnostics.
      exclude = {},
    },
  },
}
```

### Helix

```toml
# languages.toml
[language-server.solidity-language-server.config]
inlayHints.parameters = true
inlayHints.gasEstimates = true
lint.enabled = true
lint.severity = ["high", "med"]
lint.exclude = ["pascal-case-struct"]
```

## Verify Release Binaries

Release binaries are GPG-signed. Download `checksums-sha256.txt`, `checksums-sha256.txt.asc`, and [`public-key.asc`](public-key.asc) from the [release](https://github.com/mmsaki/solidity-language-server/releases/latest):

```sh
gpg --import public-key.asc
gpg --verify checksums-sha256.txt.asc checksums-sha256.txt
sha256sum -c checksums-sha256.txt
```
