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
| 0.1.24 | `solidity-language-server 0.1.24+commit.a347b33.macos.aarch64` |
| 0.1.23 | `solidity-language-server 0.1.24+commit.a347b33.macos.aarch64` |

---

## Summary

| Method | 0.1.24 | 0.1.23 |
|--------|--------|--------|
| [initialize](#initialize) | **22.1ms** | 31.4ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 149.2ms | **94.5ms** |
| [textDocument/definition](#textdocumentdefinition) | **2.8ms** | 5.3ms |
| [textDocument/declaration](#textdocumentdeclaration) | 2.5ms | **1.7ms** |
| [textDocument/hover](#textdocumenthover) | 3.4ms | **3.3ms** |
| [textDocument/references](#textdocumentreferences) | **3.4ms** | 3.7ms |
| [textDocument/completion](#textdocumentcompletion) | **0.1ms** | **0.1ms** |
| [textDocument/rename](#textdocumentrename) | **3.4ms** | 4.2ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | **0.1ms** | 0.2ms |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | **1.2ms** | 1.2ms |
| [textDocument/formatting](#textdocumentformatting) | **13.1ms** | 16.6ms |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | **1.6ms** | **1.6ms** |
| [workspace/symbol](#workspacesymbol) | **1.1ms** | 1.1ms |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **0.1.24** | **10** | **13** |
| 0.1.23 | 5 | 13 |

---

## Results

### initialize

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **22.1ms** | - | ok | ✓ |
| **0.1.23** | 31.4ms | - | ok | ✓ |

### textDocument/diagnostic

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | 149.2ms | 13.0 MB | 1 diagnostics | ✓ |
| **0.1.23** | **94.5ms** | **12.9 MB** | 1 diagnostics | ✓ |

### textDocument/definition

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **2.8ms** | **12.7 MB** | `Shop.sol:68` | ✓ |
| **0.1.23** | 5.3ms | 12.7 MB | `Shop.sol:68` | ✓ |

### textDocument/declaration

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | 2.5ms | 12.8 MB | `Shop.sol:68` | ✓ |
| **0.1.23** | **1.7ms** | **12.7 MB** | `Shop.sol:68` | ✓ |

### textDocument/hover

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | 3.4ms | 12.9 MB | uint256 internal immutable PRICE | ✓ |
| **0.1.23** | **3.3ms** | **12.8 MB** | uint256 internal immutable PRICE | ✓ |

### textDocument/references

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **3.4ms** | **12.8 MB** | 4 references | ✓ |
| **0.1.23** | 3.7ms | 12.8 MB | 4 references | ✓ |

### textDocument/completion

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **0.1ms** | 12.9 MB | 5 items (buyer, nonce, amount) | ✓ |
| **0.1.23** | **0.1ms** | **12.9 MB** | 5 items (buyer, nonce, amount) | ✓ |

### textDocument/rename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **3.4ms** | 12.8 MB | 4 edits in 1 files | ✓ |
| **0.1.23** | 4.2ms | **12.7 MB** | 4 edits in 1 files | ✓ |

### textDocument/prepareRename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **0.1ms** | 13.0 MB | ready (line 136) | ✓ |
| **0.1.23** | 0.2ms | **12.9 MB** | ready (line 136) | ✓ |

### textDocument/documentSymbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **1.2ms** | 12.9 MB | 3 symbols | ✓ |
| **0.1.23** | 1.2ms | **12.9 MB** | 3 symbols | ✓ |

### textDocument/formatting

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **13.1ms** | 12.7 MB | 1 edits | ✓ |
| **0.1.23** | 16.6ms | **12.7 MB** | 1 edits | ✓ |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **1.6ms** | 12.8 MB | 451 tokens | ✓ |
| **0.1.23** | **1.6ms** | **12.8 MB** | 451 tokens | ✓ |

### workspace/symbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **0.1.24** | **1.1ms** | 12.9 MB | 61 symbols | ✓ |
| **0.1.23** | 1.1ms | **12.7 MB** | 61 symbols | ✓ |

---

*Benchmark run: 2026-02-21T22:47:51Z*
