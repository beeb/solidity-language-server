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
| v0.1.25 | `0.1.25` |
| 0.1.24 | `0.1.24` |

---

## Summary

| Method | v0.1.25 | 0.1.24 |
|--------|---------|--------|
| [initialize](#initialize) | 34.4ms ⚡ | 38.5ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 70.7ms ⚡ | 74.8ms |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 1.5ms ⚡ | 8.7ms |
| [textDocument/definition](#textdocumentdefinition) | 3.2ms | 2.5ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 0.2ms ⚡ | 1.5ms |
| [textDocument/hover](#textdocumenthover) | 0.9ms ⚡ | 3.0ms |
| [textDocument/references](#textdocumentreferences) | 0.4ms ⚡ | 2.0ms |
| [textDocument/completion](#textdocumentcompletion) | 0.1ms | 0.1ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 0.9ms ⚡ | 2.7ms |
| [textDocument/rename](#textdocumentrename) | 0.6ms ⚡ | 3.1ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.1ms ⚡ | 0.1ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 1.2ms | 1.1ms ⚡ |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 1.1ms ⚡ | unsupported |
| [textDocument/documentLink](#textdocumentdocumentlink) | empty | empty |
| [textDocument/formatting](#textdocumentformatting) | 10.8ms | 10.2ms ⚡ |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 1.0ms ⚡ | unsupported |
| [textDocument/inlayHint](#textdocumentinlayhint) | 1.4ms | 1.4ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 1.5ms ⚡ | 1.5ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 1.0ms | 0.9ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 1.0ms | 1.0ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **v0.1.25** | **12** | **20** |
| 0.1.24 | 8 | 20 |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 34.4ms ⚡ | - | ok |
| **0.1.24** | 38.5ms | - | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 70.7ms ⚡ | 13.8 MB | 1 diagnostics |
| **0.1.24** | 74.8ms | **12.9 MB** | 1 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 1.5ms ⚡ | 13.9 MB | delta |
| **0.1.24** | 8.7ms | **12.7 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 3.2ms | 13.9 MB | `Shop.sol:68` |
| **0.1.24** | 2.5ms ⚡ | **12.8 MB** | `Shop.sol:68` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 0.2ms ⚡ | 13.7 MB | `Shop.sol:68` |
| **0.1.24** | 1.5ms | **12.8 MB** | `Shop.sol:68` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 0.9ms ⚡ | 13.9 MB | uint256 internal immutable PRICE |
| **0.1.24** | 3.0ms | **12.6 MB** | uint256 internal immutable PRICE |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 0.4ms ⚡ | 13.7 MB | 4 references |
| **0.1.24** | 2.0ms | **12.6 MB** | 4 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 0.1ms | 13.7 MB | 5 items (buyer, nonce, amount) |
| **0.1.24** | 0.1ms ⚡ | **12.7 MB** | 5 items (buyer, nonce, amount) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 0.9ms ⚡ | 13.7 MB | function addTax(uint256 amount, uint16 tax, uint16... |
| **0.1.24** | 2.7ms | **12.8 MB** | function addTax(uint256 amount, uint16 tax, uint16... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 0.6ms ⚡ | 13.7 MB | 4 edits in 1 files |
| **0.1.24** | 3.1ms | **12.8 MB** | 4 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 0.1ms ⚡ | 13.6 MB | ready (line 136) |
| **0.1.24** | 0.1ms ⚡ | **12.8 MB** | ready (line 136) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 1.2ms | 13.8 MB | 3 symbols |
| **0.1.24** | 1.1ms ⚡ | **12.7 MB** | 3 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 1.1ms ⚡ | **13.6 MB** | [{"kind":3,"range":{"end":{"character":2... |
| **0.1.24** | - | 12.8 MB | unsupported |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | - | 13.9 MB | empty |
| **0.1.24** | - | 12.7 MB | empty |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 10.8ms | 13.7 MB | 1 edits |
| **0.1.24** | 10.2ms ⚡ | **12.7 MB** | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 1.0ms ⚡ | **13.7 MB** | [{"endCharacter":1,"endLine":53,"startCh... |
| **0.1.24** | - | 12.7 MB | unsupported |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 1.4ms | 13.6 MB | 24 hints (tax:, base:, buyer:) |
| **0.1.24** | 1.4ms ⚡ | **12.6 MB** | 24 hints (tax:, base:, buyer:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 1.5ms ⚡ | 13.6 MB | 451 tokens |
| **0.1.24** | 1.5ms | **12.6 MB** | 451 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 1.0ms | 13.7 MB | 160 tokens |
| **0.1.24** | 0.9ms ⚡ | **12.7 MB** | 160 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **v0.1.25** | 1.0ms | 13.7 MB | 61 symbols |
| **0.1.24** | 1.0ms ⚡ | **12.6 MB** | 61 symbols |

---

*Benchmark run: 2026-02-23T22:16:43Z*
