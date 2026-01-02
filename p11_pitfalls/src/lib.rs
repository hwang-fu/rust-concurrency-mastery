//! Pitfalls & Debugging
//!
//! A "museum of bugs" demonstrating common concurrency mistakes
//! and how to fix them.

pub mod arc_cycle_leak;
pub mod clone_confusion;
pub mod diagnostics;
pub mod poisoned_mutex;
pub mod unnecessary_arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_confusion_fixed() {
        clone_confusion::demo_fixed();
    }

    #[test]
    fn test_clone_confusion_loop() {
        clone_confusion::demo_loop_pattern();
    }

    #[test]
    fn test_poisoned_mutex_recovery() {
        poisoned_mutex::demo_simple_recovery();
    }

    #[test]
    fn test_arc_cycle_fixed() {
        arc_cycle_leak::fixed::demo_no_leak();
    }

    #[test]
    fn test_unnecessary_arc_scoped() {
        unnecessary_arc::demo_scoped_threads();
    }

    #[test]
    fn test_diagnostics() {
        diagnostics::demo_reference_counting();
        diagnostics::demo_drop_tracing();
        diagnostics::demo_weak_validity();
    }
}
