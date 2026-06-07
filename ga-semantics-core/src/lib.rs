pub mod bagua;
pub mod blade;
pub mod encoding;
pub mod error;
pub mod multivector;
pub mod relation_type;
pub mod rotor;
pub mod semantics;

#[cfg(feature = "store")]
pub mod store;

#[cfg(feature = "serde")]
#[path = "serde.rs"]
pub mod serde_impl;

#[cfg(feature = "python")]
#[path = "python.rs"]
pub mod python_impl;

pub use blade::Blade;
pub use error::AlgebraicError;
pub use multivector::Multivector;
pub use relation_type::RelationType;
pub use rotor::Rotor;
pub use encoding::{hash_encode, llm_encode, multivector_describe, multivector_to_roles,
    text_to_multivector, word_to_multivector};
pub use semantics::{Context, analogy, analogy_confidence, compose_chain, compose_relations,
    dominant_similarity, inverse_relation, is_contradictory, relation_strength,
    semantic_difference, semantic_relation, semantic_similarity};

/// Advanced types (Bagua trigrams, hexagrams, WuXing) are available here.
/// The primary public interface uses `RelationType` semantic role labels.
pub mod advanced {
    pub use crate::bagua::{Hexagram, Trigram, WuXing, trigram_transform_details, wuxing_controlling_chain, wuxing_generating_chain};
}

pub mod prelude {
    pub use crate::blade::Blade;
    pub use crate::encoding::{hash_encode, llm_encode, multivector_describe, multivector_to_roles,
        text_to_multivector, word_to_multivector};
    pub use crate::multivector::Multivector;
    pub use crate::relation_type::RelationType;
    pub use crate::rotor::Rotor;
    pub use crate::semantics::{Context, analogy, analogy_confidence, compose_chain, compose_relations,
        dominant_similarity, inverse_relation, is_contradictory, relation_strength,
        semantic_difference, semantic_relation, semantic_similarity};
}
