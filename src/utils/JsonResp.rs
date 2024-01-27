use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResp<T>
where
    T: salvo::Scribe,
{
    pub status: i32,
    pub data: Option<T>,
}