mod models;
mod utils;
use models::*;
use utils::AppConfig::*;

// 使用内建的测试框架编写测试
#[cfg(test)]
mod tests {
    use crate::utils::common::common;

    use self::{accounts::*, files::*, TModel::TModel};

    use super::*;

    // #[test]
    // fn SQL() -> Result<(), Box<dyn std::error::Error>> {
    //     let conn = common::get_conn()?;
    //     let res = conn.execute("
    //     create table person (
    //         id int,
    //         name varchar(50),
    //         age int
    //     )
    //     ", [])?;
    //     // let res = conn.execute(
    //     //     "
    //     // CREATE TABLE IF NOT EXISTS accounts (
    //     //     id INTEGER PRIMARY KEY AUTOINCREMENT,
    //     //     name VARCHAR(50),
    //     //     pwd VARCHAR(50),
    //     //     create_date datetime
    //     // );
    //     // INSERT INTO accounts(name,pwd,create_date) values('admin','123456','2024-01-01 00:00:00');
    //     // CREATE TABLE IF NOT EXISTS files (
    //     //     id INTEGER PRIMARY KEY AUTOINCREMENT,
    //     //     display_name VARCHAR(50),
    //     //     file_name VARCHAR(50),
    //     //     dir_name VARCHAR(50),
    //     //     create_date datetime
    //     // );
    //     // ",
    //     //     [],
    //     // )?;
    //     println!("{:?}", res);
    //     Ok(())
    // }

    #[test]
    fn test1() {
        let mut model = AccountsModelInput {
            name: "".to_string(),
            pwd: "".to_string(),
            id: None,
        };
        if let Ok(res) = accounts::AccountsModel::get_list(&mut model) {
            println!("{:#?}", res);
        } else {
            println!("empty...");
        }
    }
    #[test]
    fn test2() {
        let mut model = AccountsModelInput {
            name: "admin".to_string(),
            pwd: "".to_string(),
            id: None,
        };
        if let Ok(res) = accounts::AccountsModel::exists_user(&mut model) {
            println!("{:#?}", res);
        } else {
            println!("empty...");
        }
    }
    #[test]
    fn test3() {
        let mut model = AccountsModelInput {
            name: "admin".to_string(),
            pwd: "123456".to_string(),
            id: None,
        };
        if let Ok(res) = accounts::AccountsModel::insert(&mut model) {
            println!("{:?}", res);
        }
    }
    #[test]
    fn test4() {
        let mut model = FilesModelInput {
            id: None,
            display_name: "".to_string(),
            file_name: "".to_string(),
        };
        if let Ok(res) = FilesModel::get_list(&mut model) {
            println!("{:?}", res);
        }
    }
}
