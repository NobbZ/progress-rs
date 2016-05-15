extern crate progress;

use progress::builder::ProgressBuilder;

#[test]
fn generated_with_default_values() {
    let p = ProgressBuilder::new().build();

    assert_eq!(0, p.current());
    assert_eq!(100, p.total());
}

#[test]
fn creates_progress_bar() {
    let p = ProgressBuilder::new()
        .set_start(5)
        .set_finish(14)
        .build();

    assert_eq!(5, p.current());
    assert_eq!(14, p.total());
}

#[test]
fn increments_one() {
    let mut p = ProgressBuilder::new()
        .set_start(1)
        .set_finish(192)
        .build();

    assert_eq!(1, p.current());
    p.increment();
    assert_eq!(2, p.current());
}

#[test]
fn do_not_increment_above_total() {
    let mut p = ProgressBuilder::new()
        .set_start(100)
        .build();
    assert_eq!(100, p.current());
    p.increment();
    assert_eq!(100, p.current());
}

#[test]
fn decrements_one() {
    let mut p = ProgressBuilder::new()
        .set_start(235)
        .set_finish(1232)
        .build();
    assert_eq!(235, p.current());
    p.decrement();
    assert_eq!(234, p.current());
}

#[test]
fn do_not_decrement_below_zero() {
    let mut p = ProgressBuilder::new()
        .set_finish(124)
        .build();
    assert_eq!(0, p.current());
    p.decrement();
    assert_eq!(0, p.current());
}

#[test]
fn not_finished_while_in_progress() {
    let p = ProgressBuilder::new()
        .set_start(1)
        .set_finish(345)
        .build();
    assert!(!p.finished());
}

#[test]
fn finished_when_finished() {
    let p = ProgressBuilder::new()
        .set_start(124)
        .set_finish(124)
        .build();
    assert!(p.finished());
}
