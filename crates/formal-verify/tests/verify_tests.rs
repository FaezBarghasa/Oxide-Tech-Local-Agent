#[cfg(test)]
mod tests {
    use formal_verify::{solve_task_schedule, generate_kani_proof_harness, TaskConstraint, KaniHarnessTarget};

    #[test]
    fn test_schedule_feasibility() {
        let tasks = vec![
            TaskConstraint {
                task_id: "adc_read".to_string(),
                duration_cycles: 10,
                deadline_cycles: 20,
                priority: 1,
            },
            TaskConstraint {
                task_id: "pid_calc".to_string(),
                duration_cycles: 20,
                deadline_cycles: 50,
                priority: 2,
            },
        ];

        let result = solve_task_schedule(&tasks, 1);
        assert!(result.is_ok());
        let sched = result.unwrap();
        assert!(sched.verified_feasible);
        assert_eq!(sched.total_span, 30);
    }

    #[test]
    fn test_impossible_deadline() {
        let tasks = vec![
            TaskConstraint {
                task_id: "heavy_task".to_string(),
                duration_cycles: 100,
                deadline_cycles: 50,
                priority: 1,
            },
        ];

        let result = solve_task_schedule(&tasks, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_kani_harness_generation() {
        let target = KaniHarnessTarget {
            function_name: "calculate_pwm_duty".to_string(),
            input_bounds: vec![("raw_val".to_string(), "u16".to_string())],
            check_no_panic: true,
            check_no_overflow: true,
        };

        let code = generate_kani_proof_harness(&target);
        assert!(code.contains("#[kani::proof]"));
        assert!(code.contains("calculate_pwm_duty"));
    }
}
