//! This module contains a scheduler.

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use Shared;

/// A routine which could be run.
pub(crate) trait Runnable {
    /// Runs a routine with a context instance.
    fn run(&mut self, _: &mut ());
}

/// This is a global scheduler suitable to schedule and run any tasks.
pub struct Scheduler<CTX> {
    context: Shared<CTX>,
    sequence: Shared<VecDeque<Box<Runnable>>>,
}

impl<CTX> Clone for Scheduler<CTX> {
    fn clone(&self) -> Self {
        Scheduler {
            context: self.context.clone(),
            sequence: self.sequence.clone(),
        }
    }
}

impl<CTX> Scheduler<CTX> {
    /// Creates a new scheduler with a context.
    pub fn new(context: CTX) -> Self {
        let sequence = VecDeque::new();
        Scheduler {
            context: Rc::new(RefCell::new(context)),
            sequence: Rc::new(RefCell::new(sequence)),
        }
    }

    pub(crate) fn put_and_try_run(&self, runnable: Box<Runnable>) {
        self.sequence.borrow_mut().push_back(runnable);
        if let Ok(_) = self.context.try_borrow_mut() {
            loop {
                let do_next = self.sequence.borrow_mut().pop_front();
                if let Some(mut runnable) = do_next {
                    runnable.run(&mut ());
                } else {
                    break;
                }
            }
        }
    }
}
