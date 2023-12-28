use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

impl User {
    pub fn create(conn: &mut PgConnection, username: &str, email: &str, pubkey: &Vec<Option<String>>) -> User {
        use crate::schema::users;

        let new_user = NewUser { username, email, pubkey };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
    }

    pub fn get_with_username(conn: &mut PgConnection, username: &str) -> Option<User> {
        use crate::schema::users::dsl;

        let user  = dsl::users
            .filter(dsl::username.eq(username))
            .select(User::as_select())
            .first(conn)
            .optional();
        match user {
            Ok(user) => user,
            Err(_) => None,
        }
    }

    pub fn delete(conn: &mut PgConnection, user_id: i32) -> bool {
        use crate::schema::users::dsl::users;

        diesel::delete(users.find(user_id))
            .execute(conn)
            .is_ok()
    }
}

impl Repository{
    pub fn create(conn: &mut PgConnection, name: &str, repo_type: &Repotype, owner_id: i32) -> Repository {
        use crate::schema::repositories;

        let new_repository = NewRepository { name, repo_type, owner_id };

        diesel::insert_into(repositories::table)
            .values(&new_repository)
            .returning(Repository::as_returning())
            .get_result(conn)
            .expect("Error saving new repository")
    }

    pub fn get_with_name(conn: &mut PgConnection, name: &str) -> Option<Repository> {
        use crate::schema::repositories::dsl;

        let repository  = dsl::repositories
            .filter(dsl::name.eq(name))
            .select(Repository::as_select())
            .first(conn)
            .optional();
        match repository {
            Ok(repository) => repository,
            Err(_) => None,
        }
    }

    pub fn delete(conn: &mut PgConnection, repository_id: i32) -> bool {
        use crate::schema::repositories::dsl::repositories;

        diesel::delete(repositories.find(repository_id))
            .execute(conn)
            .is_ok()
    }
}

impl Group{
    pub fn create(conn: &mut PgConnection, teacher_id: i32) -> Group {
        use crate::schema::groups;

        let new_group = NewGroup { teacher_id };

        diesel::insert_into(groups::table)
            .values(&new_group)
            .returning(Group::as_returning())
            .get_result(conn)
            .expect("Error saving new group")
    }

    pub fn get_with_id(conn: &mut PgConnection, group_id: i32) -> Option<Group> {
        use crate::schema::groups::dsl;

        let group  = dsl::groups
            .filter(dsl::id.eq(group_id))
            .select(Group::as_select())
            .first(conn)
            .optional();
        match group {
            Ok(group) => group,
            Err(_) => None,
        }
    }

    pub fn delete(conn: &mut PgConnection, group_id: i32) -> bool {
        use crate::schema::groups::dsl::groups;

        diesel::delete(groups.find(group_id))
            .execute(conn)
            .is_ok()
    }
}

impl Assignment{
    pub fn create(conn: &mut PgConnection, group_id: i32, base_repo_id: i32, test_repo_id: i32, correction_repo_id: i32) -> Assignment {
        use crate::schema::assignments;

        let new_assignment = NewAssignment { group_id, base_repo_id, test_repo_id, correction_repo_id };

        diesel::insert_into(assignments::table)
            .values(&new_assignment)
            .returning(Assignment::as_returning())
            .get_result(conn)
            .expect("Error saving new assignment")
    }

    pub fn get_with_id(conn: &mut PgConnection, assignment_id: i32) -> Option<Assignment> {
        use crate::schema::assignments::dsl;

        let assignment  = dsl::assignments
            .filter(dsl::id.eq(assignment_id))
            .select(Assignment::as_select())
            .first(conn)
            .optional();
        match assignment {
            Ok(assignment) => assignment,
            Err(_) => None,
        }
    }

    pub fn delete(conn: &mut PgConnection, assignment_id: i32) -> bool {
        use crate::schema::assignments::dsl::assignments;

        diesel::delete(assignments.find(assignment_id))
            .execute(conn)
            .is_ok()
    }
}

impl GroupStudent{
    pub fn create(conn: &mut PgConnection, group_id: i32, student_id: i32) -> GroupStudent {
        use crate::schema::group_students;

        let new_group_student = NewGroupStudent { group_id, student_id };

        diesel::insert_into(group_students::table)
            .values(&new_group_student)
            .returning(GroupStudent::as_returning())
            .get_result(conn)
            .expect("Error saving new group_student")
    }

    pub fn get_with_ids(conn: &mut PgConnection, group_id: i32, student_id: i32) -> Option<GroupStudent> {
        use crate::schema::group_students::dsl;

        let group_student  = dsl::group_students
            .filter(dsl::group_id.eq(group_id))
            .filter(dsl::student_id.eq(student_id))
            .select(GroupStudent::as_select())
            .first(conn)
            .optional();
        match group_student {
            Ok(group_student) => group_student,
            Err(_) => None,
        }
    }

    pub fn delete(conn: &mut PgConnection, group_id: i32, student_id: i32) -> bool {
        use crate::schema::group_students::dsl::group_students;

        diesel::delete(group_students.find((group_id, student_id)))
            .execute(conn)
            .is_ok()
    }
}

impl Cirun{
    pub fn create(conn: &mut PgConnection, repository_id: i32, commit: &str, status: &Status) -> Cirun {
        use crate::schema::cirun;

        let new_cirun = NewCirun { repository_id, commit, status };

        diesel::insert_into(cirun::table)
            .values(&new_cirun)
            .returning(Cirun::as_returning())
            .get_result(conn)
            .expect("Error saving new cirun")
    }

    pub fn get_with_id(conn: &mut PgConnection, cirun_id: i32) -> Option<Cirun> {
        use crate::schema::cirun::dsl;

        let cirun  = dsl::cirun
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

        diesel::delete(cirun.find(cirun_id))
            .execute(conn)
            .is_ok()
    }
}

impl Comment {
    pub fn create(conn: &mut PgConnection, repository_id: i32, commit_hash: &str, comment_type: &Commenttype, message: &str, author_type: &Commentauthor, author_id: i32, date: &std::time::SystemTime) -> Comment {
        use crate::schema::comments;

        let new_comment = NewComment { repository_id, commit_hash, comment_type, message, author_type, author_id, date };

        diesel::insert_into(comments::table)
            .values(&new_comment)
            .returning(Comment::as_returning())
            .get_result(conn)
            .expect("Error saving new comment")
    }

    pub fn get_with_id(conn: &mut PgConnection, comment_id: i32) -> Option<Comment> {
        use crate::schema::comments::dsl;

        let comment  = dsl::comments
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
