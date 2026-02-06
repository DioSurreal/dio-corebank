// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (account_id) {
        account_id -> Int8,
        customer_id -> Int8,
        #[max_length = 50]
        account_number -> Varchar,
        balance -> Numeric,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cheque_transactions (cheque_id) {
        cheque_id -> Int8,
        #[max_length = 100]
        cheque_number -> Varchar,
        #[max_length = 50]
        issuing_bank_code -> Varchar,
        #[max_length = 100]
        issuing_account_number -> Varchar,
        amount -> Numeric,
        depositor_account_id -> Int8,
        #[max_length = 50]
        status -> Varchar,
        received_at -> Timestamp,
        cleared_at -> Nullable<Timestamp>,
        settled_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    clearing_cheque_batch (clearing_cheque_id) {
        clearing_cheque_id -> Int8,
        cheque_id -> Int8,
        account_id -> Int8,
        amount -> Numeric,
        #[max_length = 50]
        bank_code -> Varchar,
        job_id -> Int8,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    clearing_jobs (job_id) {
        job_id -> Int8,
        #[max_length = 50]
        clearing_type -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    clearing_transactions_batch (clearing_item_id) {
        clearing_item_id -> Int8,
        transaction_id -> Int8,
        account_id -> Int8,
        amount -> Numeric,
        #[max_length = 50]
        bank_code -> Varchar,
        job_id -> Int8,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    customers (customer_id) {
        customer_id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    fees (fee_id) {
        fee_id -> Int8,
        #[max_length = 100]
        fee_type -> Varchar,
        rate -> Numeric,
        active -> Bool,
        #[max_length = 100]
        updated_by -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    ledger_entries (entry_id) {
        entry_id -> Int8,
        transaction_id -> Int8,
        account_id -> Int8,
        #[max_length = 10]
        direction -> Varchar,
        amount -> Numeric,
        created_at -> Timestamp,
    }
}

diesel::table! {
    system_accounts (system_account_id) {
        system_account_id -> Int8,
        #[max_length = 50]
        code -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    transactions (transaction_id) {
        transaction_id -> Int8,
        #[max_length = 100]
        idempotency_key -> Varchar,
        #[max_length = 50]
        transaction_type -> Varchar,
        #[max_length = 50]
        channel -> Varchar,
        amount -> Numeric,
        #[max_length = 50]
        receiver_bank_code -> Varchar,
        from_account_id -> Nullable<Int8>,
        to_account_id -> Nullable<Int8>,
        #[max_length = 50]
        status -> Varchar,
        requested_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(accounts -> customers (customer_id));
diesel::joinable!(cheque_transactions -> accounts (depositor_account_id));
diesel::joinable!(clearing_cheque_batch -> accounts (account_id));
diesel::joinable!(clearing_cheque_batch -> cheque_transactions (cheque_id));
diesel::joinable!(clearing_cheque_batch -> clearing_jobs (job_id));
diesel::joinable!(clearing_transactions_batch -> accounts (account_id));
diesel::joinable!(clearing_transactions_batch -> clearing_jobs (job_id));
diesel::joinable!(clearing_transactions_batch -> transactions (transaction_id));
diesel::joinable!(ledger_entries -> accounts (account_id));
diesel::joinable!(ledger_entries -> transactions (transaction_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    cheque_transactions,
    clearing_cheque_batch,
    clearing_jobs,
    clearing_transactions_batch,
    customers,
    fees,
    ledger_entries,
    system_accounts,
    transactions,
);
