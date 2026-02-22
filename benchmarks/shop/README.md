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
| [mmsaki latest](https://github.com/mmsaki/solidity-language-server) | `solidity-language-server 0.1.24+commit.9746134.macos.aarch64` |
| [solc](https://docs.soliditylang.org) | `0.8.26+commit.8a97fa7a.Darwin.appleclang` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | `solidity-ls 0.5.4` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | `vscode-solidity-server 0.0.187` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | `@nomicfoundation/solidity-language-server 0.8.25` |

---

## Summary

| Method | mmsaki latest | solc | qiuxiang | juanfranblanco | nomicfoundation |
|--------|---------------|------|----------|----------------|-----------------|
| [initialize](#initialize) | **12.2ms** | 122.5ms | 93.0ms | 576.9ms | 846.0ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 81.8ms | **2.7ms** | 275.8ms | 771.4ms | 506.3ms |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | **1.5ms** | error | unsupported | unsupported | unsupported |
| [textDocument/definition](#textdocumentdefinition) | **3.2ms** | 2.5ms | 21.1ms | 70.0ms | 1.4ms |
| [textDocument/declaration](#textdocumentdeclaration) | **1.4ms** | unsupported | unsupported | unsupported | unsupported |
| [textDocument/hover](#textdocumenthover) | **4.4ms** | crash | 20.0ms | 67.2ms | 1.5ms |
| [textDocument/references](#textdocumentreferences) | 2.2ms | 2.3ms | 21.0ms | 71.2ms | **1.8ms** |
| [textDocument/completion](#textdocumentcompletion) | **0.2ms** | 2.3ms | 20.2ms | 55.1ms | 37.5ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | **2.7ms** | unsupported | empty | empty | empty |
| [textDocument/rename](#textdocumentrename) | 4.0ms | 2.7ms | 22.0ms | 66.0ms | **1.9ms** |
| [textDocument/prepareRename](#textdocumentpreparerename) | **0.2ms** | unsupported | unsupported | unsupported | unsupported |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | **1.2ms** | unsupported | unsupported | 6.8ms | 16.0ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | empty | unsupported | unsupported | unsupported | unsupported |
| [textDocument/formatting](#textdocumentformatting) | **12.7ms** | 2.5ms | 18.6ms | 67.7ms | 175.7ms |
| [textDocument/inlayHint](#textdocumentinlayhint) | **1.9ms** | unsupported | unsupported | unsupported | unsupported |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | **1.6ms** | error | unsupported | unsupported | 14.3ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | **1.0ms** | unsupported | unsupported | unsupported | unsupported |
| [workspace/symbol](#workspacesymbol) | **1.1ms** | unsupported | unsupported | timeout | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki latest** | **14** | **18** |
| nomicfoundation | 2 | 18 |
| solc | 1 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **12.2ms** | - | ok | ✓ |
| **solc** | 122.5ms | - | ok | ✓ |
| **qiuxiang** | 93.0ms | - | ok | ✓ |
| **juanfranblanco** | 576.9ms | - | ok | ✓ |
| **nomicfoundation** | 846.0ms | - | ok | ✓ |

### textDocument/diagnostic

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | 81.8ms | 12.8 MB | 1 diagnostics | ✓ |
| **solc** | **2.7ms** | 26.2 MB | 0 diagnostics | ✓ |
| **qiuxiang** | 275.8ms | 6.7 MB | 0 diagnostics | ✓ |
| **juanfranblanco** | 771.4ms | **6.6 MB** | 0 diagnostics | ✓ |
| **nomicfoundation** | 506.3ms | 6.6 MB | 0 diagnostics | ✓ |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **1.5ms** | **12.6 MB** | delta | ✓ |
| **solc** | - | 26.0 MB | error | ✗ |
| **qiuxiang** | - | 6.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | 6.6 MB | unsupported | ✗ |
| **nomicfoundation** | - | 6.6 MB | unsupported | ✗ |

### textDocument/definition

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **3.2ms** | 12.6 MB | `Shop.sol:68` | ✓ |
| **solc** | 2.5ms | 25.9 MB | empty | ✗ |
| **qiuxiang** | 21.1ms | 6.6 MB | `Shop.sol:121` | ✓ |
| **juanfranblanco** | 70.0ms | **6.6 MB** | `Shop.sol:68` | ✓ |
| **nomicfoundation** | 1.4ms | 6.5 MB | empty | ✗ |

### textDocument/declaration

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **1.4ms** | **12.7 MB** | `Shop.sol:68` | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 6.7 MB | unsupported | ✗ |
| **juanfranblanco** | - | 6.6 MB | unsupported | ✗ |
| **nomicfoundation** | - | 6.5 MB | unsupported | ✗ |

### textDocument/hover

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **4.4ms** | 12.7 MB | function addTax(uint256 amount, uint16 tax, uint16... | ✓ |
| **solc** | - | 26.0 MB | crash | ✗ |
| **qiuxiang** | 20.0ms | 6.7 MB | empty | ✗ |
| **juanfranblanco** | 67.2ms | **6.6 MB** | ### Function: addTax | ✓ |
| **nomicfoundation** | 1.5ms | 6.6 MB | empty | ✗ |

### textDocument/references

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | 2.2ms | 12.7 MB | 11 references | ✓ |
| **solc** | 2.3ms | 25.9 MB | {"error":"Unknown method textDocument/re... | ✗ |
| **qiuxiang** | 21.0ms | 6.6 MB | 2 references | ✓ |
| **juanfranblanco** | 71.2ms | 6.6 MB | 42 references | ✓ |
| **nomicfoundation** | **1.8ms** | **6.6 MB** | 11 references | ✓ |

### textDocument/completion

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **0.2ms** | 12.7 MB | 5 items (buyer, nonce, amount) | ✓ |
| **solc** | 2.3ms | 26.0 MB | {"error":"Unknown method textDocument/co... | ✗ |
| **qiuxiang** | 20.2ms | **6.7 MB** | 3 items (data, sender, sig) | ✓ |
| **juanfranblanco** | 55.1ms | 6.5 MB | 0 items | ✗ |
| **nomicfoundation** | 37.5ms | 6.6 MB | empty | ✗ |

### textDocument/signatureHelp

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **2.7ms** | **12.7 MB** | function addTax(uint256 amount, uint16 tax, uint16... | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 6.6 MB | empty | ✗ |
| **juanfranblanco** | - | 6.6 MB | empty | ✗ |
| **nomicfoundation** | - | 6.6 MB | empty | ✗ |

### textDocument/rename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | 4.0ms | 12.7 MB | 3 edits in 1 files | ✓ |
| **solc** | 2.7ms | 26.1 MB | {"error":"Unhandled exception: /solidity... | ✗ |
| **qiuxiang** | 22.0ms | 6.6 MB | 2 edits in 1 files | ✓ |
| **juanfranblanco** | 66.0ms | 6.6 MB | {"error":"Unhandled method textDocument/... | ✗ |
| **nomicfoundation** | **1.9ms** | **6.6 MB** | 11 edits in 1 files | ✓ |

### textDocument/prepareRename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **0.2ms** | **12.6 MB** | ready (line 136) | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 6.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | 6.6 MB | unsupported | ✗ |
| **nomicfoundation** | - | 6.6 MB | unsupported | ✗ |

### textDocument/documentSymbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **1.2ms** | 12.8 MB | 3 symbols | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 6.6 MB | unsupported | ✗ |
| **juanfranblanco** | 6.8ms | **6.6 MB** | 2 symbols | ✓ |
| **nomicfoundation** | 16.0ms | 6.6 MB | 2 symbols | ✓ |

### textDocument/documentLink

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | - | 12.7 MB | empty | ✗ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 6.7 MB | unsupported | ✗ |
| **juanfranblanco** | - | 6.6 MB | unsupported | ✗ |
| **nomicfoundation** | - | 6.6 MB | unsupported | ✗ |

### textDocument/formatting

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **12.7ms** | 12.8 MB | 1 edits | ✓ |
| **solc** | 2.5ms | 26.0 MB | {"error":"Unknown method textDocument/fo... | ✗ |
| **qiuxiang** | 18.6ms | 6.6 MB | {"error":"Request textDocument/formattin... | ✗ |
| **juanfranblanco** | 67.7ms | **6.6 MB** | {"error":"Unhandled method textDocument/... | ✗ |
| **nomicfoundation** | 175.7ms | **6.6 MB** | 1 edits | ✓ |

### textDocument/inlayHint

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **1.9ms** | **12.6 MB** | 24 hints (tax:, base:, buyer:) | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 6.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | 6.6 MB | unsupported | ✗ |
| **nomicfoundation** | - | 6.6 MB | unsupported | ✗ |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **1.6ms** | 12.6 MB | 451 tokens | ✓ |
| **solc** | - | 26.2 MB | error | ✗ |
| **qiuxiang** | - | 6.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | 6.5 MB | unsupported | ✗ |
| **nomicfoundation** | 14.3ms | **6.6 MB** | 56 tokens | ✓ |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **1.0ms** | **12.7 MB** | 160 tokens | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 6.7 MB | unsupported | ✗ |
| **juanfranblanco** | - | 6.5 MB | unsupported | ✗ |
| **nomicfoundation** | - | 6.6 MB | unsupported | ✗ |

### workspace/symbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki latest** | **1.1ms** | **12.6 MB** | 61 symbols | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 6.7 MB | unsupported | ✗ |
| **juanfranblanco** | - | 6.6 MB | timeout | ✗ |
| **nomicfoundation** | - | 6.6 MB | unsupported | ✗ |

---

*Benchmark run: 2026-02-22T02:27:13Z*
