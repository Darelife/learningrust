use crate::database::project::Project;
#[derive(Debug)]

pub struct Person {
    pub name: String,
    pub project: Option<Project>,
}
