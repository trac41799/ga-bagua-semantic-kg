pub mod agent;
pub mod belief;
pub mod compatibility;
pub mod goal;
pub mod learning;

pub mod prelude {
    pub use crate::agent::AgentStore;
    pub use crate::belief::{BeliefSnapshot, BeliefTimeline};
    pub use crate::compatibility::{form_best_team, form_team, personality_compatibility, CompatibilityReport};
    pub use crate::goal::{GoalNode, GoalTree};
    pub use crate::learning::{detect_prerequisites, generate_learning_path, LearningPath, LearningStep};
}
