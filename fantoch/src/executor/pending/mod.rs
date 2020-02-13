// This module contains the implementation of `SimplePending`.
mod simple;

// This module contains the implementation of `AggregatePending`.
mod aggregate;

use crate::command::Command;
use crate::executor::ExecutorResult;
use crate::id::Rifl;
use crate::kvs::{KVOpResult, Key};
use aggregate::AggregatePending;
use simple::SimplePending;

pub struct Pending {
    // TODO can we not have both simple and aggregate pending and move that
    // decision to compile-time
    aggregate: bool,
    simple_pending: SimplePending,
    aggregate_pending: AggregatePending,
}

/// Creates a new `Pending` instance.
impl Pending {
    pub fn new(aggregate: bool) -> Self {
        Self {
            aggregate,
            simple_pending: SimplePending::new(),
            aggregate_pending: AggregatePending::new(),
        }
    }

    /// Starts tracking a command submitted by some client.
    pub fn wait_for(&mut self, cmd: &Command) -> bool {
        if self.aggregate {
            self.aggregate_pending.wait_for(cmd)
        } else {
            self.simple_pending.wait_for(cmd)
        }
    }

    /// Increases the number of expected notifications on some `Rifl` by one.
    pub fn wait_for_rifl(&mut self, rifl: Rifl) {
        if self.aggregate {
            self.aggregate_pending.wait_for_rifl(rifl)
        } else {
            self.simple_pending.wait_for_rifl(rifl)
        }
    }

    /// Adds a new partial command result.
    pub fn add_partial<P>(
        &mut self,
        rifl: Rifl,
        partial: P,
    ) -> Option<ExecutorResult>
    where
        P: FnOnce() -> (Key, KVOpResult),
    {
        if self.aggregate {
            self.aggregate_pending.add_partial(rifl, partial)
        } else {
            self.simple_pending.add_partial(rifl, partial)
        }
    }
}