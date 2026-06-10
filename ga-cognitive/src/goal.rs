use ga_semantics_core::prelude::*;
use ga_semantics_core::semantics::is_contradictory;

pub struct GoalTree {
    pub root: GoalNode,
    pub coherence: f64,
}

pub struct GoalNode {
    pub name: String,
    pub encoding: [f64; 8],
    pub phase: String,
    pub children: Vec<GoalNode>,
}

impl GoalTree {
    pub fn new(root_name: &str, root_encoding: &[f64; 8]) -> Self {
        let mv = Multivector::new(*root_encoding);
        let phase = format!("{:?}", mv.dominant_role().wuxing_phase());
        GoalTree {
            root: GoalNode {
                name: root_name.to_string(),
                encoding: *root_encoding,
                phase,
                children: vec![],
            },
            coherence: 1.0,
        }
    }

    pub fn add_subgoal(&mut self, parent_name: &str, name: &str, encoding: &[f64; 8]) -> bool {
        fn add_to_node(node: &mut GoalNode, parent: &str, name: &str, enc: &[f64; 8]) -> bool {
            if node.name == parent {
                let mv = Multivector::new(*enc);
                let phase = format!("{:?}", mv.dominant_role().wuxing_phase());
                node.children.push(GoalNode {
                    name: name.to_string(),
                    encoding: *enc,
                    phase,
                    children: vec![],
                });
                return true;
            }
            for child in &mut node.children {
                if add_to_node(child, parent, name, enc) { return true; }
            }
            false
        }
        add_to_node(&mut self.root, parent_name, name, encoding)
    }

    pub fn compute_coherence(&mut self) {
        let all_goals = self.collect_all();
        let mut contradictions = 0;
        let mut total_pairs = 0;

        for i in 0..all_goals.len() {
            for j in (i + 1)..all_goals.len() {
                total_pairs += 1;
                let a = Multivector::new(all_goals[i].1);
                let b = Multivector::new(all_goals[j].1);
                if is_contradictory(&a, &b, 0.5) { contradictions += 1; }
            }
        }

        self.coherence = if total_pairs > 0 {
            1.0 - (contradictions as f64 / total_pairs as f64)
        } else { 1.0 };
    }

    fn collect_all(&self) -> Vec<(String, [f64; 8])> {
        fn collect(node: &GoalNode, result: &mut Vec<(String, [f64; 8])>) {
            result.push((node.name.clone(), node.encoding));
            for child in &node.children {
                collect(child, result);
            }
        }
        let mut result = vec![];
        collect(&self.root, &mut result);
        result
    }

    pub fn phase_coverage(&self) -> f64 {
        let all = self.collect_all();
        let mut phases: std::collections::HashSet<String> = std::collections::HashSet::new();
        for (_, enc) in &all {
            let mv = Multivector::new(*enc);
            phases.insert(format!("{:?}", mv.dominant_role().wuxing_phase()));
        }
        phases.len() as f64 / 5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ga_semantics_core::blade::Blade;

    #[test]
    fn create_goal_tree_with_root() {
        let mv = Multivector::from_blade(Blade::E1, 1.0);
        let tree = GoalTree::new("MainGoal", mv.coefficients());
        assert_eq!(tree.root.name, "MainGoal");
        assert!(!tree.root.phase.is_empty());
        assert_eq!(tree.coherence, 1.0);
    }

    #[test]
    fn add_subgoal_and_compute_coherence() {
        let root_mv = Multivector::from_blade(Blade::E1, 1.0);
        let mut tree = GoalTree::new("Root", root_mv.coefficients());

        let child_a = Multivector::from_blade(Blade::E2, 1.0);
        assert!(tree.add_subgoal("Root", "ChildA", child_a.coefficients()));

        let child_b = Multivector::from_blade(Blade::Scalar, 1.0);
        assert!(tree.add_subgoal("Root", "ChildB", child_b.coefficients()));

        tree.compute_coherence();
        assert!(tree.coherence >= 0.0 && tree.coherence <= 1.0);
    }

    #[test]
    fn add_subgoal_to_nonexistent_parent_fails() {
        let mv = Multivector::from_blade(Blade::E1, 1.0);
        let mut tree = GoalTree::new("Root", mv.coefficients());
        let child = Multivector::from_blade(Blade::E2, 1.0);
        assert!(!tree.add_subgoal("NonExistent", "Child", child.coefficients()));
    }

    #[test]
    fn phase_coverage_single_goal() {
        let mv = Multivector::from_blade(Blade::E1, 1.0);
        let tree = GoalTree::new("Root", mv.coefficients());
        assert!(tree.phase_coverage() > 0.0);
        assert!(tree.phase_coverage() <= 1.0);
    }

    #[test]
    fn phase_coverage_all_five_phases() {
        let blades = [Blade::E1, Blade::E12, Blade::Scalar, Blade::E123, Blade::E2];
        let mut tree = GoalTree::new("0", &Multivector::from_blade(blades[0], 1.0).coefficients()[..8].try_into().unwrap());
        for i in 1..5 {
            let mv = Multivector::from_blade(blades[i], 1.0);
            tree.add_subgoal("0", &format!("goal{}", i), mv.coefficients());
        }
        assert!(tree.phase_coverage() > 0.5);
    }
}
