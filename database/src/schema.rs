// @generated automatically by Diesel CLI.

pub mod sql_types {
  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "commentauthor"))]
  pub struct Commentauthor;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "repotype"))]
  pub struct Repotype;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "status"))]
  pub struct Status;
}

diesel::table! {
    assignments (id) {
        id -> Int4,
        group_id -> Int4,
        base_repo_id -> Int4,
        test_repo_id -> Nullable<Int4>,
        correction_repo_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Status;

    cirun (id) {
        id -> Int4,
        repository_id -> Int4,
        commit -> Text,
        status -> Status,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Commentauthor;

    comments (id) {
        id -> Int4,
        repository_id -> Int4,
        #[max_length = 255]
        commit_hash -> Varchar,
        respond_to -> Nullable<Int4>,
        #[max_length = 255]
        file_path -> Nullable<Varchar>,
        message -> Text,
        author_type -> Commentauthor,
        author_id -> Nullable<Int4>,
        date -> Timestamp,
    }
}

diesel::table! {
    group_students (group_id, student_id) {
        group_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Text,
        teacher_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Repotype;

    repositories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        repo_type -> Repotype,
        owner_id -> Int4,
        assignment_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        email -> Text,
        pubkey -> Array<Nullable<Text>>,
    }
}

diesel::joinable!(assignments -> groups (group_id));
diesel::joinable!(cirun -> repositories (repository_id));
diesel::joinable!(comments -> repositories (repository_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(group_students -> groups (group_id));
diesel::joinable!(group_students -> users (student_id));
diesel::joinable!(groups -> users (teacher_id));
diesel::joinable!(repositories -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
  assignments,
  cirun,
  comments,
  group_students,
  groups,
  repositories,
  users,
);
