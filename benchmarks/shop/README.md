# Solidity LSP Competition

Benchmarked against `example` — `Shop.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `Shop.sol` |
| Position | line 136, col 32 |
| Iterations | 10 (2 warmup) |
| Timeout | 10s |

## Servers

| Server | Version |
|--------|---------|
| [mmsaki v0.1.24](https://github.com/mmsaki/solidity-language-server) | `solidity-language-server 0.1.24+commit.9746134.macos.aarch64` |
| [solc](https://docs.soliditylang.org) | `0.8.26+commit.8a97fa7a.Darwin.appleclang` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | `solidity-ls 0.5.4` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | `vscode-solidity-server 0.0.187` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | `@nomicfoundation/solidity-language-server 0.8.25` |

---

## Summary

| Method | [mmsaki v0.1.24](https://github.com/mmsaki/solidity-language-server) solidity-language-server 0.1.24+commit.9746134.macos.aarch64 | [solc](https://docs.soliditylang.org) 0.8.26+commit.8a97fa7a.Darwin.appleclang | [qiuxiang](https://github.com/qiuxiang/solidity-ls) solidity-ls 0.5.4 | [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) vscode-solidity-server 0.0.187 | [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) @nomicfoundation/solidity-language-server 0.8.25 |
|--------|----------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------|-----------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| [initialize](#initialize) | 7.0ms ⚡ | 116.6ms | 137.3ms | 570.2ms | 854.0ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 78.7ms | 2.4ms ⚡ | 216.8ms | 872.8ms | 512.9ms |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 1.5ms ⚡ | error | unsupported | unsupported | unsupported |
| [textDocument/definition](#textdocumentdefinition) | 2.9ms | 2.3ms | 2.4ms ⚡ | 55.2ms | 1.4ms |
| [textDocument/declaration](#textdocumentdeclaration) | 1.5ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/hover](#textdocumenthover) | 4.2ms ⚡ | crash | 2.1ms | 53.1ms | 1.4ms |
| [textDocument/references](#textdocumentreferences) | 2.3ms | 2.3ms | 2.5ms | 73.2ms | 1.5ms ⚡ |
| [textDocument/completion](#textdocumentcompletion) | 1.0ms ⚡ | 2.4ms | 2.3ms | 53.2ms | 37.5ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 2.6ms ⚡ | unsupported | empty | empty | empty |
| [textDocument/rename](#textdocumentrename) | 3.9ms | 2.3ms | 2.6ms | 50.4ms | 1.7ms ⚡ |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 1.2ms ⚡ | unsupported | unsupported | 13.7ms | 14.4ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | empty | unsupported | unsupported | unsupported | unsupported |
| [textDocument/formatting](#textdocumentformatting) | 11.1ms ⚡ | 2.3ms | 1.8ms | 62.1ms | 189.1ms |
| [textDocument/inlayHint](#textdocumentinlayhint) | 1.5ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 1.6ms ⚡ | error | unsupported | unsupported | 13.8ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 1.0ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [workspace/symbol](#workspacesymbol) | 1.1ms ⚡ | unsupported | unsupported | timeout | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki v0.1.24** | **13** | **18** |
| nomicfoundation | 2 | 18 |
| solc | 1 | 18 |
| qiuxiang | 1 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 7.0ms ⚡ | - | ok | ✓ |
| **solc** | 116.6ms | - | ok | ✓ |
| **qiuxiang** | 137.3ms | - | ok | ✓ |
| **juanfranblanco** | 570.2ms | - | ok | ✓ |
| **nomicfoundation** | 854.0ms | - | ok | ✓ |

### textDocument/diagnostic

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 78.7ms | **12.8 MB** | 1 diagnostics | ✓ |
| **solc** | 2.4ms ⚡ | 26.2 MB | 0 diagnostics | ✓ |
| **qiuxiang** | 216.8ms | 64.6 MB | 0 diagnostics | ✓ |
| **juanfranblanco** | 872.8ms | 438.2 MB | 0 diagnostics | ✓ |
| **nomicfoundation** | 512.9ms | 412.8 MB | 0 diagnostics | ✓ |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.5ms ⚡ | **12.6 MB** | delta | ✓ |
| **solc** | - | 26.1 MB | error | ✗ |
| **qiuxiang** | - | 64.0 MB | unsupported | ✗ |
| **juanfranblanco** | - | 438.3 MB | unsupported | ✗ |
| **nomicfoundation** | - | 413.5 MB | unsupported | ✗ |

### textDocument/definition

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 2.9ms | **12.8 MB** | `Shop.sol:68` | ✓ |
| **solc** | 2.3ms | 25.8 MB | empty | ✗ |
| **qiuxiang** | 2.4ms ⚡ | 64.5 MB | `Shop.sol:121` | ✓ |
| **juanfranblanco** | 55.2ms | 427.5 MB | `Shop.sol:68` | ✓ |
| **nomicfoundation** | 1.4ms | 408.6 MB | empty | ✗ |

### textDocument/declaration

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.5ms ⚡ | **12.6 MB** | `Shop.sol:68` | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 64.5 MB | unsupported | ✗ |
| **juanfranblanco** | - | 438.0 MB | unsupported | ✗ |
| **nomicfoundation** | - | 412.9 MB | unsupported | ✗ |

### textDocument/hover

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 4.2ms ⚡ | **12.5 MB** | function addTax(uint256 amount, uint16 tax, uint16... | ✓ |
| **solc** | - | 26.1 MB | crash | ✗ |
| **qiuxiang** | 2.1ms | 64.3 MB | empty | ✗ |
| **juanfranblanco** | 53.1ms | 426.8 MB | ### Function: addTax | ✓ |
| **nomicfoundation** | 1.4ms | 409.8 MB | empty | ✗ |

### textDocument/references

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 2.3ms | **12.7 MB** | 11 references | ✓ |
| **solc** | 2.3ms | 26.1 MB | {"error":"Unknown method textDocument/re... | ✗ |
| **qiuxiang** | 2.5ms | 64.4 MB | 2 references | ✓ |
| **juanfranblanco** | 73.2ms | 433.1 MB | 42 references | ✓ |
| **nomicfoundation** | 1.5ms ⚡ | 408.8 MB | 11 references | ✓ |

### textDocument/completion

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.0ms ⚡ | **12.6 MB** | 5 items (buyer, nonce, amount) | ✓ |
| **solc** | 2.4ms | 25.8 MB | {"error":"Unknown method textDocument/co... | ✗ |
| **qiuxiang** | 2.3ms | 64.8 MB | 3 items (data, sender, sig) | ✓ |
| **juanfranblanco** | 53.2ms | 428.1 MB | 0 items | ✗ |
| **nomicfoundation** | 37.5ms | 410.0 MB | empty | ✗ |

### textDocument/signatureHelp

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 2.6ms ⚡ | **12.6 MB** | function addTax(uint256 amount, uint16 tax, uint16... | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 64.6 MB | empty | ✗ |
| **juanfranblanco** | - | 427.1 MB | empty | ✗ |
| **nomicfoundation** | - | 411.5 MB | empty | ✗ |

### textDocument/rename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 3.9ms | **12.7 MB** | 3 edits in 1 files | ✓ |
| **solc** | 2.3ms | 26.0 MB | {"error":"Unhandled exception: /solidity... | ✗ |
| **qiuxiang** | 2.6ms | 64.7 MB | 2 edits in 1 files | ✓ |
| **juanfranblanco** | 50.4ms | 433.9 MB | {"error":"Unhandled method textDocument/... | ✗ |
| **nomicfoundation** | 1.7ms ⚡ | 413.4 MB | 11 edits in 1 files | ✓ |

### textDocument/prepareRename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 0.2ms ⚡ | **12.7 MB** | ready (line 136) | ✓ |
| **solc** | - | 26.2 MB | unsupported | ✗ |
| **qiuxiang** | - | 64.4 MB | unsupported | ✗ |
| **juanfranblanco** | - | 425.9 MB | unsupported | ✗ |
| **nomicfoundation** | - | 410.7 MB | unsupported | ✗ |

### textDocument/documentSymbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.2ms ⚡ | **12.5 MB** | 3 symbols | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 64.3 MB | unsupported | ✗ |
| **juanfranblanco** | 13.7ms | 426.9 MB | 2 symbols | ✓ |
| **nomicfoundation** | 14.4ms | 407.0 MB | 2 symbols | ✓ |

### textDocument/documentLink

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | - | 12.8 MB | empty | ✗ |
| **solc** | - | 26.2 MB | unsupported | ✗ |
| **qiuxiang** | - | 64.4 MB | unsupported | ✗ |
| **juanfranblanco** | - | 438.5 MB | unsupported | ✗ |
| **nomicfoundation** | - | 412.0 MB | unsupported | ✗ |

### textDocument/formatting

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 11.1ms ⚡ | **12.5 MB** | 1 edits | ✓ |
| **solc** | 2.3ms | 26.1 MB | {"error":"Unknown method textDocument/fo... | ✗ |
| **qiuxiang** | 1.8ms | 64.4 MB | {"error":"Request textDocument/formattin... | ✗ |
| **juanfranblanco** | 62.1ms | 432.6 MB | {"error":"Unhandled method textDocument/... | ✗ |
| **nomicfoundation** | 189.1ms | 412.2 MB | 1 edits | ✓ |

### textDocument/inlayHint

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.5ms ⚡ | **12.7 MB** | 24 hints (tax:, base:, buyer:) | ✓ |
| **solc** | - | 26.2 MB | unsupported | ✗ |
| **qiuxiang** | - | 64.3 MB | unsupported | ✗ |
| **juanfranblanco** | - | 433.0 MB | unsupported | ✗ |
| **nomicfoundation** | - | 408.3 MB | unsupported | ✗ |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.6ms ⚡ | **12.7 MB** | 451 tokens | ✓ |
| **solc** | - | 25.9 MB | error | ✗ |
| **qiuxiang** | - | 64.4 MB | unsupported | ✗ |
| **juanfranblanco** | - | 435.2 MB | unsupported | ✗ |
| **nomicfoundation** | 13.8ms | 408.9 MB | 56 tokens | ✓ |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.0ms ⚡ | **12.6 MB** | 160 tokens | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 64.5 MB | unsupported | ✗ |
| **juanfranblanco** | - | 425.7 MB | unsupported | ✗ |
| **nomicfoundation** | - | 408.1 MB | unsupported | ✗ |

### workspace/symbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.1ms ⚡ | **12.7 MB** | 61 symbols | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 64.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | 418.3 MB | timeout | ✗ |
| **nomicfoundation** | - | 409.3 MB | unsupported | ✗ |

---

*Benchmark run: 2026-02-22T03:42:38Z*
