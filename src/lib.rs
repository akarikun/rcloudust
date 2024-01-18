mod models;
mod utils;
use models::*;
use utils::AppConfig::*;

// 使用内建的测试框架编写测试
#[cfg(test)]
mod tests {
    use crate::utils::db::DB;

    use self::{accounts::*, TModel::TModel};

    use super::*;

    #[test]
    fn SQL() -> Result<(), Box<dyn std::error::Error>> {
        let conn = DB::get_conn()?;
        let res = conn.execute(
            "
        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT VARCHAR(50),
            pwd TEXT VARCHAR(50),
            create_date datetime
        );
        INSERT INTO accounts(name,pwd,create_date) values('admin','123456','2024-01-01 00:00:00');
        ",
            [],
        )?;
        println!("{:?}",res);
        Ok(())
    }

    #[test]
    fn test1() {
        if let Ok(res) = accounts::AccountsModel::get_model(AccountsModelInput {
            name: "admin".to_string(),
            pwd: "123456".to_string(),
        }) {
            println!("{:#?}", res);
        } else {
            println!("empty...");
        }
    }
}
