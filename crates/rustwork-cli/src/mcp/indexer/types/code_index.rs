use super::{ProjectInfo, SourceFile};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Complete code index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIndex {
    pub files: HashMap<String, SourceFile>,
    pub call_graph: HashMap<String, HashSet<String>>, // function -> called functions
    pub reverse_call_graph: HashMap<String, HashSet<String>>, // function -> callers
    pub projects: Vec<ProjectInfo>,                   // All discovered projects/services
    pub last_scan: u64,
}

impl CodeIndex {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            call_graph: HashMap::new(),
            reverse_call_graph: HashMap::new(),
            projects: Vec::new(),
            last_scan: 0,
        }
    }

    /// Build call graphs from indexed files
    pub fn build_call_graphs(&mut self) {
        self.call_graph.clear();
        self.reverse_call_graph.clear();

        for file in self.files.values() {
            for func in &file.functions {
                let full_name = format!("{}::{}", file.module_path, func.name);

                // Direct call graph
                let mut called = HashSet::new();
                for call in &func.calls {
                    called.insert(call.clone());

                    // Reverse call graph
                    self.reverse_call_graph
                        .entry(call.clone())
                        .or_default()
                        .insert(full_name.clone());
                }
                self.call_graph.insert(full_name, called);
            }
        }
    }

    /// Get all callers of a function (who calls this function)
    pub fn get_callers(&self, function: &str) -> Vec<String> {
        self.reverse_call_graph
            .get(function)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all functions called by a function
    pub fn get_calls(&self, function: &str, depth: usize) -> HashMap<String, usize> {
        let mut result = HashMap::new();
        let mut visited = HashSet::new();
        self.get_calls_recursive(function, depth, 0, &mut result, &mut visited);
        result
    }

    fn get_calls_recursive(
        &self,
        function: &str,
        max_depth: usize,
        current_depth: usize,
        result: &mut HashMap<String, usize>,
        visited: &mut HashSet<String>,
    ) {
        if current_depth >= max_depth || visited.contains(function) {
            return;
        }

        visited.insert(function.to_string());

        if let Some(calls) = self.call_graph.get(function) {
            for called in calls {
                result.insert(called.clone(), current_depth + 1);
                self.get_calls_recursive(called, max_depth, current_depth + 1, result, visited);
            }
        }
    }
}
