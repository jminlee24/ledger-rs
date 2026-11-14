use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

use crate::money::MoneyError::{InvalidCurrency, MoneyOverflow};

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Currency {
    USD,
    EUR
}

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoneyError {
    #[error("Int overflow")]
    MoneyOverflow,
    #[error("Invalid Money value")]
    InvalidMoneyValue,
    #[error("Invalid Currency: {0} {1}")]
    InvalidCurrency(Currency, Currency),
    #[error("unknown money error")]
    Unknown,
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Money {
    amount: u64,
    currency: Currency,
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Money {
    pub fn new(amount: u64, currency: Currency) -> Self {
        Self { amount, currency }
    }
    pub fn checked_add(&self, other: &Self) -> Result<Self, MoneyError> {
        if self.currency != other.currency {
            Err(InvalidCurrency(self.currency, other.currency))
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
