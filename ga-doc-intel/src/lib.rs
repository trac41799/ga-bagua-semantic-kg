pub mod alignment;
pub mod coherence;
pub mod contract;
pub mod document;
pub mod fallacy;
pub mod synthesis;

pub mod prelude {
    pub use crate::alignment::{align_documents, AlignmentReport, ClaimAlignment};
    pub use crate::coherence::{inter_coherence, intra_coherence, CoherenceReport};
    pub use crate::contract::{audit_contract, ContractAuditReport};
    pub use crate::document::{Document, DocumentStore};
    pub use crate::fallacy::{analyze_argument, ArgumentGraph, FallacyResult};
    pub use crate::synthesis::{find_gaps, SynthesisReport};
}
