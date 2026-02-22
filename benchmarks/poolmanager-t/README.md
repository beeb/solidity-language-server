# Solidity LSP Competition

Benchmarked against `v4-core` — `test/PoolManager.t.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `test/PoolManager.t.sol` |
| Position | line 116, col 51 |
| Iterations | 10 (2 warmup) |
| Timeout | 10s |

## Servers

| Server | Version |
|--------|---------|
| [mmsaki latest](https://github.com/mmsaki/solidity-language-server) | `solidity-language-server 0.1.24+commit.9746134.macos.aarch64` |
| [solc](https://docs.soliditylang.org) | `0.8.26+commit.8a97fa7a.Darwin.appleclang` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | `solidity-ls 0.5.4` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | `vscode-solidity-server 0.0.187` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | `@nomicfoundation/solidity-language-server 0.8.25` |

---

## Summary

| Method | mmsaki latest | solc | qiuxiang | juanfranblanco | nomicfoundation |
|--------|---------------|------|----------|----------------|-----------------|
| [initialize](#initialize) | **12.5ms** | 124.6ms | 113.2ms | 552.3ms | 873.6ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 2.2s | **155.5ms** | 204.4ms | crash | 2.7s |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | **12.1ms** | error | unsupported | crash | unsupported |
| [textDocument/definition](#textdocumentdefinition) | **138.9ms** | empty | empty | crash | empty |
| [textDocument/declaration](#textdocumentdeclaration) | **133.2ms** | unsupported | unsupported | crash | unsupported |
| [textDocument/hover](#textdocumenthover) | **242.6ms** | crash | empty | crash | empty |
| [textDocument/references](#textdocumentreferences) | **137.9ms** | unsupported | empty | crash | empty |
| [textDocument/completion](#textdocumentcompletion) | **4.8s** | unsupported | empty | crash | empty |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | **33.2ms** | unsupported | empty | crash | empty |
| [textDocument/rename](#textdocumentrename) | **270.5ms** | error | 0.3ms | crash | 0.8ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | **0.2ms** | unsupported | unsupported | crash | unsupported |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | **6.2ms** | unsupported | unsupported | crash | 90.0ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | **2.7ms** | unsupported | unsupported | crash | unsupported |
| [textDocument/formatting](#textdocumentformatting) | **21.0ms** | 275.9ms | 1.9ms | crash | 1.0s |
| [textDocument/inlayHint](#textdocumentinlayhint) | **32.8ms** | unsupported | unsupported | crash | unsupported |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | **10.2ms** | error | unsupported | crash | 89.9ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | **6.8ms** | unsupported | unsupported | crash | unsupported |
| [workspace/symbol](#workspacesymbol) | **6.1ms** | unsupported | unsupported | crash | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki latest** | **17** | **18** |
| solc | 1 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **12.5ms** | - | ok | ✓ |
| **solc** | 124.6ms | - | ok | ✓ |
| **qiuxiang** | 113.2ms | - | ok | ✓ |
| **juanfranblanco** | 552.3ms | - | ok | ✓ |
| **nomicfoundation** | 873.6ms | - | ok | ✓ |

### textDocument/diagnostic

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | 2.2s | 229.1 MB | 15 diagnostics | ✓ |
| **solc** | **155.5ms** | **26.1 MB** | 0 diagnostics | ✓ |
| **qiuxiang** | 204.4ms | 62.6 MB | 1 diagnostics | ✓ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 2.7s | 661.5 MB | 1 diagnostics | ✓ |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **12.1ms** | **227.7 MB** | delta | ✓ |
| **solc** | - | 26.0 MB | error | ✗ |
| **qiuxiang** | - | 62.3 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.6 MB | unsupported | ✗ |

### textDocument/definition

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **138.9ms** | **227.5 MB** | `TickMath.sol:9` | ✓ |
| **solc** | - | 25.9 MB | empty | ✗ |
| **qiuxiang** | - | 62.4 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 658.0 MB | empty | ✗ |

### textDocument/declaration

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **133.2ms** | **227.8 MB** | `TickMath.sol:9` | ✓ |
| **solc** | - | 25.8 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.3 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 657.8 MB | unsupported | ✗ |

### textDocument/hover

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **242.6ms** | **227.7 MB** | error PoolNotInitialized() | ✓ |
| **solc** | - | 25.9 MB | crash | ✗ |
| **qiuxiang** | - | 62.2 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 658.0 MB | empty | ✗ |

### textDocument/references

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **137.9ms** | **227.0 MB** | 7 references | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.5 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.1 MB | empty | ✗ |

### textDocument/completion

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **4.8s** | **227.6 MB** | 23 items (amount0, amount1, checkTicks) | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.2 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.9 MB | empty | ✗ |

### textDocument/signatureHelp

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **33.2ms** | **227.6 MB** | function bound(uint256 x, uint256 min, uint256 max... | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.4 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.9 MB | empty | ✗ |

### textDocument/rename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **270.5ms** | **227.9 MB** | 9 edits in 1 files | ✓ |
| **solc** | - | 26.0 MB | error | ✗ |
| **qiuxiang** | 0.3ms | 62.4 MB | 0 edits in 0 files | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 0.8ms | 658.0 MB | 0 edits in 0 files | ✗ |

### textDocument/prepareRename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **0.2ms** | **227.2 MB** | ready (line 116) | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 660.3 MB | unsupported | ✗ |

### textDocument/documentSymbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **6.2ms** | **227.1 MB** | 35 symbols | ✓ |
| **solc** | - | 25.7 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.4 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 90.0ms | 658.0 MB | 1 symbols | ✓ |

### textDocument/documentLink

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **2.7ms** | **227.2 MB** | 33 links | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.3 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.0 MB | unsupported | ✗ |

### textDocument/formatting

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **21.0ms** | **227.3 MB** | 1 edits | ✓ |
| **solc** | 275.9ms | 26.0 MB | {"error":"Unknown method textDocument/fo... | ✗ |
| **qiuxiang** | 1.9ms | 62.5 MB | {"error":"Request textDocument/formattin... | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 1.0s | 659.0 MB | 1 edits | ✓ |

### textDocument/inlayHint

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **32.8ms** | **228.2 MB** | 1080 hints (name:, hooks:, name:) | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.5 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.8 MB | unsupported | ✗ |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **10.2ms** | **228.7 MB** | 1512 tokens | ✓ |
| **solc** | - | 26.1 MB | error | ✗ |
| **qiuxiang** | - | 62.4 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 89.9ms | 658.8 MB | 385 tokens | ✓ |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **6.8ms** | **226.9 MB** | 417 tokens | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.7 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.2 MB | unsupported | ✗ |

### workspace/symbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **6.1ms** | **227.4 MB** | 90 symbols | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.3 MB | unsupported | ✗ |

---

*Benchmark run: 2026-02-22T02:28:49Z*
