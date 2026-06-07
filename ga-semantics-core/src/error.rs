use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum AlgebraicError {
    #[error("multivector has zero norm, cannot compute inverse")]
    ZeroNorm,
}
