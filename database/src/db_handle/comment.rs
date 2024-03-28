use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, OptionalExtension, QueryDsl,
  RunQueryDsl, Selectable, SelectableHelper,
};
use diesel_derive_enum::DbEnum;

use crate::{error::DatabaseError, DbHandle};

#[derive(Debug, DbEnum, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::Commentauthor"]
pub enum Commentauthor {
  User,
  Automated,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
  pub id: i32,
  pub repository_id: i32,
  pub commit_hash: String,

  pub respond_to: Option<i32>,
  pub file_path: Option<String>,

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

  pub respond_to: Option<i32>,
  pub file_path: Option<&'a str>,

  pub message: &'a str,
  pub author_type: &'a Commentauthor,
  pub author_id: Option<i32>,
  pub date: &'a std::time::SystemTime,
}

pub trait CommentDbHandle {
  fn add_comment(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
    author_id: i32,
    message: &str,
  ) -> Result<Comment, DatabaseError>;

  fn add_file_comment(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
    file_path: &str,
    author_id: i32,
    message: &str,
  ) -> Result<Comment, DatabaseError>;

  fn add_ci_comment(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
    message: &str,
  ) -> Result<Comment, DatabaseError>;

  fn add_ci_file_comment(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
    file_path: &str,
    message: &str,
  ) -> Result<Comment, DatabaseError>;

  fn add_response_comment(
    &mut self,
    reply_to: i32,
    author_id: i32,
    message: &str,
  ) -> Result<Comment, DatabaseError>;

  fn get_comment_by_id(&mut self, comment_id: i32) -> Result<Option<Comment>, DatabaseError>;

  fn list_commit_comments(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
  ) -> Result<Vec<Comment>, DatabaseError>;

  fn list_response_comments(&mut self, comment_id: i32) -> Result<Vec<Comment>, DatabaseError>;

  fn delete_comment(&mut self, comment_id: i32) -> Result<(), DatabaseError>;
}

#[cfg_attr(feature = "mock", faux::methods(path = "super"))]
impl DbHandle {
  fn add_comment_inner(&mut self, new_comment: NewComment) -> Result<Comment, DatabaseError> {
    use crate::schema::comments;

    diesel::insert_into(comments::table)
      .values(&new_comment)
      .get_result(&mut self.conn)
      .map_err(DatabaseError::from)
  }
}

#[cfg_attr(feature = "mock", faux::methods(path = "super"))]
impl CommentDbHandle for DbHandle {
  fn add_comment(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
    author_id: i32,
    message: &str,
  ) -> Result<Comment, DatabaseError> {
    let new_comment = NewComment {
      repository_id,
      commit_hash: commit_hash,
      respond_to: None,
      file_path: None,
      message,
      author_type: &Commentauthor::User,
      author_id: Some(author_id),
      date: &std::time::SystemTime::now(),
    };

    self.add_comment_inner(new_comment)
  }

  fn add_file_comment(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
    file_path: &str,
    author_id: i32,
    message: &str,
  ) -> Result<Comment, DatabaseError> {
    let new_comment = NewComment {
      repository_id,
      commit_hash: commit_hash,
      respond_to: None,
      file_path: Some(file_path),
      message,
      author_type: &Commentauthor::User,
      author_id: Some(author_id),
      date: &std::time::SystemTime::now(),
    };

    self.add_comment_inner(new_comment)
  }

  fn add_ci_comment(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
    message: &str,
  ) -> Result<Comment, DatabaseError> {
    let new_comment = NewComment {
      repository_id,
      commit_hash: commit_hash,
      respond_to: None,
      file_path: None,
      message,
      author_type: &Commentauthor::Automated,
      author_id: None,
      date: &std::time::SystemTime::now(),
    };

    self.add_comment_inner(new_comment)
  }

  fn add_ci_file_comment(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
    file_path: &str,
    message: &str,
  ) -> Result<Comment, DatabaseError> {
    let new_comment = NewComment {
      repository_id,
      commit_hash: commit_hash,
      respond_to: None,
      file_path: Some(file_path),
      message,
      author_type: &Commentauthor::Automated,
      author_id: None,
      date: &std::time::SystemTime::now(),
    };

    self.add_comment_inner(new_comment)
  }

  fn add_response_comment(
    &mut self,
    reply_to: i32,
    author_id: i32,
    message: &str,
  ) -> Result<Comment, DatabaseError> {
    let comment = self
      .get_comment_by_id(reply_to)?
      .ok_or(DatabaseError::NotFound)?;
    let new_comment = NewComment {
      repository_id: comment.repository_id,
      commit_hash: &comment.commit_hash,
      respond_to: Some(reply_to),
      file_path: comment.file_path.as_deref(),
      message,
      author_type: &Commentauthor::User,
      author_id: Some(author_id),
      date: &std::time::SystemTime::now(),
    };

    self.add_comment_inner(new_comment)
  }

  fn get_comment_by_id(&mut self, comment_id: i32) -> Result<Option<Comment>, DatabaseError> {
    use crate::schema::comments::dsl;

    dsl::comments
      .filter(dsl::id.eq(comment_id))
      .select(Comment::as_select())
      .first(&mut self.conn)
      .optional()
      .map_err(DatabaseError::from)
  }

  fn list_commit_comments(
    &mut self,
    repository_id: i32,
    commit_hash: &str,
  ) -> Result<Vec<Comment>, DatabaseError> {
    use crate::schema::comments::dsl;

    dsl::comments
      .filter(dsl::repository_id.eq(repository_id))
      .filter(dsl::commit_hash.eq(commit_hash))
      .select(Comment::as_select())
      .load(&mut self.conn)
      .map_err(DatabaseError::from)
  }

  fn list_response_comments(&mut self, comment_id: i32) -> Result<Vec<Comment>, DatabaseError> {
    use crate::schema::comments::dsl;

    dsl::comments
      .filter(dsl::respond_to.eq(comment_id))
      .select(Comment::as_select())
      .load(&mut self.conn)
      .map_err(DatabaseError::from)
  }

  fn delete_comment(&mut self, comment_id: i32) -> Result<(), DatabaseError> {
    use crate::schema::comments::dsl::*;

    diesel::delete(comments.find(comment_id))
      .execute(&mut self.conn)
      .map(|_| ())
      .map_err(DatabaseError::from)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    db_handle::{
      repository::{RepositoryDbHandle, Repotype},
      user::UserDbHandle,
    },
    transaction_tests,
  };

  use super::{CommentDbHandle, Commentauthor};

  transaction_tests! {
    fn add_comment_missing_repository(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let result = tx.add_comment(0, "commit", user.id, "message");
      result.expect_err("Expected error when adding comment to nonexistent repository");
    }

    fn add_comment_missing_author(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let result = tx.add_comment(repository.id, "commit", 0, "message");
      result.expect_err("Expected error when adding comment with nonexistent author");
    }

    fn add_comment_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      assert_eq!(comment.repository_id, repository.id);
      assert_eq!(comment.commit_hash, "commit");
      assert_eq!(comment.author_id, Some(user.id));
      assert_eq!(comment.message, "message");
      assert_eq!(comment.author_type, Commentauthor::User);
    }

    fn add_file_comment_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_file_comment(repository.id, "commit", "file", user.id, "message")?;
      assert_eq!(comment.repository_id, repository.id);
      assert_eq!(comment.commit_hash, "commit");
      assert_eq!(comment.author_id, Some(user.id));
      assert_eq!(comment.message, "message");
      assert_eq!(comment.author_type, Commentauthor::User);
      assert_eq!(comment.file_path, Some("file".to_string()));
    }

    fn add_ci_comment_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_ci_comment(repository.id, "commit", "message")?;
      assert_eq!(comment.repository_id, repository.id);
      assert_eq!(comment.commit_hash, "commit");
      assert_eq!(comment.author_id, None);
      assert_eq!(comment.message, "message");
      assert_eq!(comment.author_type, Commentauthor::Automated);
    }

    fn add_ci_file_comment_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_ci_file_comment(repository.id, "commit", "file#42-43", "message")?;
      assert_eq!(comment.repository_id, repository.id);
      assert_eq!(comment.commit_hash, "commit");
      assert_eq!(comment.author_id, None);
      assert_eq!(comment.message, "message");
      assert_eq!(comment.author_type, Commentauthor::Automated);
      assert_eq!(comment.file_path, Some("file#42-43".to_string()));
    }

    fn add_response_comment_missing_reply_to(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let result = tx.add_response_comment(0, user.id, "message");
      result.expect_err("Expected error when adding response comment to nonexistent comment");
    }

    fn add_response_comment_missing_author(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      let result = tx.add_response_comment(comment.id, 0, "message");
      result.expect_err("Expected error when adding response comment with nonexistent author");
    }

    fn add_response_comment_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      let response = tx.add_response_comment(comment.id, user.id, "message")?;
      assert_eq!(response.respond_to, Some(comment.id));
      assert_eq!(response.author_id, Some(user.id));
      assert_eq!(response.message, "message");
      assert_eq!(response.author_type, Commentauthor::User);
    }

    fn listed_comments_include_responses(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      let response = tx.add_response_comment(comment.id, user.id, "message")?;
      tx.add_response_comment(comment.id, user.id, "message")?;
      tx.add_response_comment(response.id, user.id, "message")?;
      let comments = tx.list_commit_comments(repository.id, "commit")?;
      assert_eq!(comments.len(), 4);
    }

    fn get_comment_by_id_missing_comment(tx: &mut DbHandle) {
      let comment = tx.get_comment_by_id(0)?;
      assert!(comment.is_none());
    }

    fn get_comment_by_id_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      let found_comment = tx.get_comment_by_id(comment.id)?.expect("Comment not found");
      assert_eq!(found_comment.id, comment.id);
      assert_eq!(found_comment.repository_id, repository.id);
      assert_eq!(found_comment.commit_hash, "commit");
      assert_eq!(found_comment.author_id, Some(user.id));
      assert_eq!(found_comment.message, "message");
      assert_eq!(found_comment.author_type, Commentauthor::User);
    }

    fn list_commit_comments_empty(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comments = tx.list_commit_comments(repository.id, "commit")?;
      assert!(comments.is_empty());
    }

    fn list_commit_comments_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      let comments = tx.list_commit_comments(repository.id, "commit")?;
      assert_eq!(comments.len(), 1);
      assert_eq!(comments[0].id, comment.id);
      assert_eq!(comments[0].repository_id, repository.id);
      assert_eq!(comments[0].commit_hash, "commit");
      assert_eq!(comments[0].author_id, Some(user.id));
      assert_eq!(comments[0].message, "message");
      assert_eq!(comments[0].author_type, Commentauthor::User);
    }

    fn list_commit_comments_multiple_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      tx.add_response_comment(comment.id, user.id, "message")?;
      tx.add_ci_file_comment(repository.id, "commit", "file", "message")?;
      tx.add_ci_comment(repository.id, "commit", "message")?;

      let comments = tx.list_commit_comments(repository.id, "commit")?;
      assert_eq!(comments.len(), 4);
    }

    fn list_response_nonexistent_comments_empty(tx: &mut DbHandle) {
      let comments = tx.list_response_comments(0)?;
      assert!(comments.is_empty());
    }

    fn list_response_comments_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      let response = tx.add_response_comment(comment.id, user.id, "message")?;
      let comments = tx.list_response_comments(comment.id)?;
      assert_eq!(comments.len(), 1);
      assert_eq!(comments[0].id, response.id);
      assert_eq!(comments[0].repository_id, repository.id);
      assert_eq!(comments[0].commit_hash, "commit");
      assert_eq!(comments[0].author_id, Some(user.id));
      assert_eq!(comments[0].message, "message");
      assert_eq!(comments[0].author_type, Commentauthor::User);
    }

    fn delete_comment_missing_comment(tx: &mut DbHandle) {
      let result = tx.delete_comment(0);
      result.expect("Expected nothing to happen when deleting nonexistent comment");
    }

    fn delete_comment_success(tx: &mut DbHandle) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repository = tx.create_repository("repo", &Repotype::Default, user.id, None)?;
      let comment = tx.add_comment(repository.id, "commit", user.id, "message")?;
      let result = tx.delete_comment(comment.id);
      result.expect("Expected comment to be deleted");
      let found_comment = tx.get_comment_by_id(comment.id)?;
      assert!(found_comment.is_none(), "Expected comment to be deleted");
    }
  }
}
