use uuid::Uuid;

use crate::account::DebitCredit::*;

pub enum DebitCredit {
    Debit,
    Credit,
}

pub enum AccountClassification {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

pub struct Account {
    id: Uuid,
    name: String,
    classification: AccountClassification,
}

impl AccountClassification {
    pub fn normal_balance(&self) -> DebitCredit {
        match self {
            AccountClassification::Asset => Debit,
            AccountClassification::Expense => Debit,
            AccountClassification::Revenue => Credit,
            AccountClassification::Liability => Credit,
            AccountClassification::Equity => Credit,
        }
    }
}
