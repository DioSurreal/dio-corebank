use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::infrastructure::postgres::schema::customers;


#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = customers)]
pub struct CustomerEntity {
    pub customer_id: i64,
    pub name: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = customers)]
pub struct CreatCustomerEntity {
    pub name: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}