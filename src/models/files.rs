use crate::models::TModel::*;
use crate::utils;
use chrono::NaiveDateTime;
use rusqlite::{params, params_from_iter, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FilesModel {
    pub id: i32,
    pub display_name: String,
    pub file_name: String,
    pub create_date: Option<NaiveDateTime>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FilesModelInput {
    pub id: Option<i32>,
    pub display_name: String,
    pub file_name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FilesModelOutput {
    pub id: i32,
    pub display_name: String,
    pub file_name: String,
    pub create_date: Option<NaiveDateTime>,
}

impl TModel<FilesModelInput, FilesModelOutput> for FilesModel {
     fn get_list(
        input: &mut FilesModelInput,
    ) -> rusqlite::Result<Vec<FilesModelOutput>, rusqlite::Error> {
        let conn = utils::common::common::get_conn()?;
        
        let mut sql = String::from("SELECT * from accounts where 1=1 ");
        let mut arr: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];
        if !input.id.is_none() {
            sql += " and id=?";
            arr.push(Box::new(input.id));
        }
        if !input.file_name.is_empty() {
            sql += " and file_name like ? ";
            arr.push(Box::new(format!("%{}%", input.file_name)));
        }
        if !input.display_name.is_empty() {
            sql += " and display_name like ? ";
            arr.push(Box::new(format!("%{}%", input.file_name)));
        }
        let mut stmt = conn.prepare(&sql)?;
        let models = stmt
            .query_map(params_from_iter(arr.iter()), |row| {
                let create_date_str: String = row.get(3)?;
                let create_date =
                    NaiveDateTime::parse_from_str(&create_date_str, "%Y-%m-%d %H:%M:%S");
                Ok(FilesModelOutput {
                    id: row.get(0)?,
                    display_name: row.get(1)?,
                    file_name: row.get(2)?,
                    create_date: create_date.ok(),
                })
            })?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(models)
    }

    fn insert(input: &mut FilesModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        let conn = utils::common::common::get_conn()?;
        let now = chrono::Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let res = conn.execute(
            "insert into files(display_name,file_name,create_date) values(?,?,?); ",
            params![input.display_name, input.file_name, now],
        )?;
        Ok(res > 0)
    }

    fn update(input: &mut FilesModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        // let conn = utils::common::common::get_conn()?;
        // let res = conn.execute(
        //     "update files set display_name=$,file_name=$ where id=$;",
        //     params![input.display_name, input.file_name, input.id],
        // )?;
        // Ok(res > 0)
        let conn = utils::common::common::get_conn()?;
        let mut arr: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

        if !input.id.is_none() {
            panic!("accounts update params:id is none");
        }
        let mut sql = vec![];
        if !input.display_name.is_empty() {
            sql.push(" display_name=? ");
            arr.push(Box::new(format!("%{}%", input.display_name)));
        }
        if !input.file_name.is_empty() {
            sql.push(" file_name=? ");
            arr.push(Box::new(format!("%{}%", input.file_name)));
        }
        let res = conn.execute(
            &format!(
                "update files set {} where id={:?}",
                sql.join(","),
                &input.id
            ),
            params_from_iter(arr.iter()),
        )?;

        Ok(res > 0)
    }

    fn delete(input: &mut FilesModelInput) -> rusqlite::Result<bool, rusqlite::Error> {
        let conn = utils::common::common::get_conn()?;
        let res = conn.execute("delete from files where id=$", params![input.id])?;
        Ok(res > 0)
    }
}
