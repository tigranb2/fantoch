// This module contains the definition of `Pending`.
mod pending;

// This module contains the implementation of a basic executor that executes
// operations as soon as it receives them.
mod basic;

// This module contains the implementation of a dependency graph executor.
mod graph;

// This module contains the implementation of a votes table executor.
mod table;

// Re-exports.
pub use basic::{BasicExecutionInfo, BasicExecutor};
pub use graph::{GraphExecutionInfo, GraphExecutor};
pub use pending::Pending;
pub use table::{TableExecutionInfo, TableExecutor};

use crate::command::{Command, CommandResult};
use crate::config::Config;
use crate::id::{ClientId, Rifl};
use crate::kvs::{KVOpResult, Key};
use std::fmt::Debug;

pub trait Executor {
    type ExecutionInfo: Clone + Debug + Send + MessageKey;

    fn new(config: Config) -> Self;

    fn wait_for(&mut self, cmd: &Command);

    // Parallel executors may receive several waits for the same `Rifl`.
    fn wait_for_rifl(&mut self, rifl: Rifl);

    // TODO we can return an iterator here
    #[must_use]
    fn handle(&mut self, infos: Self::ExecutionInfo) -> Vec<ExecutorResult>;

    fn parallel() -> bool;

    fn show_metrics(&self) {
        // by default, nothing to show
    }
}

pub trait MessageKey {
    /// If `None` is returned, then the message is sent the *single* executor
    /// process. If there's more than one executor, and this function
    /// returns `None`, the runtime will panic.
    fn key(&self) -> Option<&Key> {
        None
    }
}

#[derive(Debug)]
pub enum ExecutorResult {
    /// this contains a complete command result
    Ready(CommandResult),
    /// this contains a partial command result
    Partial(Rifl, Key, KVOpResult),
}

impl ExecutorResult {
    /// Check which client should receive this result.
    pub fn client(&self) -> ClientId {
        match self {
            ExecutorResult::Ready(cmd_result) => cmd_result.rifl().source(),
            ExecutorResult::Partial(rifl, _, _) => rifl.source(),
        }
    }

    /// Extracts a ready results from self. Panics if not ready.
    pub fn unwrap_ready(self) -> CommandResult {
        match self {
            ExecutorResult::Ready(cmd_result) => cmd_result,
            ExecutorResult::Partial(_, _, _) => panic!(
                "called `ExecutorResult::unwrap_ready()` on a `ExecutorResult::Partial` value"
            ),
        }
    }
    /// Extracts a partial result from self. Panics if not partial.
    pub fn unwrap_partial(self) -> (Rifl, Key, KVOpResult) {
        match self {
            ExecutorResult::Partial(rifl, key, result) => (rifl, key, result),
            ExecutorResult::Ready(_) => panic!(
                "called `ExecutorResult::unwrap_partial()` on a `ExecutorResult::Ready` value"
            ),
        }
    }
}