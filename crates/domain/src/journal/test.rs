use chrono::{NaiveDate, Utc};

use super::*;
use crate::{
    account::DebitCredit::{self, Credit, Debit},
    money::{
        Currency::{self},
        Money,
    },
};

fn create_test_je(jls: Vec<JournalLine>) -> JournalEntry {
    let uuid = uuid::Uuid::now_v7();

    JournalEntry::new(
        uuid,
        "test".to_string(),
        jls,
        NaiveDate::MIN,
        Utc::now(),
        Currency::USD,
    )
}

fn create_test_jl(side: DebitCredit, amt: Money) -> JournalLine {
    let uuid = uuid::Uuid::now_v7();
    JournalLine::new(uuid, side, amt).unwrap()
}

#[test]
fn empty_journal_entry() {
    let je = create_test_je(Vec::new());
    assert_eq!(je.validate(), Err(JournalError::EmptyJournalEntry));
}

#[test]
fn one_debit_line() {
    let jl = create_test_jl(Debit, Money::new(19, Currency::USD));
    let je = create_test_je(Vec::from([jl]));
    assert!(matches!(
        je.validate(),
        Err(JournalError::ImbalancedJournalEntry { .. })
    ));
}

#[test]
fn equal_lines() {
    let dl = create_test_jl(Debit, Money::new(19, Currency::USD));
    let cl = create_test_jl(Credit, Money::new(19, Currency::USD));
    let je = create_test_je(Vec::from([dl, cl]));
    assert_eq!(je.validate(), Ok(()));
}

#[test]
fn unequal_lines() {
    let dl = create_test_jl(Debit, Money::new(19, Currency::USD));
    let cl = create_test_jl(Credit, Money::new(21, Currency::USD));
    let je = create_test_je(Vec::from([dl, cl]));

    assert!(matches!(
        je.validate(),
        Err(JournalError::ImbalancedJournalEntry { .. })
    ));
}

#[test]
fn currency_mismatch() {
    let dl = create_test_jl(Debit, Money::new(1, Currency::USD));
    let cl = create_test_jl(Credit, Money::new(1, Currency::EUR));
    let je = create_test_je(Vec::from([dl, cl]));

    assert!(matches!(je.validate(), Err(JournalError::Money { .. })));
}

#[test]
fn int_overflow() {
    let dl = create_test_jl(Debit, Money::new(u64::MAX, Currency::USD));
    let cl = create_test_jl(Debit, Money::new(1, Currency::USD));
    let je = create_test_je(Vec::from([dl, cl]));

    assert!(matches!(je.validate(), Err(JournalError::Money { .. })));
}

#[test]
fn zero_journal_line() {
    let uuid = uuid::Uuid::now_v7();
    let jl = JournalLine::new(uuid, Debit, Money::new(0, Currency::USD));
    assert!(matches!(jl, Err(JournalError::ZeroAmountJournalLine)));
}
