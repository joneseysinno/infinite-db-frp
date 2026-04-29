# plexus-loom

A unified facade for the Plexus-Loom backend: graph modeling, weaving,
reactive signals, async execution runtime, and durable storage — one
dependency, feature-gated subsystems.

## Quick start

```toml
[dependencies]
# Light defaults: core primitives + domain model + block weaving + reactive signals
plexus-loom = "0.1"

# Opt into the async execution runtime
plexus-loom = { version = "0.1", features = ["runtime"] }

# Opt into durable on-disk storage
plexus-loom = { version = "0.1", features = ["storage"] }

# Everything
plexus-loom = { version = "0.1", features = ["all"] }
```

## Feature flags

| Feature     | Enables                                            | Default |
|-------------|----------------------------------------------------|---------|
| `core`      | Typed IDs, SPC encoding, `Value`, type primitives  | yes     |
| `domain`    | Storage traits, domain model (atoms/blocks/edges)  | yes     |
| `weave`     | Block assembly (`Composer`) and validation         | yes     |
| `signals`   | Pull-based reactive primitives (`Signal`, `Slot`)  | yes     |
| `runtime`   | Async graph execution (`Graph`, `Executor`)        | no      |
| `storage`   | Durable on-disk storage (`InfiniteDbStore`)        | no      |
| `in-memory` | In-memory store implementations                   | no      |
| `all`       | All of the above                                   | no      |

## Modules

| Module          | Feature     | Contents                                      |
|-----------------|-------------|-----------------------------------------------|
| `core`          | `core`      | `AtomId`, `Value`, `SpcKey`, `TypeSig`, …     |
| `store`         | `domain`    | `AtomStore`, `BlockStore`, `EdgeStore`, …     |
| `store::memory` | `in-memory` | `InMemoryAtomStore`, `InMemoryBlockStore`, …  |
| `domain`        | `domain`    | `Atom`, `Block`, `HyperEdge`, `Port`, …       |
| `weave`         | `weave`     | `Composer`, `Validator`, `Archetype`, …       |
| `signal`        | `signals`   | `Signal`, `Computed`, `Slot`, `DirtySet`, …   |
| `engine`        | `runtime`   | `Graph`, `Executor`, `Scheduler`              |
| `engine::advanced` | `runtime` | `TransformRegistry`, `toposort`, …          |
| `persistence`   | `storage`   | `InfiniteDbStore`, `PersistenceError`         |

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
