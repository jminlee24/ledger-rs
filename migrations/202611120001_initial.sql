-- Initial relational schema. Business rules and posting operations are intentionally NOT implemented.
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE accounts (
 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
 code TEXT NOT NULL UNIQUE,
 name TEXT NOT NULL,
 kind TEXT NOT NULL CHECK (kind IN ('asset','liability','equity','income','expense')),
 parent_id UUID REFERENCES accounts(id),
 currency CHAR(3) NOT NULL DEFAULT 'USD',
 active BOOLEAN NOT NULL DEFAULT TRUE,
 created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE journal_entries (
 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
 transaction_date DATE NOT NULL,
 description TEXT NOT NULL,
 status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft','posted')),
 reversal_of UUID UNIQUE REFERENCES journal_entries(id),
 recorded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 modified_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 posted_at TIMESTAMPTZ,
 external_id TEXT UNIQUE
);

CREATE TABLE journal_lines (
 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
 entry_id UUID NOT NULL REFERENCES journal_entries(id) ON DELETE RESTRICT,
 account_id UUID NOT NULL REFERENCES accounts(id),
 side TEXT NOT NULL CHECK (side IN ('debit','credit')),
 amount NUMERIC(20,2) NOT NULL CHECK (amount > 0),
 memo TEXT
);
CREATE INDEX journal_lines_entry_idx ON journal_lines(entry_id);
CREATE INDEX journal_lines_account_idx ON journal_lines(account_id);
CREATE INDEX journal_entries_date_idx ON journal_entries(transaction_date);

CREATE TABLE budget_pools (
 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
 name TEXT NOT NULL UNIQUE,
 kind TEXT NOT NULL CHECK (kind IN ('recurring','sinking_fund','reserve')),
 active BOOLEAN NOT NULL DEFAULT TRUE,
 created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE budget_periods (
 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
 start_date DATE NOT NULL,
 end_date DATE NOT NULL,
 status TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open','closed')),
 closed_at TIMESTAMPTZ,
 CHECK (end_date >= start_date),
 UNIQUE (start_date, end_date)
);

CREATE TABLE budget_targets (
 period_id UUID NOT NULL REFERENCES budget_periods(id),
 pool_id UUID NOT NULL REFERENCES budget_pools(id),
 amount NUMERIC(20,2) NOT NULL CHECK (amount >= 0),
 PRIMARY KEY (period_id, pool_id)
);

CREATE TABLE budget_movements (
 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
 pool_id UUID NOT NULL REFERENCES budget_pools(id),
 transaction_date DATE NOT NULL,
 amount NUMERIC(20,2) NOT NULL CHECK (amount <> 0),
 kind TEXT NOT NULL CHECK (kind IN ('allocation','spending','transfer','adjustment')),
 journal_entry_id UUID REFERENCES journal_entries(id),
 transfer_group_id UUID,
 memo TEXT,
 recorded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 modified_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX budget_movements_pool_date_idx ON budget_movements(pool_id, transaction_date);

CREATE TABLE period_snapshots (
 period_id UUID NOT NULL REFERENCES budget_periods(id),
 pool_id UUID NOT NULL REFERENCES budget_pools(id),
 opening_balance NUMERIC(20,2) NOT NULL,
 allocated NUMERIC(20,2) NOT NULL,
 spent NUMERIC(20,2) NOT NULL,
 closing_balance NUMERIC(20,2) NOT NULL,
 captured_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 PRIMARY KEY (period_id, pool_id)
);

CREATE TABLE audit_events (
 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
 entity_type TEXT NOT NULL,
 entity_id UUID NOT NULL,
 action TEXT NOT NULL,
 recorded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 before_state JSONB,
 after_state JSONB
);
CREATE INDEX audit_events_entity_idx ON audit_events(entity_type, entity_id);

-- TODO: enforce balanced posted entries atomically in application posting transaction.
-- TODO: prevent modification/deletion of posted journal entries and lines.
-- TODO: maintain modified_at via application or triggers.
-- TODO: define budget transfer and reconciliation invariants.
