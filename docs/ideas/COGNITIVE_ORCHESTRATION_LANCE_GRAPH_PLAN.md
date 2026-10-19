# Cognitive orchestration on lance-graph: CerebroCortex-RS exploration plan

> **Status:** Design proposal only. No integration or performance claims are made by this document.
>
> **Target:** This fork's cognitive-memory library and optional adapters to the public
> [AdaWorldAPI/lance-graph](https://github.com/AdaWorldAPI/lance-graph) workspace.
>
> **Strategy:** Preserve the existing behavioral reference as an oracle. Prove narrow
> equivalent slices before replacing storage, indexing, or the LLM-oriented pipeline.

## 1. Why explore this

CerebroCortex-RS already supplies useful cognitive-memory semantics:
ACT-R-inspired base-level activation, FSRS retrievability, association
spreading, episodic memory, procedural outcomes, visibility scopes, and
offline consolidation. The question is not whether to rename graph operations
after brain regions. It is whether these **measurable mechanisms** can be
implemented over a common columnar, addressable relational substrate while
preserving their meaning.

Proposed integration:

~~~text
external events / goals                    optional adapters
      |                                     Rig / graph-flow / z8run
      v                                            |
CerebroCortex cognitive API <----------------------+
  | attention, episodic recall, procedural selection, consolidation
  v
ACT-R-inspired control loop
  | production eligibility -> arbitration -> selected transition
  v
lance-graph relational compiler / execution
  | relalg / R2IL / Datalog -> Quack / mask-risc
  v
canonical SoA planes + addressed thoughts + version witnesses
  ^
  | mathematical evaluation and experimental calibration
JC + ndarray
~~~

This diagram proposes contracts, **not** current call relationships. The
source-of-truth SoA format, witness semantics, and cross-repository API will
need explicit agreements before implementation.

## 2. Verified starting point (current fork)

- `crates/cerebro/src/activation/actr.rs` implements ACT-R-inspired
  base-level activation `ln(sum(t_k^-d))`, with tests against known values.
- `activation/mod.rs` combines activation, associative contribution,
  FSRS retrievability, salience and vector similarity for recall ranking.
- `engines/prefrontal.rs` ranks retrieved memory candidates, while
  `engines/cerebellum.rs` is still a thin structural wrapper. Procedural
  grading, selection, and variation live partly in the coordinator, storage,
  and dream machinery rather than a standalone production-rule engine.
- `activation/spreading.rs` traverses a `petgraph` cache, with explicit
  visibility filtering and trace events.
- `storage/sqlite.rs`, `storage/vector.rs`, and `storage/graph.rs`
  currently use SQLite/FTS5/sqlite-vec and derived graph indices.
- `engines/dream.rs` contains an eight-phase offline consolidation flow,
  including LLM-assisted hypotheses and procedural variation/competition.
- Existing MCP, REST, CLI, and Lucida surfaces are useful external adapters,
  not candidates for immediate removal.

In the target workspace, `lance-graph-relalg` is an IR/rewrites/lowering
layer; `lance-graph-r2il` is a lowering target; `lance-graph-quack` is
**already a lance-graph crate**, a columnar query surface that lowers
operations to `lance-graph-mask-risc`. Mask-RISC is the sole bulk evaluator
over `ndarray::simd`, not a second SQL engine or an imported DuckDB runtime.
`jc` contains independent mathematical and statistical validations.

### What is *not* implemented today

- A full ACT-R production system (buffer matching, procedural conflict
  resolution, firing, and ACT-R-like timing) is not established by the
  presence of ACT-R **memory activation** alone.
- CerebroCortex memory IDs are currently string-backed UUIDs, **not**
  interchangeable with the thought-address/version contract.
- Neither common SoA authority nor native masked retrieval has been wired
  into this fork.
- No claim of biological fidelity, consciousness, or empirical cognitive
  advantage follows from adopting anatomical module names.

## 3. Design decisions to test

### 3.1 Keep one source of truth

The proposed target makes addressed, versioned SoA state authoritative for
**cognitive facts and executable thought state**. CerebroCortex owns the
*behavioral API*; it does not retain a competing canonical relationship graph.

Early milestones may continue using SQLite as the baseline and compatibility
store. Migrating persistency is a separate, gated step: do not silently
translate a SQLite row ID into the identity of a thought. Operational state,
credentials, transport caches, and user-interface metadata need not share
the same authority as cognition.

Introduce a typed binding between legacy `MemoryId` and a canonical
`ThoughtAddr` plus `WitnessVersion`. Map identities explicitly, with a
recorded migration provenance and an unambiguous reverse lookup. Neither the
version nor a transient row offset is an alternative primary identity.

### 3.2 Cognitive semantics versus execution algebra

| Concern | Owner | Contract |
| --- | --- | --- |
| Memory types, attention, goals and retrieval behavior | CerebroCortex facade | Stable typed operations and scoped outcomes |
| Eligibility of a production | relalg / Datalog lowering | Typed conditions over resident planes/masks |
| Execution of selection predicates | Quack + Mask-RISC | Lower to the **one** masking evaluator, no duplicate interpreter |
| Production selection and state transition | ACT-R-inspired controller | Explicit utility, activation, tie-breaking and timing policy |
| Truth/evidence revision | lance-graph NAL/causal components | Not confused with accessibility or action utility |
| SIMD numeric work | ndarray | Preserve prescribed substrate and arithmetic contract |
| Reliability, calibration and uncertainty | JC | Recorded assumptions and falsifiable measurements |
| Model and external-tool effects | Optional Rig adapter | Authorized, isolated, replay-identifiable external calls |
| Long-running workflow, approvals and pauses | Optional graph-flow adapter | Session/task coordination, not cognitive state authority |
| Visual triggers and service automation | Optional z8run adapter | Outer workflow boundary, not hot cognitive execution |

No SQL is required in the cognitive hot path. If SQL is useful to inspect
or query cognitive state, the interface belongs through
`lance-graph-quack`'s existing planner surface. Do not build a separate
SQLite virtual-table evaluator or duplicate a query interpreter.

### 3.3 Memory activation is not epistemic truth

Keep **three** quantities separately represented:

1. *Evidence/truth*: how well a proposition is supported (e.g. NAL).
2. *Activation*: how accessible a memory is under ACT-R-style decay/retrieval.
3. *Utility*: how useful a procedural action appears under measured outcomes.

A well-supported fact may be hard to retrieve. A frequent memory may be
wrong. A historically effective procedure may be inappropriate now. Any
quantization into compact registers must document error bounds and preserve
this separation.

**Motivation is a fourth, optional signal.** An OpenPsi- or
Ada-rs-style goal/drive provider may propose priorities, but it cannot
silently override evidence quality, visibility checks, or effect permissions.
That provider must be replaceable and is not a prerequisite for ACT-R
production matching.

### 3.4 A bounded active workspace, not a bounded knowledge store

Prototype a **maximum of 64 active thought slots**, configurable as an
experimental policy. This is not a claim that ACT-R specifies 64 buffers,
64 rules, or 64 cognitive cycles. The resident SoA population may be far
larger. Think of the slots as a bounded execution/attention frontier.

Match production conditions in parallel when profitable, but begin with
one logically selected *production commit* per ACT-R-compatible cycle.
If speculative or parallel firing is introduced, it must have explicit
dependency checks and deterministic commit/replay semantics.

### 3.5 Witnesses and effects

A candidate decision should include the addressed goal, the state version
observed, the eligible rule IDs, the selected action, and a traceable reason
for selection. Apply a transition only against its expected witness;
a stale witness must produce a documented retry/reconsideration result.

Human approval and irreversible tool execution are independent authorization
decisions. Cognitive selection alone must **never** authorize external effects.

Suggested Rust-shaped contract (illustrative, not compiled API):

~~~rust
struct CognitiveSnapshot {
    goal: ThoughtAddr,
    witness: WitnessVersion,
    active: PresenceMask,
    budget: ThoughtBudget,
}

enum CognitiveIntent {
    Retrieve(ThoughtAddr),
    Infer(RuleId),
    Revise(ThoughtAddr),
    RequestModel(ModelId),
    RequestTool(ToolId),
    AwaitEvidence,
    Finish,
}

trait CognitiveController {
    fn eligible(&self, state: &CognitiveSnapshot) -> CandidateMask;
    fn arbitrate(&self, state: &CognitiveSnapshot, candidates: CandidateMask)
        -> CognitiveIntent;
    fn record_outcome(&mut self, decision: DecisionWitness, outcome: Outcome);
}
~~~

Identifiers and masks above are **placeholder contract types**; use canonical
lance-graph types rather than inventing parallel ABI definitions. Transport
adapters may expose textual IDs, but string parsing and JSON serialization
must stop at the outbound/inbound membrane, not enter resident SoA lanes.

### 3.6 Consolidation should not require an LLM

Retain the eight-phase dream pipeline as a behavioral reference, but split
its operations into:

- Deterministic replay, linkage, confidence/utility updates, pruning
  proposals, and evidence-qualified schema discovery.
- Optional generative proposals for clustering, abstraction, and variants.
  A proposal is *not* evidence and acquires no privileged truth on arrival.
- Independent validation of candidates before promotion, using explicit
  provenance and witnessed outcomes.

A procedure should eventually be representable as a typed, executable
relational condition plus a permitted transition, not solely a text snippet.
Preserve existing outcomes, niche comparisons and historical provenance
during migration.

## 4. Proposed delivery slices and gates

| Phase | Change | Acceptance gate |
| --- | --- | --- |
| **P0** Reference inventory | Pin baseline activation, recall, visibility, procedure, and dream traces; map exact source APIs and fixtures | Reproducible baseline with pinned inputs; no implementation rewrite |
| **P1** Typed cognitive facade | Introduce memory/attention/selection/storage traits and explicit `MemoryId` to thought-address mapping | Legacy provider still passes baseline parity; scope and identity tests |
| **P2** SoA-backed candidate reads | Prototype presence-masked lookup with Quack/Mask-RISC over a small typed population | Exactly the eligible memory set, stable ordering policy, no unauthorized candidates |
| **P3** Production eligibility | Define a minimal goal/retrieval-buffer production subset and compile conditions via relalg/R2IL | Differential rule-match tests against a simple, explicit reference interpreter |
| **P4** Arbitration and witness | Add activation/utility arbitration, deterministic tie-break, version-checked commit and trace | Fixed-seed replay equals reference; stale-witness and effect-gate tests |
| **P5** Learning and consolidation | Port outcome feedback and deterministic consolidation; LLM ideas become optional hypotheses | No unsupported promotion; procedure lineage and rollback tests |
| **P6** Interop adapters | Optional Rig effects, graph-flow task jumps, z8run visual/trigger integration | All adapters share typed contract; no new hot-path serialization |
| **P7** Persistence cutover | Only after measured parity, introduce authoritative SoA-backed state and migration tooling | Round-trip migration, restart, rollback, and multi-agent isolation tests |

Each phase should be a separate small PR. No P7 migration should be merged
just because P2 query performance looks promising.

## 5. Evaluation and falsifiers

Use existing Python-parity activation/FSRS fixtures (typically `1e-4`
tolerance) as initial numeric references; expand the suite to property,
long-history, time-boundary and degenerate-input tests. Keep documented
`f32` numerical behavior until an independently verified quantized or SIMD
path earns replacement.

**Functional and isolation gates**

- Golden-set equivalence for recall candidates and ranking, with deterministic
  timestamps/seeds where required.
- ACL/visibility scope enforced *before* spreading, candidate generation,
  ranking and any export; invisible memories cannot influence visible scores.
- Thought identity preserved across recall, procedure execution, version
  updates, snapshots, and restart.
- Demonstrable trace fidelity: decision candidates, mask, selected rule,
  witnessed version, effects, and outcome can be replayed or explicitly marked
  nondeterministic.
- Differential check that ACT-R-style production *matching* is equivalent
  before measuring speed. Do not claim behavioral ACT-R parity from base-level
  activation parity alone.

**Performance and learning gates**

- Compare reference storage, native SoA access and Quack/masked candidate reads
  on the **same** workloads, separating warm/cold and indexed/unindexed cases.
- Measure latency p50/p95/p99, RSS, memory traffic, number of read lanes,
  selected candidates, and effect/API calls. Maintain existing embedded-device
  budgets (including the prior recall-path p99 increment target) where relevant.
- Ablate activation, FSRS, spreading, utility, and LLM proposals individually.
  Compare fixed workflows versus cognitively arbitrated workflows on held-out
  tasks. Keep datasets, seeds and decision thresholds preregistered.
- Use `jc` for fit/reliability/significance where available; respect its
  distinction between IID point-estimate tests and weak-dependence calibration.
  Report uncertainty and avoid treating correlated samples as independent
  replications.
- Require a **negative-result path**: if a masking, scheduling, or topology
  mechanism does not improve the predeclared metric, retain the baseline.

**Optional statistical API exploration:** JC can expose the statistics
through a restricted R-style formula frontend (for example, `score ~ cohort`)
or through an external R adapter. That frontend should compile *statistical
intent* into a typed relational plan, accumulate sufficient statistics over
selected SoA lanes, and call JC as the mathematical authority. This is not a
proposal to embed the R interpreter in the cognitive hot path. In ACT-R the
letter R means *Rational*, not the statistical language.

**Explicit rejection conditions:** scope leakage; a second authoritative
cognitive store; undocumented changes to ACT-R/FSRS math; hidden JSON/string
hot-path data models; duplicate arithmetic evaluators; unbounded speculative
side effects; success-only reporting; anthropomorphic claims substituted
for measured behavior.

## 6. Interfaces, licensing and operational boundaries

- Do not modify upstream reference behavior by surprise. This fork's current
  `CONTRIBUTING.md` specifies a parity-oriented Rust port; the proposed
  redesign must be opt-in and explicitly scoped as fork experimentation.
- Preserve the external MCP tool contract while the baseline compatibility
  mode remains supported. JSON-RPC, REST and third-party model payloads are
  **external protocol formats**, not internal representations.
- Keep external workflow frameworks as adapters, not core dependencies.
  `graph-flow` already offers resumable tasks and `NextAction::GoTo`;
  Rig offers provider/tool contracts; z8run offers a visual automation shell.
  None is required to run the symbolic control cycle.
- Preserve source attributions, license texts and notices from reused
  MIT-licensed components; separately check the actual licenses of any new
  dependencies before importing code.
- This public plan contains only public/open-source architectural concerns.
  Do not include employer-specific systems, private project details,
  credentials, proprietary schemas, internal documentation or reverse
  engineering notes.

## 7. Open design questions

1. Which public lane/address contract is stable enough to map existing
   `MemoryId` values without creating a second identity authority?
2. Should access-time history be held as explicit timestamp observations,
   a compressed sufficient-statistic sidecar, or both? Can truncation preserve
   retrieval-order parity within a specified horizon?
3. Can temporal spreading and production matching share the same mask planner
   without replacing their distinct scheduling semantics?
4. Where should transition authorization and optimistic version checks live
   when a selected cognitive action fans out to an external async workflow?
5. Which offline consolidation operations can be rewritten deterministically
   without losing source attribution or contradicting current outcomes?
6. Which P2/P4 benchmark result would justify touching the storage authority
   rather than keeping a pure adapter?

## 8. Suggested first implementation PR

Implement **P0 only**: a versioned, deterministic differential-replay harness
for (a) activation/FSRS recall ranking, (b) visibility-scoped association
spreading, and (c) procedural outcome selection. Record the reference inputs,
timestamps/seeds and result ordering. Make no storage migration and introduce
no new cognitive semantics.

Then P1 can introduce a typed boundary while repeatedly proving that the
reference behavior still holds. This is the smallest experiment that can
falsify the proposed integration before it becomes expensive.

---

**Document status:** Proposal for review, not approval or an implemented
feature. All architectural extensions and performance claims require their
own tests and follow-on PRs.
