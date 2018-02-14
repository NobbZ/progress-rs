extern crate progress;

use progress::Progress;

#[test]
fn generated_with_default_values() {
    let p = Progress::default();

    assert_eq!(0, p.current());
    assert_eq!(100, p.total());
}

#[test]
fn creates_progress_bar() {
    let p = Progress::new("", 5, 14);

    assert_eq!(5, p.current());
    assert_eq!(14, p.total());
}

#[test]
fn creates_caption() {
    let p = Progress::new("Test", 0, 100);

    assert_eq!("Test".to_owned(), *p.caption());
}

#[test]
fn increments_one() {
    let mut p = Progress::new("", 1, 192);

    assert_eq!(1, p.current());
    p.increment();
    assert_eq!(2, p.current());
}

#[test]
fn do_not_increment_above_total() {
    let mut p = Progress::new("", 100, 100);
    assert_eq!(100, p.current());
    p.increment();
    assert_eq!(100, p.current());
}

#[test]
fn decrements_one() {
    let mut p = Progress::new("", 235, 1232);
    assert_eq!(235, p.current());
    p.decrement();
    assert_eq!(234, p.current());
}

#[test]
fn do_not_decrement_below_zero() {
    let mut p = Progress::new("", 0, 124);
    assert_eq!(0, p.current());
    p.decrement();
    assert_eq!(0, p.current());
}

#[test]
fn not_finished_while_in_progress() {
    let p = Progress::new("", 1, 345);
    assert!(!p.finished());
}

#[test]
fn finished_when_finished() {
    let p = Progress::new("", 124, 124);
    assert!(p.finished());
}

#[test]
fn advances_accordingly() {
    let mut p = Progress::default();
    assert_eq!(0, p.current());
    p.forward(50);
    assert_eq!(50, p.current());
    p.backward(25);
    assert_eq!(25, p.current());
    p.forward(75);
    assert_eq!(100, p.current());
}

#[test]
fn backwards_does_not_get_below_zero() {
    let mut p = Progress::default();
    assert_eq!(0, p.current());
    p.forward(50);
    assert_eq!(50, p.current());
    p.backward(75);
    assert_eq!(0, p.current());
}

#[test]
fn forward_does_not_advance_above_total() {
    let mut p = Progress::default();
    assert_eq!(0, p.current());
    p.forward(150);
    assert_eq!(100, p.current());
}
