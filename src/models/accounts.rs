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
    pub id: Option<i32>,
    pub name: String,
    pub pwd: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsModelOutput {
    pub id: i32,
    pub name: String,
    pub pwd: String,
    pub create_date: Option<NaiveDateTime>,
}

impl AccountsModel {
    pub fn exists_user(input: &mut AccountsModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        let conn = utils::db::DB::get_conn()?;
        let r: i32 = conn.query_row(
            "SELECT count(*) from accounts where name=?1",
            params![input.name],
            |row| row.get(0),
        )?;
        Ok(r > 0)
    }
}

impl TModel<AccountsModelInput, AccountsModelOutput> for AccountsModel {
    fn get_model(
        input: &mut AccountsModelInput,
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
    fn insert(input: &mut AccountsModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        let conn = utils::db::DB::get_conn()?;
        if Self::exists_user(input).unwrap() {
            return Ok(false);
        }
        let now = chrono::Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let res = conn.execute(
            "insert into accounts(name,pwd,create_date) values(?1,?2,?3) ",
            params![input.name, input.pwd, now],
        )?;
        Ok(res > 0)
    }
    fn update(input: &mut AccountsModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        let conn = utils::db::DB::get_conn()?;
        let res = conn.execute(
            "update accounts set name=$1,pwd=$2 where id=$3",
            params![input.name, input.pwd, input.id],
        )?;
        Ok(res > 0)
    }
    fn delete(input: &mut AccountsModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        let conn = utils::db::DB::get_conn()?;
        let res = conn.execute("delete accounts where id=$1", params![input.id])?;
        Ok(res > 0)
    }
}
