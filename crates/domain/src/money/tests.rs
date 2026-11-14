use super::*;

#[test]
fn money_add_two_values() {
    let m1 = Money::new(10, Currency::USD);
    let m2 = Money::new(20, Currency::USD);

    let m3 = m1.checked_add(&m2).unwrap();

    assert_eq!(m3, Money::new(30, Currency::USD));
}

#[test]
fn money_add_zero() {
    let m1 = Money::new(10, Currency::USD);
    let m2 = Money::new(0, Currency::USD);

    let m3 = m1.checked_add(&m2).unwrap();

    assert_eq!(m3, Money::new(10, Currency::USD));
}

#[test]
fn money_add_overflow() {
    let m1 = Money::new(u64::MAX, Currency::USD);
    let m2 = Money::new(1, Currency::USD);

    assert_eq!(m1.checked_add(&m2), Err(MoneyError::MoneyOverflow));
}

#[test]
fn money_checked_add_preserves_operands() {
    let m1 = Money::new(100, Currency::USD);
    let m2 = Money::new(1, Currency::USD);

    let _res = m1.checked_add(&m2);

    assert_eq!(m1, Money::new(100, Currency::USD));
    assert_eq!(m2, Money::new(1, Currency::USD));
}
