use ethos_core::{CollapsedStack, TraceStep};
use std::collections::HashMap;
use log::debug;

pub struct Aggregator;

impl Aggregator {
    /// Build collapsed stacks from a sequence of raw trace steps (structLogs style).
    ///
    /// # Algorithm
    /// 1. Walk through execution steps
    /// 2. Track call stack depth
    /// 3. Build stack strings for each gas-consuming operation
    /// 4. Aggregate by unique stack (sum gas weights)
    pub fn build_collapsed_stacks(steps: &[TraceStep]) -> Vec<CollapsedStack> {
        debug!("Building collapsed stacks from {} execution steps", steps.len());

        // Map to aggregate stacks: stack_string -> (total_gas, last_pc)
        let mut stack_map: HashMap<String, (u64, u64)> = HashMap::new();

        // Current call stack
        let mut call_stack: Vec<String> = Vec::new();

        for step in steps {
            let operation = step.op.clone();
            let current_depth = step.depth as usize;

            // If depth decreased, we returned from function calls
            if current_depth < call_stack.len() {
                call_stack.truncate(current_depth);
            }

            // If depth increased, we entered a new call
            while call_stack.len() < current_depth {
                call_stack.push("CALL".to_string());
            }

            // Build the full stack string with current operation
            let stack_str = if call_stack.is_empty() {
                operation.clone()
            } else {
                format!("{};{}", call_stack.join(";"), operation)
            };

            // Accumulate gas cost
            let entry = stack_map.entry(stack_str).or_insert((0, 0));
            entry.0 += step.gas_cost;
            entry.1 = step.pc;

            // Important: we push the actual smart contract address or function 
            // if we can extract it in the future, but for raw structural mapping, 
            // the operation often serves as the leaf node.
        }

        let mut stacks: Vec<CollapsedStack> = stack_map
            .into_iter()
            .map(|(stack, (weight, pc))| CollapsedStack {
                stack,
                weight,
                last_pc: Some(pc),
            })
            .collect();

        stacks.sort_by(|a, b| b.weight.cmp(&a.weight));
        debug!("Built {} unique collapsed stacks", stacks.len());

        stacks
    }
}
