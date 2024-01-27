use crate::models::TModel::*;
use crate::utils;
use chrono::NaiveDateTime;
use rusqlite::{params, params_from_iter, Result};
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
        let conn = utils::common::common::get_conn()?;
        let r: i32 = conn.query_row(
            "SELECT count(*) from accounts where name=?",
            params![input.name],
            |row| row.get(0),
        )?;
        Ok(r > 0)
    }
}

impl TModel<AccountsModelInput, AccountsModelOutput> for AccountsModel {
    fn get_list(
        input: &mut AccountsModelInput,
    ) -> rusqlite::Result<Vec<AccountsModelOutput>, rusqlite::Error> {
        let conn = utils::common::common::get_conn()?;

        let mut sql = String::from("SELECT * from accounts where 1=1 ");
        let mut arr: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

        if !input.id.is_none() {
            sql += " and id=?";
            arr.push(Box::new(input.id));
        }
        if !input.name.is_empty() && !input.pwd.is_empty() {
            sql += " and name=? and pwd=? ";
            arr.push(Box::new(&input.name));
            arr.push(Box::new(&input.pwd));
        }

        let mut stmt = conn.prepare(&sql)?;
        let accounts = stmt
            .query_map(params_from_iter(arr.iter()), |row| {
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
        let conn = utils::common::common::get_conn()?;
        if Self::exists_user(input).unwrap() {
            return Ok(false);
        }
        let now = utils::common::common::now();
        let res = conn.execute(
            "insert into accounts(name,pwd,create_date) values(?,?,?) ",
            params![input.name, input.pwd, now],
        )?;
        Ok(res > 0)
    }
    fn update(input: &mut AccountsModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        let conn = utils::common::common::get_conn()?;
        let mut arr: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

        if !input.id.is_none() {
            panic!("accounts update params:id is none");
        }
        let mut sql = vec![];
        if !input.name.is_empty() {
            sql.push(" name=? ");
            arr.push(Box::new(&input.name));
        }
        if !input.pwd.is_empty() {
            sql.push(" pwd=? ");
            arr.push(Box::new(&input.pwd));
        }
        let res = conn.execute(
            &format!(
                "update accounts set {} where id={:?}",
                sql.join(","),
                &input.id
            ),
            params_from_iter(arr.iter()),
        )?;

        Ok(res > 0)
    }
    fn delete(input: &mut AccountsModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        let conn = utils::common::common::get_conn()?;
        let res = conn.execute("delete from accounts where id=$1", params![input.id])?;
        Ok(res > 0)
    }
}
