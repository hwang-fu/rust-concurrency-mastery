//! Deadlock Laboratory
//!
//! A collection of deadlock scenarios and their fixes.
//! Run these examples to observe deadlock behavior.

pub mod scenario_a_lock_order;
pub mod scenario_b_recursive;
pub mod scenario_c_starvation;

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: We only test the FIXED versions.
    // The deadlock demos would hang forever!

    #[test]
    fn test_scenario_a_fixed() {
        scenario_a_lock_order::demo_fixed();
    }

    #[test]
    fn test_scenario_b_fixed_drop_early() {
        scenario_b_recursive::demo_fixed_drop_early();
    }

    #[test]
    fn test_scenario_b_fixed_pass_guard() {
        scenario_b_recursive::demo_fixed_pass_guard();
    }

    #[test]
    fn test_scenario_b_nested_with_drop() {
        // This tests the nested scenario WITH the drop() call
        scenario_b_recursive::demo_deadlock_nested();
    }

    #[test]
    fn test_scenario_c_starvation_demo() {
        // This won't actually starve on Linux (writer-preferring)
        scenario_c_starvation::demo_potential_starvation();
    }

    #[test]
    fn test_scenario_c_discussion() {
        scenario_c_starvation::discussion();
    }
}
