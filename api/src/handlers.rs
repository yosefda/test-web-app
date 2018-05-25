use actix_web::{Json, Result};

use todo;

pub fn new_todo(payload: Json<todo::Todo>) -> Result<String> {
    Ok(format!("{:?}", payload))
}