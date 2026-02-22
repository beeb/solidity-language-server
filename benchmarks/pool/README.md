# Solidity LSP Competition

Benchmarked against `v4-core` — `src/libraries/Pool.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `src/libraries/Pool.sol` |
| Position | line 102, col 15 |
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
| [initialize](#initialize) | **7.9ms** | 113.9ms | 116.7ms | 541.6ms | 869.0ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 499.1ms | **149.2ms** | 256.4ms | crash | 945.5ms |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | **3.5ms** | 0.1ms | 0.2ms | crash | 0.2ms |
| [textDocument/definition](#textdocumentdefinition) | 11.0ms | empty | **0.4ms** | crash | empty |
| [textDocument/declaration](#textdocumentdeclaration) | **9.2ms** | unsupported | unsupported | crash | unsupported |
| [textDocument/hover](#textdocumenthover) | **14.0ms** | crash | empty | crash | empty |
| [textDocument/references](#textdocumentreferences) | **12.0ms** | unsupported | empty | crash | empty |
| [textDocument/completion](#textdocumentcompletion) | **0.2ms** | unsupported | 0.3ms | crash | 16.0ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | **11.7ms** | unsupported | empty | crash | empty |
| [textDocument/rename](#textdocumentrename) | **20.1ms** | error | 0.3ms | crash | 0.6ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | **0.2ms** | unsupported | unsupported | crash | unsupported |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | **2.5ms** | unsupported | unsupported | crash | 29.8ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | **0.8ms** | unsupported | unsupported | crash | unsupported |
| [textDocument/formatting](#textdocumentformatting) | **19.5ms** | 276.3ms | 2.0ms | crash | 430.4ms |
| [textDocument/inlayHint](#textdocumentinlayhint) | **2.7ms** | unsupported | unsupported | crash | unsupported |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | **3.7ms** | error | unsupported | crash | 28.4ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | **2.5ms** | unsupported | unsupported | crash | unsupported |
| [workspace/symbol](#workspacesymbol) | **2.2ms** | unsupported | unsupported | crash | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki latest** | **16** | **18** |
| qiuxiang | 1 | 18 |
| solc | 1 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **7.9ms** | - | ok | ✓ |
| **solc** | 113.9ms | - | ok | ✓ |
| **qiuxiang** | 116.7ms | - | ok | ✓ |
| **juanfranblanco** | 541.6ms | - | ok | ✓ |
| **nomicfoundation** | 869.0ms | - | ok | ✓ |

### textDocument/diagnostic

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | 499.1ms | 54.5 MB | 4 diagnostics | ✓ |
| **solc** | **149.2ms** | **26.2 MB** | 0 diagnostics | ✓ |
| **qiuxiang** | 256.4ms | 72.4 MB | 0 diagnostics | ✓ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 945.5ms | 525.5 MB | 0 diagnostics | ✓ |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **3.5ms** | **53.7 MB** | delta | ✓ |
| **solc** | 0.1ms | 26.1 MB | {"error":"Unknown method textDocument/se... | ✗ |
| **qiuxiang** | 0.2ms | 70.9 MB | {"error":"Unhandled method textDocument/... | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 0.2ms | 524.8 MB | {"error":"Unhandled method textDocument/... | ✗ |

### textDocument/definition

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | 11.0ms | **52.5 MB** | `TickMath.sol:9` | ✓ |
| **solc** | - | 26.0 MB | empty | ✗ |
| **qiuxiang** | **0.4ms** | 71.6 MB | `Pool.sol:102` | ✓ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 525.7 MB | empty | ✗ |

### textDocument/declaration

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **9.2ms** | **52.6 MB** | `TickMath.sol:9` | ✓ |
| **solc** | - | 26.2 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.8 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 524.2 MB | unsupported | ✗ |

### textDocument/hover

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **14.0ms** | **52.7 MB** | function modifyLiquidity(struct Pool.State storage... | ✓ |
| **solc** | - | 25.9 MB | crash | ✗ |
| **qiuxiang** | - | 71.6 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 524.0 MB | empty | ✗ |

### textDocument/references

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **12.0ms** | **52.8 MB** | 24 references | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.5 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 524.7 MB | empty | ✗ |

### textDocument/completion

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **0.2ms** | **54.0 MB** | 28 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) | ✓ |
| **solc** | - | 25.8 MB | unsupported | ✗ |
| **qiuxiang** | 0.3ms | 71.5 MB | 7 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) | ✓ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 16.0ms | 525.1 MB | 7 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) | ✓ |

### textDocument/signatureHelp

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **11.7ms** | **53.3 MB** | function getTickAtSqrtPrice(uint160 sqrtPriceX96) ... | ✓ |
| **solc** | - | 25.8 MB | unsupported | ✗ |
| **qiuxiang** | - | 72.2 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 524.7 MB | empty | ✗ |

### textDocument/rename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **20.1ms** | **53.6 MB** | 13 edits in 1 files | ✓ |
| **solc** | - | 25.9 MB | error | ✗ |
| **qiuxiang** | 0.3ms | 72.3 MB | 0 edits in 0 files | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 0.6ms | 526.5 MB | 0 edits in 0 files | ✗ |

### textDocument/prepareRename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **0.2ms** | **53.2 MB** | ready (line 102) | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 526.3 MB | unsupported | ✗ |

### textDocument/documentSymbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **2.5ms** | **53.0 MB** | 16 symbols | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 29.8ms | 524.0 MB | 1 symbols | ✓ |

### textDocument/documentLink

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **0.8ms** | **52.9 MB** | 14 links | ✓ |
| **solc** | - | 25.8 MB | unsupported | ✗ |
| **qiuxiang** | - | 72.0 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 523.4 MB | unsupported | ✗ |

### textDocument/formatting

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **19.5ms** | **54.1 MB** | 1 edits | ✓ |
| **solc** | 276.3ms | 26.1 MB | {"error":"Unknown method textDocument/fo... | ✗ |
| **qiuxiang** | 2.0ms | 71.3 MB | {"error":"Request textDocument/formattin... | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 430.4ms | 525.1 MB | 1 edits | ✓ |

### textDocument/inlayHint

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **2.7ms** | **52.5 MB** | 114 hints (value1:, value2:, value:) | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.5 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 527.5 MB | unsupported | ✗ |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **3.7ms** | **52.7 MB** | 697 tokens | ✓ |
| **solc** | - | 26.0 MB | error | ✗ |
| **qiuxiang** | - | 71.4 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 28.4ms | 524.6 MB | 82 tokens | ✓ |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **2.5ms** | **53.6 MB** | 274 tokens | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.3 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 525.0 MB | unsupported | ✗ |

### workspace/symbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **2.2ms** | **52.7 MB** | 68 symbols | ✓ |
| **solc** | - | 25.8 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.1 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 526.0 MB | unsupported | ✗ |

---

*Benchmark run: 2026-02-22T02:05:35Z*
