#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

extern crate progress;

extern crate quickcheck;

use progress::Progress;

use quickcheck::TestResult;

#[test]
fn generated_with_default_values() {
    let p = Progress::default();

    assert_eq!(0, p.current());
    assert_eq!(100, p.total());
}

#[quickcheck]
fn creates_progress_bar(c: usize, t: usize) -> bool {
    let p = Progress::new(c, t);

    (p.current() == c) && (p.total() == t)
}

#[quickcheck]
fn increments_one(c: usize, t: usize) -> TestResult {
    if c >= t { return TestResult::discard() }
    let mut p = Progress::new(c, t);
    p.increment();
    TestResult::from_bool(p.current() == c + 1)
}

#[quickcheck]
fn do_not_increment_above_total(t: usize) -> bool {
    let mut p = Progress::new(t, t);
    p.increment();
    p.current() == p.total()
}

#[quickcheck]
fn decrements_one(c: usize, t: usize) -> TestResult {
    if c == 0 { return TestResult::discard() }
    let mut p = Progress::new(c, t);
    p.decrement();
    TestResult::from_bool(p.current() == c - 1)
}

#[quickcheck]
fn do_not_decrement_below_zero(t: usize) -> bool {
    let mut p = Progress::new(0, t);
    p.decrement();
    p.current() == 0
}

#[quickcheck]
fn not_finished_while_in_progress(c: usize, t: usize) -> TestResult {
    if c >= t { return TestResult::discard() }
    let p = Progress::new(c, t);
    TestResult::from_bool(!p.finished())
}

#[quickcheck]
fn finished_when_finished(t: usize) -> bool {
    let p = Progress::new(t, t);
    p.finished()
}
