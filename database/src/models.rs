use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub password: String,
  pub pubkey: Vec<Option<String>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub email: &'a str,
  pub password: &'a str,
  pub pubkey: &'a Vec<Option<String>>,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Repotype"]
pub enum Repotype {
  Default,
  Ci,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::repositories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Repository {
  pub id: i32,
  pub name: String,
  pub repo_type: Repotype,
  pub owner_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::repositories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRepository<'a> {
  pub name: &'a str,
  pub repo_type: &'a Repotype,
  pub owner_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
  pub id: i32,
  pub teacher_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGroup {
  pub teacher_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::group_students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupStudent {
  pub group_id: i32,
  pub student_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::group_students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGroupStudent {
  pub group_id: i32,
  pub student_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Assignment {
  pub id: i32,
  pub group_id: i32,
  pub base_repo_id: i32,
  pub test_repo_id: i32,
  pub correction_repo_id: i32,
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
  pub author_id: i32,
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
