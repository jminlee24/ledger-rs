use uuid::Uuid;

use crate::{account::DebitCredit, money::{Currency, Money}};

struct JournalLine {
    account_id: Uuid,
    side: DebitCredit,
    amount: Money,
    currency: Currency,
}
