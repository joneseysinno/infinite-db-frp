# infinite-db-frp

A unified facade for the infinite-db FRP backend: **graph modeling, weaving,
reactive signals, and async execution** — one dependency, feature-gated
subsystems.

This crate is **read + execute oriented**. It does **not** bundle on-disk
persistence; depend on [`frp-persistence`](https://crates.io/crates/frp-persistence)
(or Hyperblock) to write and read [`infinite-db`](https://github.com/joneseysinno/infinitedb).

## Quick start

```toml
[dependencies]
# Defaults: core + domain + weave + signals
infinite-db-frp = "0.2"

# Async graph execution
infinite-db-frp = { version = "0.2", features = ["runtime"] }

# Runtime + in-memory stores (everything this facade offers)
infinite-db-frp = { version = "0.2", features = ["all"] }
```

## Feature flags

| Feature     | Enables                                            | Default |
|-------------|----------------------------------------------------|---------|
| `core`      | Typed IDs, SPC encoding, `Value`, type primitives  | yes     |
| `domain`    | Storage traits, domain model (atoms/blocks/edges)  | yes     |
| `weave`     | Block assembly (`Composer`) and validation         | yes     |
| `signals`   | Pull-based reactive primitives (`Signal`, `Slot`)  | yes     |
| `runtime`   | Async graph execution (`Graph`, `Executor`)        | no      |
| `in-memory` | In-memory store implementations                   | no      |
| `all`       | `runtime` + `in-memory`                            | no      |

## Modules

| Module             | Feature     | Contents                                      |
|--------------------|-------------|-----------------------------------------------|
| `core`             | `core`      | `AtomId`, `Value`, `SpcKey`, `TypeSig`, …     |
| `store`            | `domain`    | `AtomStore`, `BlockStore`, `EdgeStore`, …     |
| `store::memory`    | `in-memory` | `InMemoryAtomStore`, `InMemoryBlockStore`, …  |
| `domain`           | `domain`    | `Atom`, `Block`, `HyperEdge`, `Port`, …       |
| `weave`            | `weave`     | `Composer`, `Validator`, `Archetype`, …       |
| `signal`           | `signals`   | `Signal`, `Computed`, `Slot`, `DirtySet`, …   |
| `engine`           | `runtime`   | `Graph`, `Executor`, `Scheduler`              |
| `engine::advanced` | `runtime`   | `TransformRegistry`, `toposort`, …          |

## Persisting / authoring

- **Executable database:** [infinite-db](https://github.com/joneseysinno/infinitedb)
- **Rust store implementation:** add `frp-persistence` for `InfiniteDbStore`
  and trait-based put/get against on-disk storage.
- **Authoring stacks:** Hyperblock (or similar) should write the canonical
  snapshot; this facade focuses on types and runtime execution.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
