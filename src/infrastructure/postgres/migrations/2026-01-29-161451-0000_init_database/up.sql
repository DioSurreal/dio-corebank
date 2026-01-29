-- Your SQL goes here
-- =========================
-- Extensions
-- =========================
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =========================
-- customers
-- =========================
CREATE TABLE customers (
    customer_id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

-- =========================
-- accounts
-- =========================
CREATE TABLE accounts (
    account_id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(customer_id),
    account_number VARCHAR(50) NOT NULL UNIQUE,
    currency CHAR(3) NOT NULL,
    balance NUMERIC(18,2) NOT NULL DEFAULT 0,
    available_balance NUMERIC(18,2) NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX idx_accounts_customer_id ON accounts(customer_id);

-- =========================
-- transactions
-- =========================
CREATE TABLE transactions (
    transaction_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    idempotency_key VARCHAR(100) NOT NULL UNIQUE,
    transaction_type VARCHAR(50) NOT NULL,
    channel VARCHAR(50) NOT NULL,
    from_account_id BIGINT REFERENCES accounts(account_id),
    to_account_id BIGINT REFERENCES accounts(account_id),
    status VARCHAR(50) NOT NULL,
    requested_at TIMESTAMP NOT NULL DEFAULT now(),
    completed_at TIMESTAMP
);

CREATE INDEX idx_transactions_from_account ON transactions(from_account_id);
CREATE INDEX idx_transactions_to_account ON transactions(to_account_id);

-- =========================
-- ledger_entries
-- =========================
CREATE TABLE ledger_entries (
    entry_id BIGSERIAL PRIMARY KEY,
    transaction_id UUID NOT NULL REFERENCES transactions(transaction_id),
    account_id BIGINT NOT NULL REFERENCES accounts(account_id),
    direction VARCHAR(10) NOT NULL,
    amount NUMERIC(18,2) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    CONSTRAINT chk_ledger_amount_positive CHECK (amount > 0),
    CONSTRAINT chk_ledger_direction CHECK (direction IN ('DEBIT','CREDIT'))
);

CREATE INDEX idx_ledger_account_id ON ledger_entries(account_id);
CREATE INDEX idx_ledger_transaction_id ON ledger_entries(transaction_id);

-- =========================
-- fees
-- =========================
CREATE TABLE fees (
    fee_id BIGSERIAL PRIMARY KEY,
    fee_type VARCHAR(100) NOT NULL,
    rate NUMERIC(8,4) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    updated_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

-- =========================
-- cheque_transactions
-- =========================
CREATE TABLE cheque_transactions (
    cheque_id BIGSERIAL PRIMARY KEY,
    cheque_number VARCHAR(100) NOT NULL,
    issuing_bank_code VARCHAR(50) NOT NULL,
    issuing_account_number VARCHAR(100) NOT NULL,
    amount NUMERIC(18,2) NOT NULL,

    depositor_account_id BIGINT NOT NULL REFERENCES accounts(account_id),

    status VARCHAR(50) NOT NULL,

    received_at TIMESTAMP NOT NULL DEFAULT now(),
    cleared_at TIMESTAMP,
    settled_at TIMESTAMP,

    CONSTRAINT uq_cheque UNIQUE (cheque_number, issuing_bank_code)
);

CREATE INDEX idx_cheque_status ON cheque_transactions(status);

-- =========================
-- clearing_jobs
-- =========================
CREATE TABLE clearing_jobs (
    job_id BIGSERIAL PRIMARY KEY,
    clearing_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

-- =========================
-- clearing_items
-- =========================
CREATE TABLE clearing_items (
    clearing_item_id BIGSERIAL PRIMARY KEY,
    job_id BIGINT NOT NULL REFERENCES clearing_jobs(job_id),
    transaction_id UUID NOT NULL REFERENCES transactions(transaction_id),
    account_id BIGINT NOT NULL REFERENCES accounts(account_id),
    amount NUMERIC(18,2) NOT NULL,
    bank_code VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    completed_at TIMESTAMP
);

CREATE INDEX idx_clearing_items_job_id ON clearing_items(job_id);
CREATE INDEX idx_clearing_items_account_id ON clearing_items(account_id);

-- =========================
-- system accounts (optional but recommended)
-- =========================
CREATE TABLE system_accounts (
    system_account_id BIGSERIAL PRIMARY KEY,
    code VARCHAR(50) UNIQUE NOT NULL,
    description VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

-- =========================
-- seed system accounts
-- =========================
INSERT INTO system_accounts (code, description) VALUES
('CASH_VAULT', 'Physical cash vault'),
('ATM_SETTLEMENT', 'ATM settlement account'),
('INTERBANK_SUSPENSE', 'Interbank clearing suspense account'),
('CHEQUE_SUSPENSE', 'Cheque clearing suspense account');

