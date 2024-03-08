use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, OptionalExtension, QueryDsl,
  RunQueryDsl, Selectable, SelectableHelper,
};

use crate::{
  error::DatabaseError,
  schema::{assignments, group_students, groups},
  TransactionHandler,
};

use super::{assignment::Assignment, user::User};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
  pub id: i32,
  pub teacher_id: Option<i32>,
  pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGroup {
  pub teacher_id: Option<i32>,
  pub name: String,
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

pub trait GroupTransactionHandler {
  fn create_group(&mut self, name: &str, teacher_id: Option<i32>) -> Result<Group, DatabaseError>;

  fn get_group_by_id(&mut self, group_id: i32) -> Result<Option<Group>, DatabaseError>;

  fn get_teacher(&mut self, group_id: i32) -> Result<Option<User>, DatabaseError>;

  fn set_teacher(&mut self, group_id: i32, teacher_id: Option<i32>)
    -> Result<Group, DatabaseError>;
    
  fn list_group_assignments(&mut self, group_id: i32) -> Result<Vec<Assignment>, DatabaseError>;

  fn add_student(&mut self, group_id: i32, student_id: i32) -> Result<(), DatabaseError>;

  fn list_students(&mut self, group_id: i32) -> Result<Vec<User>, DatabaseError>;

  fn delete_group(&mut self, group_id: i32) -> bool;
}

impl<'a> GroupTransactionHandler for TransactionHandler<'a> {
  fn create_group(&mut self, name: &str, teacher_id: Option<i32>) -> Result<Group, DatabaseError> {
    let new_group = NewGroup {
      teacher_id,
      name: name.to_string(),
    };

    diesel::insert_into(groups::table)
      .values(&new_group)
      .returning(Group::as_returning())
      .get_result(self.conn)
      .map_err(DatabaseError::from)
  }

  fn get_group_by_id(&mut self, group_id: i32) -> Result<Option<Group>, DatabaseError> {
    use crate::schema::groups::dsl;

    groups::table
      .filter(dsl::id.eq(group_id))
      .select(Group::as_select())
      .first(self.conn)
      .optional()
      .map_err(DatabaseError::from)
  }

  fn get_teacher(&mut self, group_id: i32) -> Result<Option<User>, DatabaseError> {
    use crate::schema::groups::dsl;

    let res = groups::table
      .filter(dsl::id.eq(group_id))
      .inner_join(crate::schema::users::table)
      .select(crate::schema::users::all_columns)
      .first(self.conn);

    match res {
      Ok(user) => Ok(Some(user)),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(DatabaseError::from(e)),
    }
  }

  fn set_teacher(
    &mut self,
    group_id: i32,
    teacher_id: Option<i32>,
  ) -> Result<Group, DatabaseError> {
    use crate::schema::groups::dsl;

    diesel::update(groups::table.filter(dsl::id.eq(group_id)))
      .set(dsl::teacher_id.eq(teacher_id))
      .returning(Group::as_returning())
      .get_result(self.conn)
      .map_err(DatabaseError::from)
  }
    
  fn list_group_assignments(&mut self, group_id: i32) -> Result<Vec<Assignment>, DatabaseError> {
    use crate::schema::assignments::dsl;

    assignments::table
      .filter(dsl::group_id.eq(group_id))
      .select(Assignment::as_select())
      .load(self.conn)
      .map_err(DatabaseError::from)
  }

  fn add_student(&mut self, group_id: i32, student_id: i32) -> Result<(), DatabaseError> {
    let new_group_student = NewGroupStudent {
      group_id,
      student_id,
    };

    diesel::insert_into(group_students::table)
      .values(&new_group_student)
      .execute(self.conn)
      .map_err(DatabaseError::from)?;

    Ok(())
  }

  fn list_students(&mut self, group_id: i32) -> Result<Vec<User>, DatabaseError> {
    use crate::schema::group_students::dsl;

    group_students::table
      .filter(dsl::group_id.eq(group_id))
      .inner_join(crate::schema::users::table)
      .select(crate::schema::users::all_columns)
      .load(self.conn)
      .map_err(DatabaseError::from)
  }

  fn delete_group(&mut self, group_id: i32) -> bool {
    use crate::schema::groups::dsl;

    diesel::delete(groups::table.filter(dsl::id.eq(group_id)))
      .execute(self.conn)
      .is_ok()
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    db_handle::{assignment::AssignmentTransactionHandler, group::GroupTransactionHandler, repository::{RepositoryTransactionHandler, Repotype}, user::UserTransactionHandler},
    transaction_tests,
  };

  transaction_tests! {
    fn create_group(tx: &mut TransactionHandler) {
      let group = tx.create_group("test_group", None)?;
      assert_eq!(group.name, "test_group");
      assert_eq!(group.teacher_id, None);
    }

    fn create_group_with_teacher(tx: &mut TransactionHandler) {
      let teacher = tx.create_user("teacher", "email", "password", None)?;
      let group = tx.create_group("test_group", Some(teacher.id))?;
      assert_eq!(group.name, "test_group");
      assert_eq!(group.teacher_id, Some(teacher.id));
    }

    fn create_group_nonexistent_teacher_fails(tx: &mut TransactionHandler) {
      let group = tx.create_group("test_group", Some(1));
      assert!(group.is_err());
    }

    fn get_group_by_id(tx: &mut TransactionHandler) {
      let group = tx.create_group("test_group", None)?;
      let group = tx.get_group_by_id(group.id)?;
      let group = group.expect("Group not found");
      assert_eq!(group.name, "test_group");
      assert_eq!(group.teacher_id, None);
    }

    fn get_group_by_id_nonexistent_group(tx: &mut TransactionHandler) {
      let group = tx.get_group_by_id(1)?;
      assert!(group.is_none());
    }

    fn get_teacher(tx: &mut TransactionHandler) {
      let teacher = tx.create_user("teacher", "email", "password", None)?;
      let group = tx.create_group("test_group", Some(teacher.id))?;
      let teacher = tx.get_teacher(group.id)?;
      let teacher = teacher.expect("Teacher not found");
      assert_eq!(teacher.id, teacher.id);
    }

    fn get_teacher_nonexistent_group(tx: &mut TransactionHandler) {
      let teacher = tx.get_teacher(1)?;
      assert!(teacher.is_none());
    }

    fn set_teacher(tx: &mut TransactionHandler) {
      let teacher = tx.create_user("teacher", "email", "password", None)?;
      let group = tx.create_group("test_group", None)?;
      let group = tx.set_teacher(group.id, Some(teacher.id))?;
      assert_eq!(group.teacher_id, Some(teacher.id));
    }

    fn set_teacher_nonexistent_group_fails(tx: &mut TransactionHandler) {
      let teacher = tx.create_user("teacher", "email", "password", None)?;
      let group = tx.set_teacher(1, Some(teacher.id));
      assert!(group.is_err());
    }

    fn unset_teacher(tx: &mut TransactionHandler) {
      let teacher = tx.create_user("teacher", "email", "password", None)?;
      let group = tx.create_group("test_group", Some(teacher.id))?;
      let group = tx.set_teacher(group.id, None)?;
      assert_eq!(group.teacher_id, None);
    }

    fn list_group_assignments(tx: &mut TransactionHandler) {
      let user = tx.create_user("user", "email", "password", None)?;
      let group = tx.create_group("test_group", None)?;
      let assignments = vec![
        "repo1",
        "repo2",
        "repo3",
      ];
      assignments.iter().for_each(|repo_name| {
        let repo = tx.create_repository(repo_name, &Repotype::Default, user.id, None).expect("Error creating repository");
        tx.create_assignment(
          group.id,
          repo.id,
        ).expect("Error creating assignment");
      });
      let assignments = tx.list_group_assignments(group.id)?;
      assert_eq!(assignments.len(), 3);
    }

    fn add_student(tx: &mut TransactionHandler) {
      let student = tx.create_user("student", "email", "password", None)?;
      let group = tx.create_group("test_group", None)?;
      tx.add_student(group.id, student.id)?;
      let students = tx.list_students(group.id)?;
      assert_eq!(students.len(), 1);
      assert_eq!(students[0].id, student.id);
    }

    fn add_student_nonexistent_group_fails(tx: &mut TransactionHandler) {
      let student = tx.create_user("student", "email", "password", None)?;
      let group = tx.add_student(1, student.id);
      assert!(group.is_err());
    }

    fn list_students(tx: &mut TransactionHandler) {
      let students = vec![
        ("student-1", "email-1", "password-1"),
        ("student-2", "email-2", "password-2"),
        ("student-3", "email-3", "password-3"),
      ];
      let group = tx.create_group("test_group", None)?;
      students.iter().for_each(|(username, email, password)| {
        let student = tx.create_user(username, email, password, None).expect("Error creating user");
        tx.add_student(group.id, student.id).expect("Error adding student");
      });
      let students = tx.list_students(group.id)?;
      assert_eq!(students.len(), 3);
    }

    fn list_students_nonexistent_group(tx: &mut TransactionHandler) {
      let students = tx.list_students(1)?;
      assert!(students.is_empty());
    }

    fn delete_group(tx: &mut TransactionHandler) {
      let group = tx.create_group("test_group", None)?;
      let success = tx.delete_group(group.id);
      assert!(success, "Expected group to be deleted");
    }
  }
}
