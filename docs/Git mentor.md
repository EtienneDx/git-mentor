# Git mentor

Git mentor aims to provide an efficient way to manage repositories for a classroom. Its goal is to provide an enforced CI pipeline for teachers, thus ensuring students respect best practices. It will also allow some automated correction and testing to provide automated feedback for the students.

## Features

### SSH git access

Students and teachers will be able to access their repositories via git over an ssh protocol.

### Web access

Students and teachers will have access to a simplified online platform to visualize the code and manage permissions, grades and assignments

#### Diff viewer

Teachers and students should be able to see what code has been written by students and provide feedback on specific lines of code, much like CR feedback. Considering the scope, a simple viewer without marking the diffs may be enough as students are expected to, mostly, write all of the code.

### Third party APIs

In the long run, the project should be able to integrate with intranets to automatically provide grades

### Automated CI

To provide some automation that won't be altered by students, teachers will have two repositories:
- A `main` repository holding the template code for the students
- A `ci` repository holding the pipeline definition

Additionally, teachers may want a `correction` repository for their personnal use and/or to release it to students after the assignment is over.

## Infrastructure

See [Infrastucture](./Infrastructure.md) for infos on how the project will look.