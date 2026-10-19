# SoA authority and Quack cutover: CerebroCortex storage renovation

Status: proposed implementation contract; no runtime migration or performance claim.

## Observed baseline

- `StorageCoordinator` currently owns `SqliteStore`, `GraphStore`, and `VectorStore`; SQLite is authoritative.
- `graph_data_version` and `PRAGMA data_version` detect foreign writes and trigger rebuild.
- `SqliteStore` stores memory records, activation metadata, visibility, links and procedural state; its row decoding uses JSON-bearing fields.
- `lance-graph-quack` is a columnar planning surface lowering to `lance-graph-mask-risc`; it is **not** a durable SQLite replacement by itself.
- `AdaWorldAPI/petgraph` is available as a fork. Pin a revision only after verifying API compatibility with the currently used petgraph 0.6 interfaces.

## Target authority

```text
CerebroCortex cognitive facade
  | memory, procedures, scoped recall, consolidation
  v
Typed CognitiveStore contract
  | snapshot/witness + scoped reads + conditional writes
  v
Canonical lance-graph SoA authority / persistence
  | 128-bit identity, version witness, presence/visibility planes
  +--> Quack -> Mask-RISC (selection, eligibility, joins)
  +--> petgraph-compatible graph algorithms (derived view, no independent truth)
  +--> vector/embedding provider (derived, rebuildable)
```

Do not introduce SQL or JSON into the canonical cognitive hot path. JSON is allowed only at external protocol/legacy import boundaries or CI fixtures. Quack must not become a second evaluator.

## Execution sequence

1. **Inventory and equivalence contract.** Enumerate all `SqliteStore` public operations and coordinator call sites, including links, scopes, procedure outcomes, dream stages, transactions, delete/restore/purge and cross-process refresh. Record the exact semantics and test fixtures. Do not implement a partial store under the old name.
2. **Typed adapter seam.** Introduce an object-safe or generic `CognitiveStore` contract without moving authority. Keep the SQLite implementation and existing public APIs working. Explicitly separate persistent cognitive state from transport/configuration and embedding caches.
3. **Identity bridge.** Map legacy string `MemoryId` to canonical thought identity with reversible provenance; version/witness is not the identity. Reject collisions, stale writes, and ambiguous migrations. No text IDs in resident hot lanes.
4. **SoA schema and persistence contract.** Specify the canonical SoA lanes for memory metadata, links, visibility, ACT-R/FSRS state, procedural outcomes and tombstones. Identify the *actual* durable substrate and its atomic snapshot/commit semantics before implementing writes; Quack is only the query path.
5. **Read-only Quack adapter.** Implement scoped candidate filtering, typed ordering, retrieval and link lookup over snapshots; use Quack/Mask-RISC only for execution. Diff results against SQLite baseline, including zero/50+ candidates and unauthorized neighbors.
6. **Petgraph bridge.** Use the fork only after checking compatible graph APIs. Keep petgraph as derived graph algorithms / differential oracle. Avoid duplicate mutable graph authority; compare spreading activation and trace events, including invisible bridges.
7. **Write path and recovery.** Implement conditional witnessed writes, atomic multi-record updates, tombstones, purge/restore, multi-process refresh, restart/replay and rollback. Prevent stale snapshots and unauthorized side effects.
8. **Cutover.** Gated dual-read comparison, one-way migration with recorded provenance and backup, then authoritative SoA. Remove SQLite authority only after measured parity and recovery tests; keep an opt-in rollback until confidence is established.

## Required acceptance gates

- Visibility is enforced **before** candidate selection and graph spreading, including seeds, joins, traces and derived scores. No cross-scope influence.
- ACT-R, FSRS, ranking, link-decay and procedural/dream outcomes match the pinned baseline within declared tolerances.
- Stable memory identity across migration, restart, graph view, version updates and snapshots.
- Deterministic tie-breaking/replay or explicit nondeterminism annotations; time is injectable for link-decay replay.
- Multi-process consistency and atomic commit behavior replace the SQLite `data_version` mechanism with a tested snapshot/witness equivalent.
- Numeric precision/quantization changes are separately benchmarked, not smuggled into the backend swap.
- External effects require independent authorization; no LLM-generated hypothesis becomes evidence automatically.
- Cold/warm latency, throughput, allocation, and memory-footprint comparisons against the same fixtures.

## First follow-up PR

Inventory the `SqliteStore` operation surface and introduce a **read-only typed store contract** with the existing SQLite adapter unchanged. Add golden tests for scope and identity. Do not replace dependencies or persistent writes in that first PR.

## Non-goals

No premature deletion of SQLite, no SQLite-via-Quack emulation, no parallel query interpreter, no speculative claim that the petgraph fork has already been wired, and no new unverified consciousness claims.
