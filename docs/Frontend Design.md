# Frontend for Git Mentor

## App UX
![Page Routing](./graphs/page_routing.png)

## Page Routing
| route                         | Name                 | Description                             | Sprint     |
|-------------------------------|----------------------|-----------------------------------------|------------|
| /                             | HomePage             | User landing page with recaps           | User       |
| /login                        | Authentication       | Sign up page                            | User       |
| /settings                     | UserSettings         | User settings interface                 | User       |
| /groups                       | GroupsList           | List of my groups                       | DataViz    |
| /groups/create                | GroupCreator         | Creator for a group instance            | DataCreate |
| /groups/student/[id]          | GroupStudents        | List of the students of a group         | DataViz    |
| /groups/assignements/[id]     | GroupAssignements    | List of the assignements of a group     | DataViz    |
| /students                     | AllStudents          | List of all students                    | DataViz    |
| /students/create              | StudentCreator       | Creator for a student instance          | DataCreate |
| /repositories                 | RepositoriesList     | List of my repositories                 | DataViz    |
| /repositories/[id]            | RepositoryCode       | Repository's code                       | RepoViz    |
| /repositories/[id]/commits    | RepositoryCommits    | Repository's commits                    | RepoViz    |
| /repositories/[id]/settings   | RepositorySettings   | Repository's settings                   | RepoPlus   |
| /repositories/[id]/discussion | RepositoryDiscussion | Repository's discussion (only for stud) | RepoPlus   |
| /repositories/[id]/diffs      | RepositoryDiffs      | Repository's diffs (only for stud)      | RepoPlus   |
| /repositories/create          | RepositoryCreator    | Creator for a repository instance       | DataCreate |

## Sprint priority
1. DataViz: display the tables and navigate inside the database
2. User: create a friendly welcome experience for users
3. DataCreate: enable the creation of data instances
4. RepoViz: enable the interactions with a repository
5. RepoPlus: unlock more advanced features for repositories
