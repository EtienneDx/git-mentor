use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Assignment {
  pub id: i32,
  pub group_id: i32,
  pub base_repo_id: i32,
  pub test_repo_id: Option<i32>,
  pub correction_repo_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAssignment {
  pub group_id: i32,
  pub base_repo_id: i32,
  pub test_repo_id: i32,
  pub correction_repo_id: i32,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Commenttype"]
pub enum Commenttype {
  Default,
  Response,
  Line,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Commentauthor"]
pub enum Commentauthor {
  User,
  Automated,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
  pub id: i32,
  pub repository_id: i32,
  pub commit_hash: String,
  pub comment_type: Commenttype,
  pub message: String,
  pub author_type: Commentauthor,
  pub author_id: Option<i32>,
  pub date: std::time::SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewComment<'a> {
  pub repository_id: i32,
  pub commit_hash: &'a str,
  pub comment_type: &'a Commenttype,
  pub message: &'a str,
  pub author_type: &'a Commentauthor,
  pub author_id: i32,
  pub date: &'a std::time::SystemTime,
}

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::Status)]
pub enum Status {
  Success,
  Pending,
  Cancelled,
  Failed,
}

impl ToSql<crate::schema::sql_types::Status, Pg> for Status {
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
    match *self {
      Status::Success => out.write_all(b"success")?,
      Status::Pending => out.write_all(b"pending")?,
      Status::Cancelled => out.write_all(b"cancelled")?,
      Status::Failed => out.write_all(b"failed")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<crate::schema::sql_types::Status, Pg> for Status {
  fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
    match bytes.as_bytes() {
      b"user" => Ok(Status::Success),
      b"automated" => Ok(Status::Pending),
      b"cancelled" => Ok(Status::Cancelled),
      b"failed" => Ok(Status::Failed),
      _ => Err("Unrecognized enum variant".into()),
    }
  }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::cirun)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cirun {
  pub id: i32,
  pub repository_id: i32,
  pub commit: String,
  pub status: Status,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::cirun)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCirun<'a> {
  pub repository_id: i32,
  pub commit: &'a str,
  pub status: &'a Status,
}
