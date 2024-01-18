use crate::models::TModel::*;
use crate::utils;
use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseError, Utc};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AccountsModel {
    pub id: i32,
    pub name: String,
    pub pwd: String,
    pub create_date: Option<NaiveDateTime>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AccountsModelInput {
    pub name: String,
    pub pwd: String,
}
#[derive(Debug)]
pub struct AccountsModelOutput {
    pub id: i32,
    pub name: String,
    pub pwd: String,
    pub create_date: Option<NaiveDateTime>,
}

impl TModel<AccountsModelInput, AccountsModelOutput> for AccountsModel {
    fn get_model(
        input: AccountsModelInput,
    ) -> rusqlite::Result<Vec<AccountsModelOutput>, rusqlite::Error> {
        let conn = utils::db::DB::get_conn()?;
        let mut stmt = conn.prepare("SELECT * from accounts where name=?1 and pwd=?2")?;
        let accounts = stmt
            .query_map(params![input.name, input.pwd], |row| {
                let create_date_str: String = row.get(3)?;
                let create_date =
                    NaiveDateTime::parse_from_str(&create_date_str, "%Y-%m-%d %H:%M:%S");
                Ok(AccountsModelOutput {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    pwd: row.get(2)?,
                    create_date: create_date.ok(),
                })
            })?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(accounts)
    }
}
