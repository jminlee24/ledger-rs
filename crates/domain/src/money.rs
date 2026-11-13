use thiserror::Error;

use crate::money::MoneyError::{InvalidCurrency, MoneyOverflow};

#[derive(Error, Debug)]
pub enum MoneyError {
    #[error("Int overflow")]
    MoneyOverflow,
    #[error("Invalid Money value")]
    InvalidMoneyValue,
    #[error("Invalid Currency")]
    InvalidCurrency,
    #[error("unknown money error")]
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Currency {
    USD,
}

#[derive(Debug, PartialEq)]
pub struct Money {
    amount: u64,
    currency: Currency,
}

impl Money {
    pub fn new(amount: u64, currency: Currency) -> Self {
        Self { amount, currency }
    }
    pub fn checked_add(&self, other: &Self) -> Result<Self, MoneyError> {
        if self.currency != other.currency {
            Err(InvalidCurrency)
        } else {
            match self.amount.checked_add(other.amount) {
                None => Err(MoneyOverflow),
                Some(v) => Ok(Self {
                    amount: v,
                    currency: self.currency,
                }),
            }
        }
    }
}
