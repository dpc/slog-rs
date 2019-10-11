//! Nursery for slog-rs
//!
//! This crate is forever unstable, and contains things that
//! at a given moment are useful but not final.
#![warn(missing_docs, rust_2018_idioms)]

use slog::*;
use std::result;

/// `Drain` that switches destination of error
///
/// Logs everything to drain `D1`, but in case of it reporting an error,
/// switching to `D2`. If `D2` returns an error too, `Failover` will return
/// an error.
pub struct Failover<D1: Drain, D2: Drain> {
    drain1: D1,
    drain2: D2,
}

impl<D1: Drain, D2: Drain, O> Failover<D1, D2>
where
    D1: Drain<Ok = O>,
    D2: Drain<Ok = O>,
{
    /// Create `Failover`
    pub fn new(drain1: D1, drain2: D2) -> Self {
        Failover {
            drain1: drain1,
            drain2: drain2,
        }
    }
}

impl<D1, D2, E2, O> Drain for Failover<D1, D2>
where
    D1: Drain<Ok = O>,
    D2: Drain<Ok = O>,
{
    type Ok = O;
    type Err = D2::Err;
    fn log(
        &self,
        info: &Record,
        logger_values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        match self.drain1.log(info, logger_values) {
            Ok(ok) => Ok(ok),
            Err(_) => self.drain2.log(info, logger_values),
        }
    }
}

/// Failover logging to secondary drain on primary's failure
///
/// Create `Failover` drain
pub fn failover<D1: Drain, D2: Drain, O>(d1: D1, d2: D2) -> Failover<D1, D2>
where
    D1: Drain<Ok = O>,
    D2: Drain<Ok = O>,
{
    Failover::new(d1, d2)
}
