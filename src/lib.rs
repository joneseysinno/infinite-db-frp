//! # plexus-loom
//!
//! A unified facade for the Plexus-Loom backend.
//!
//! Add a single dependency to your project and opt into subsystems via feature flags:
//!
//! ```toml
//! [dependencies]
//! # Light defaults: core primitives + domain model + weaving + reactive signals
//! plexus-loom = "0.1"
//!
//! # Opt into the async execution runtime
//! plexus-loom = { version = "0.1", features = ["runtime"] }
//!
//! # Opt into durable on-disk storage
//! plexus-loom = { version = "0.1", features = ["storage"] }
//!
//! # Everything
//! plexus-loom = { version = "0.1", features = ["all"] }
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
//! | `storage`   | Durable on-disk storage (InfiniteDbStore)         | no     |
//! | `in-memory` | In-memory store implementations                  | no     |
//! | `all`       | All of the above                                  | no     |
//!
//! ## Modules
//!
//! - [`core`] — shared primitives
//! - [`store`] — storage traits and in-memory impls *(feature `domain`)*
//! - [`domain`] — graph domain model *(feature `domain`)*
//! - [`weave`] — block assembly and validation *(feature `weave`)*
//! - [`signal`] — reactive primitives *(feature `signals`)*
//! - [`engine`] — execution runtime *(feature `runtime`)*
//! - [`persistence`] — durable storage backend *(feature `storage`)*

// ---------------------------------------------------------------------------
// core
// ---------------------------------------------------------------------------

/// Shared primitives: typed IDs, SPC/Morton encoding, type signatures, and
/// the runtime `Value` enum.
///
/// Available with feature `core` (default).
#[cfg(feature = "core")]
pub mod core {
    pub use plexus_base::{
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

/// Storage traits and in-memory implementations for the Loom graph layer.
///
/// Available with feature `domain` (default).
#[cfg(feature = "domain")]
pub mod store {
    pub use loom_base::{
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
        pub use loom_base::{
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
    pub use loom_domain::{
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
    pub use loom_weave::{
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
    pub use plexus_signal::{
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
    pub use plexus_engine::{
        EngineError, Executor, Graph, Scheduler,
    };

    /// Advanced / low-level engine utilities.
    pub mod advanced {
        pub use plexus_engine::{
            BoxFuture, TransformRegistry, eval_transform, toposort,
        };
    }
}

// ---------------------------------------------------------------------------
// persistence
// ---------------------------------------------------------------------------

/// Durable on-disk storage backend.
///
/// Available with feature `storage`.
#[cfg(feature = "storage")]
pub mod persistence {
    pub use plexus_persistence::{InfiniteDbStore, PersistenceError};
}
