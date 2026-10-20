use qrunap2::{parse, run};
const BASIC: &str = include_str!("../examples/original/first-queue.qnp");
#[test]
fn deterministic_calendar_and_censoring() {
    let s = include_str!("../examples/original/deterministic.qnp");
    let r = run(s, 42).unwrap();
    assert_eq!((r.generated, r.exited, r.remaining), (10, 9, 1));
    let desk = &r.stations[1];
    assert_eq!(desk.mean_wait, Some(0.0));
    assert_eq!(desk.mean_response, Some(1.0));
    assert!((desk.utilization - 0.45).abs() < 1e-12);
    assert_eq!(
        r.initial as u64 + r.generated,
        r.exited + r.remaining as u64
    );
}
#[test]
fn examples_and_reproducibility() {
    for s in [
        BASIC,
        include_str!("../examples/original/two-servers.qnp"),
        include_str!("../examples/original/overload.qnp"),
        include_str!("../examples/historical/inria-1995-adapted.qnp"),
    ] {
        let r = run(s, 42).unwrap();
        assert_eq!(r, run(s, 42).unwrap());
        assert_eq!(
            r.initial as u64 + r.generated,
            r.exited + r.remaining as u64
        );
        assert_eq!(
            r.remaining,
            r.stations.iter().map(|s| s.remaining).sum::<usize>()
        );
        assert!(r
            .stations
            .iter()
            .all(|s| (0.0..=1.000000000001).contains(&s.utilization)));
    }
}
#[test]
fn closed_population_stays_constant() {
    let r = run(
        include_str!("../examples/historical/inria-1995-adapted.qnp"),
        42,
    )
    .unwrap();
    assert_eq!(r.remaining, 40);
    assert_eq!(r.generated, 0);
    assert_eq!(r.exited, 0);
    assert!(r.trace.iter().all(|p| p.population == 40));
    assert!((r.stations.iter().map(|s| s.mean_number).sum::<f64>() - 40.0).abs() < 1e-9);
}
#[test]
fn mm1_independent_replications_against_theory() {
    let s = BASIC.replace("10000", "200000");
    let mut l = 0.0;
    let mut w = 0.0;
    let mut u = 0.0;
    for seed in 1..=8 {
        let r = run(&s, seed).unwrap();
        l += r.stations[1].mean_number;
        w += r.stations[1].mean_response.unwrap();
        u += r.stations[1].utilization;
    }
    // rho=.5, L=1, W=2. Broad fixed regression bounds, not a confidence interval.
    assert!((l / 8.0 - 1.0).abs() < 0.06);
    assert!((w / 8.0 - 2.0).abs() < 0.1);
    assert!((u / 8.0 - 0.5).abs() < 0.02);
}
#[test]
fn multiple_servers_reduce_waiting() {
    let s = BASIC.replace("EXP(2)", "EXP(0.75)");
    let one = run(&s, 42).unwrap();
    let two = run(&s.replace("SINGLE", "MULTIPLE(2)"), 42).unwrap();
    assert!(one.stations[1].mean_queue > 100.0);
    assert!(two.stations[1].mean_queue < 5.0);
}
#[test]
fn rejects_invalid_and_unsupported_input() {
    for s in [
        BASIC.replace("EXP(2)", "EXP(0)"),
        BASIC.replace("TRANSIT=DESK", "TRANSIT=MISSING"),
        BASIC.replace("TMAX=10000;", "TMAX=10000; OPTION=SIMPAR;"),
        BASIC.replace("SIMUL;", "SOLVE;"),
        BASIC.replace("QUEUE INPUT, DESK;", "QUEUE INPUT, INPUT;"),
        BASIC.replace("SERVICE=EXP(1);", "SERVICE=EXP(1); SERVICE=EXP(2);"),
        BASIC.replace("TRANSIT=OUT;", "TRANSIT=INPUT;"),
        BASIC
            .replace("MULTIPLE(2)", "MULTIPLE(0)")
            .replace("SINGLE", "MULTIPLE(0)"),
        format!("{BASIC} trailing"),
        BASIC.replace("/END/", ""),
    ] {
        assert!(parse(&s).is_err(), "{s}");
    }
    assert!(run(BASIC, 0).is_err());
    assert!(parse("/DECLARE/\nQUEUE 😀;")
        .unwrap_err()
        .contains("Line 2"));
}
#[test]
fn runtime_limits_are_errors() {
    let s = BASIC.replace("EXP(2)", "CST(0.000001)");
    assert!(run(&s, 42).unwrap_err().contains("limit"));
}
