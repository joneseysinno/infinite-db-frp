//! # infinite-db-frp
//!
//! A unified facade for **modeling and executing** functional-reactive graphs
//! that align with an [infinite-db](https://github.com/joneseysinno/infinitedb)
//!–backed executable database.
//!
//! This crate is **read + execute oriented**: it re-exports domain types,
//! weaving, pull-based reactive primitives, and (optionally) the async graph
//! runtime. **Loading** snapshot data from `infinite-db` is expected via your
//! app, [`frp-persistence`], or Hyperblock — not through this facade.
//!
//! ## Vocabulary
//!
//! - **infinite-db** uses *Hyperedge* and *Signal* for spatial linkage and
//!   scoped fields in the database engine.
//! - This facade’s **[`domain::HyperEdge`]** is the **FRP dataflow** edge
//!   (multi-port → transform → multi-port). The **[`signal`]** module is
//!   **pull-based** reactive plumbing (`Signal`, `Computed`, …), not the DB
//!   `Signal` type.
//!
//! Keep those names distinct when wiring storage to the runtime.
//!
//! ## Dependencies
//!
//! ```toml
//! [dependencies]
//! # Defaults: core + domain + weave + signals
//! infinite-db-frp = "0.2"
//!
//! # Async graph execution
//! infinite-db-frp = { version = "0.2", features = ["runtime"] }
//!
//! # Runtime + in-memory stores (everything this facade offers)
//! infinite-db-frp = { version = "0.2", features = ["all"] }
//! ```
//!
//! ## Feature flags
//!
//! | Feature     | Enables                                          | Default |
//! |-------------|--------------------------------------------------|---------|
//! | `core`      | Typed IDs, SPC encoding, `Value`, type primitives | yes    |
//! | `domain`    | Storage traits, in-memory stores, domain model   | yes     |
//! | `weave`     | Block assembly and validation                    | yes     |
//! | `signals`   | Pull-based reactive primitives                   | yes     |
//! | `runtime`   | Async graph execution (Graph, Executor, Scheduler) | no   |
//! | `in-memory` | In-memory store implementations                  | no     |
//! | `all`       | `runtime` + `in-memory`                          | no     |
//!
//! ## Modules
//!
//! - [`core`] — shared primitives
//! - [`store`] — storage traits and in-memory impls *(feature `domain`)*
//! - [`domain`] — graph domain model *(feature `domain`)*
//! - [`weave`] — block assembly and validation *(feature `weave`)*
//! - [`signal`] — reactive primitives *(feature `signals`)*
//! - [`engine`] — execution runtime *(feature `runtime`)*
//!
//! [`frp-persistence`]: https://crates.io/crates/frp-persistence

// ---------------------------------------------------------------------------
// core
// ---------------------------------------------------------------------------

/// Shared primitives: typed IDs, SPC/Morton encoding, type signatures, and
/// the runtime `Value` enum.
///
/// Available with feature `core` (default).
#[cfg(feature = "core")]
pub mod core {
    pub use frp_plexus::{
        // Identifiers
        AtomId, BlockId, EdgeId, GraphId, IdGen, PortId,
        // Spatial encoding
        SpcKey, SpcRegion, morton_decode, morton_encode,
        // Type system
        LayerTag, TypeSig,
        // Values
        Value,
    };
}

// ---------------------------------------------------------------------------
// store
// ---------------------------------------------------------------------------

/// Storage traits and in-memory implementations for the frp graph layer.
///
/// Available with feature `domain` (default).
#[cfg(feature = "domain")]
pub mod store {
    pub use frp_loom::{
        // Error
        StoreError,
        // Pagination
        Query, QueryResult,
        // Traits
        AtomStore, BlockStore, EdgeStore,
    };

    /// In-memory store implementations.
    ///
    /// Available with feature `in-memory`.
    #[cfg(feature = "in-memory")]
    pub mod memory {
        pub use frp_loom::{
            HasAtomId, HasBlockId, HasEdgeId,
            InMemoryAtomStore, InMemoryBlockStore, InMemoryEdgeStore,
        };
    }
}

// ---------------------------------------------------------------------------
// domain
// ---------------------------------------------------------------------------

/// Domain model: atoms, blocks, ports, edges, and metadata.
///
/// Available with feature `domain` (default).
#[cfg(feature = "domain")]
pub mod domain {
    pub use frp_domain::{
        // Atoms
        Atom, AtomKind, AtomMeta,
        // Blocks
        Block, BlockBuilder, BlockSchema,
        // Edges
        EdgeSchedule, EdgeTransform, HyperEdge,
        // Error
        DomainError,
        // Metadata
        Meta,
        // Ports
        Port, PortDirection,
    };
}

// ---------------------------------------------------------------------------
// weave
// ---------------------------------------------------------------------------

/// Stateless block assembly and validation.
///
/// Available with feature `weave` (default).
#[cfg(feature = "weave")]
pub mod weave {
    pub use frp_weave::{
        // Archetypes
        Archetype, ArchetypeRegistry,
        // Templates
        BlockTemplate, PortDef,
        // Assembly / validation
        Composer, Validator,
        // Error
        WeaveError,
    };
}

// ---------------------------------------------------------------------------
// signal
// ---------------------------------------------------------------------------

/// Pull-based reactive primitives.
///
/// Available with feature `signals` (default).
#[cfg(feature = "signals")]
pub mod signal {
    pub use frp_signal::{
        Computed, DirtySet, DirtySink, Signal, SignalSubscribable, Slot,
    };
}

// ---------------------------------------------------------------------------
// engine
// ---------------------------------------------------------------------------

/// Async graph execution runtime.
///
/// Available with feature `runtime`.
///
/// # Primary surface
///
/// - [`engine::Graph`] — live container for blocks, edges, atoms, and port values
/// - [`engine::Executor`] — evaluates a single edge against the current port-value map
/// - [`engine::Scheduler`] — decides which edges fire and when
///
/// Advanced internals (transforms, toposort) are available via [`engine::advanced`].
#[cfg(feature = "runtime")]
pub mod engine {
    pub use frp_engine::{
        EngineError, Executor, Graph, Scheduler,
    };

    /// Advanced / low-level engine utilities.
    pub mod advanced {
        pub use frp_engine::{
            BoxFuture, TransformRegistry, eval_transform, toposort,
        };
    }
}
