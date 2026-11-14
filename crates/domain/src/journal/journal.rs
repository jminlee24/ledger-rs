use chrono::{DateTime, NaiveDate, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    account::DebitCredit,
    journal::JournalError::*,
    money::{Currency, Money, MoneyError},
};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum JournalError {
    #[error("Empty Journal Entry")]
    EmptyJournalEntry,
    #[error("Imbalanced Journal Entry: credit: {0}, debit: {1}")]
    ImbalancedJournalEntry(Money, Money),
    #[error("Journal Line cannot have zero amount")]
    ZeroAmountJournalLine,
    #[error("Money Error failure: {0}")]
    Money(#[from] MoneyError),
}

enum JournalEntryStatus {
    Draft,
    Balanced,
    Invalid,
    Posted,
}

pub struct JournalLine {
    account_id: Uuid,
    side: DebitCredit,
    amount: Money,
}

impl JournalLine {
    pub fn new(account_id: Uuid, side: DebitCredit, amount: Money) -> Result<Self, JournalError> {
        if amount == Money::new(0, *amount.currency()) {
            return Err(JournalError::ZeroAmountJournalLine);
        }
        Ok(Self {
            account_id,
            side,
            amount,
        })
    }
}

pub struct JournalEntry {
    entry_id: Uuid,
    memo: String,
    lines: Vec<JournalLine>,
    transaction_date: NaiveDate,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    status: JournalEntryStatus,
    currency: Currency,
}

impl JournalEntry {
    pub fn new(
        entry_id: Uuid,
        memo: String,
        lines: Vec<JournalLine>,
        transaction_date: NaiveDate,
        created_at: DateTime<Utc>,
        currency: Currency,
    ) -> Self {
        Self {
            entry_id,
            memo,
            lines,
            transaction_date,
            created_at,
            updated_at: created_at,
            status: JournalEntryStatus::Draft,
            currency,
        }
    }

    pub fn validate(&self) -> Result<(), JournalError> {
        if self.lines.is_empty() {
            return Err(EmptyJournalEntry);
        }

        self.is_balanced()
    }

    fn is_balanced(&self) -> Result<(), JournalError> {
        let debit = Money::new(0, self.currency);
        let credit = Money::new(0, self.currency);

        let (debit, credit) = self.lines.iter().try_fold(
            (debit, credit),
            |acc, val| -> Result<(Money, Money), MoneyError> {
                match val.side {
                    DebitCredit::Debit => {
                        let new_debit = acc.0.checked_add(&val.amount)?;
                        Ok((new_debit, acc.1))
                    }
                    DebitCredit::Credit => {
                        let new_credit = acc.1.checked_add(&val.amount)?;
                        Ok((acc.0, new_credit))
                    }
                }
            },
        )?;

        if debit != credit {
            return Err(ImbalancedJournalEntry(credit, debit));
        }

        let zero_money = Money::new(0, self.currency);

        if debit == zero_money || credit == zero_money {
            return Err(ImbalancedJournalEntry(credit, debit));
        }

        Ok(())
    }
}
