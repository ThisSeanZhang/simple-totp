use diesel::prelude::*;
use crate::schema::totp_keys;

#[derive(Queryable, PartialEq, Selectable, Debug)]
#[diesel(table_name = totp_keys)]
pub struct Totp {
    pub id: i32,
    pub taget: String,
    pub secret_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = totp_keys)]
pub struct NewTotp {
    pub taget: String,
    pub secret_key: String,
}
