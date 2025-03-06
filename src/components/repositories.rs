use git2::RepositoryOpenFlags;

use crate::git::Repository;

pub struct RepositoriesComponent {
    repositories: Vec<Repository>,
}

impl RepositoriesComponent {
    pub fn new(repositories: Vec<Repository>) -> Self {
        Self { repositories }
    }
}
