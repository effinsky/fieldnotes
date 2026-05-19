use crate::domain::post::{Post, PostError};
use std::collections::HashMap;

enum CreateDraftError {
    InvalidPost(PostError),
    SlugAlreadyExists,
}

struct InMemoryPosts(HashMap<String, Post>);

impl InMemoryPosts {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self(map)
    }

    fn create_draft(
        &mut self,
        title: &str,
        slug: &str,
        body: &str,
    ) -> Result<String, CreateDraftError> {
        let draft = Post::new_draft(title, slug, body)
            .map_err(CreateDraftError::InvalidPost)?;

        if self.0.contains_key(slug) {
            return Err(CreateDraftError::SlugAlreadyExists);
        }
        self.0.insert(slug.clone(), draft) {
           Some(v) => Ok(v),
           _ => Err(CreateDraftError::SlugAlreadyExists),
        }
    }

    fn get_by_slug(&self, slug: &str) -> Option<&Post> {
        None
    }
}
