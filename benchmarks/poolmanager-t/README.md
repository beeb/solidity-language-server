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
| [mmsaki](https://github.com/mmsaki/solidity-language-server) | `0.1.24` |
| [solc](https://docs.soliditylang.org) | `0.8.26` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | `0.5.4` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | `0.0.187` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | `0.8.25` |

---

## Summary

| Method | mmsaki | solc | qiuxiang | juanfranblanco | nomicfoundation |
|--------|--------|------|----------|----------------|-----------------|
| [initialize](#initialize) | 7.8ms ⚡ | 122.4ms | 113.9ms | 546.7ms | 862.3ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 2.3s | 154.4ms ⚡ | 208.6ms | crash | 2.6s |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 9.8ms ⚡ | error | unsupported | crash | unsupported |
| [textDocument/definition](#textdocumentdefinition) | 139.4ms ⚡ | empty | empty | crash | empty |
| [textDocument/declaration](#textdocumentdeclaration) | 133.1ms ⚡ | unsupported | unsupported | crash | unsupported |
| [textDocument/hover](#textdocumenthover) | 239.8ms ⚡ | crash | empty | crash | empty |
| [textDocument/references](#textdocumentreferences) | 136.8ms ⚡ | unsupported | empty | crash | empty |
| [textDocument/completion](#textdocumentcompletion) | 0.7ms ⚡ | unsupported | empty | crash | empty |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 33.5ms ⚡ | unsupported | empty | crash | empty |
| [textDocument/rename](#textdocumentrename) | 266.3ms ⚡ | error | 0.3ms | crash | 0.7ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ | unsupported | unsupported | crash | unsupported |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 6.2ms ⚡ | unsupported | unsupported | crash | 98.7ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | 2.2ms ⚡ | unsupported | unsupported | crash | unsupported |
| [textDocument/formatting](#textdocumentformatting) | 20.4ms ⚡ | 271.0ms | 2.4ms | crash | 992.5ms |
| [textDocument/inlayHint](#textdocumentinlayhint) | 9.1ms ⚡ | unsupported | unsupported | crash | unsupported |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 9.8ms ⚡ | error | unsupported | crash | 95.0ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 6.6ms ⚡ | unsupported | unsupported | crash | unsupported |
| [workspace/symbol](#workspacesymbol) | 5.8ms ⚡ | unsupported | unsupported | crash | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **17** | **18** |
| solc | 1 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 7.8ms ⚡ | - | ok | ✓ |
| **solc** | 122.4ms | - | ok | ✓ |
| **qiuxiang** | 113.9ms | - | ok | ✓ |
| **juanfranblanco** | 546.7ms | - | ok | ✓ |
| **nomicfoundation** | 862.3ms | - | ok | ✓ |

### textDocument/diagnostic

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 2.3s | 228.5 MB | 15 diagnostics | ✓ |
| **solc** | 154.4ms ⚡ | **26.2 MB** | 0 diagnostics | ✓ |
| **qiuxiang** | 208.6ms | 62.7 MB | 1 diagnostics | ✓ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 2.6s | 661.0 MB | 1 diagnostics | ✓ |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 9.8ms ⚡ | **229.0 MB** | delta | ✓ |
| **solc** | - | 26.0 MB | error | ✗ |
| **qiuxiang** | - | 62.8 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 660.4 MB | unsupported | ✗ |

### textDocument/definition

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 139.4ms ⚡ | **226.8 MB** | `TickMath.sol:9` | ✓ |
| **solc** | - | 26.0 MB | empty | ✗ |
| **qiuxiang** | - | 62.6 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 661.4 MB | empty | ✗ |

### textDocument/declaration

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 133.1ms ⚡ | **228.3 MB** | `TickMath.sol:9` | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 660.1 MB | unsupported | ✗ |

### textDocument/hover

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 239.8ms ⚡ | **227.2 MB** | error PoolNotInitialized() | ✓ |
| **solc** | - | 26.0 MB | crash | ✗ |
| **qiuxiang** | - | 62.5 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.8 MB | empty | ✗ |

### textDocument/references

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 136.8ms ⚡ | **228.8 MB** | 7 references | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.7 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.3 MB | empty | ✗ |

### textDocument/completion

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 0.7ms ⚡ | **228.2 MB** | 23 items (amount0, amount1, checkTicks) | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.7 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 660.1 MB | empty | ✗ |

### textDocument/signatureHelp

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 33.5ms ⚡ | **227.9 MB** | function bound(uint256 x, uint256 min, uint256 max... | ✓ |
| **solc** | - | 26.2 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.6 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 657.8 MB | empty | ✗ |

### textDocument/rename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 266.3ms ⚡ | **228.8 MB** | 9 edits in 1 files | ✓ |
| **solc** | - | 26.1 MB | error | ✗ |
| **qiuxiang** | 0.3ms | 62.5 MB | 0 edits in 0 files | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 0.7ms | 660.5 MB | 0 edits in 0 files | ✗ |

### textDocument/prepareRename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 0.2ms ⚡ | **228.2 MB** | ready (line 116) | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.5 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.8 MB | unsupported | ✗ |

### textDocument/documentSymbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 6.2ms ⚡ | **227.2 MB** | 35 symbols | ✓ |
| **solc** | - | 26.2 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.8 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 98.7ms | 660.8 MB | 1 symbols | ✓ |

### textDocument/documentLink

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 2.2ms ⚡ | **227.4 MB** | 33 links | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.4 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.6 MB | unsupported | ✗ |

### textDocument/formatting

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 20.4ms ⚡ | **226.7 MB** | 1 edits | ✓ |
| **solc** | 271.0ms | 26.1 MB | {"error":"Unknown method textDocument/fo... | ✗ |
| **qiuxiang** | 2.4ms | 62.4 MB | {"error":"Request textDocument/formattin... | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 992.5ms | 659.5 MB | 1 edits | ✓ |

### textDocument/inlayHint

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 9.1ms ⚡ | **226.9 MB** | 1080 hints (name:, hooks:, name:) | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.8 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 661.2 MB | unsupported | ✗ |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 9.8ms ⚡ | **227.8 MB** | 1512 tokens | ✓ |
| **solc** | - | 25.9 MB | error | ✗ |
| **qiuxiang** | - | 62.5 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 95.0ms | 657.6 MB | 385 tokens | ✓ |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 6.6ms ⚡ | **228.2 MB** | 417 tokens | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.4 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 658.7 MB | unsupported | ✗ |

### workspace/symbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | 5.8ms ⚡ | **228.0 MB** | 90 symbols | ✓ |
| **solc** | - | 25.7 MB | unsupported | ✗ |
| **qiuxiang** | - | 62.2 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 659.5 MB | unsupported | ✗ |

---

*Benchmark run: 2026-02-22T03:44:59Z*
