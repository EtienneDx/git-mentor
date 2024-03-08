use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod error;
pub mod models;
pub mod schema;

pub mod db_handle;

pub use db_handle::DbHandle;
pub use db_handle::TransactionHandler;

impl Cirun {
  pub fn create(
    conn: &mut PgConnection,
    repository_id: i32,
    commit: &str,
    status: &Status,
  ) -> Cirun {
    use crate::schema::cirun;

    let new_cirun = NewCirun {
      repository_id,
      commit,
      status,
    };

    diesel::insert_into(cirun::table)
      .values(&new_cirun)
      .returning(Cirun::as_returning())
      .get_result(conn)
      .expect("Error saving new cirun")
  }

  pub fn get_with_id(conn: &mut PgConnection, cirun_id: i32) -> Option<Cirun> {
    use crate::schema::cirun::dsl;

    let cirun = dsl::cirun
      .filter(dsl::id.eq(cirun_id))
      .select(Cirun::as_select())
      .first(conn)
      .optional();
    match cirun {
      Ok(cirun) => cirun,
      Err(_) => None,
    }
  }

  pub fn delete(conn: &mut PgConnection, cirun_id: i32) -> bool {
    use crate::schema::cirun::dsl::cirun;

    diesel::delete(cirun.find(cirun_id)).execute(conn).is_ok()
  }
}

impl Comment {
  pub fn create(
    conn: &mut PgConnection,
    repository_id: i32,
    commit_hash: &str,
    comment_type: &Commenttype,
    message: &str,
    author_type: &Commentauthor,
    author_id: i32,
    date: &std::time::SystemTime,
  ) -> Comment {
    use crate::schema::comments;

    let new_comment = NewComment {
      repository_id,
      commit_hash,
      comment_type,
      message,
      author_type,
      author_id,
      date,
    };

    diesel::insert_into(comments::table)
      .values(&new_comment)
      .returning(Comment::as_returning())
      .get_result(conn)
      .expect("Error saving new comment")
  }

  pub fn get_with_id(conn: &mut PgConnection, comment_id: i32) -> Option<Comment> {
    use crate::schema::comments::dsl;

    let comment = dsl::comments
      .filter(dsl::id.eq(comment_id))
      .select(Comment::as_select())
      .first(conn)
      .optional();
    match comment {
      Ok(comment) => comment,
      Err(_) => None,
    }
  }

  pub fn delete(conn: &mut PgConnection, comment_id: i32) -> bool {
    use crate::schema::comments::dsl::comments;

    diesel::delete(comments.find(comment_id))
      .execute(conn)
      .is_ok()
  }
}
