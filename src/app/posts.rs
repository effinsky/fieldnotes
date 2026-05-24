use crate::domain::post::{Post, PostError};
use std::collections::{HashMap, hash_map::Entry};

#[derive(Debug, PartialEq)]
enum CreateDraftError {
    InvalidPost(PostError),
    SlugAlreadyExists,
}

#[derive(Debug, PartialEq)]
struct InMemoryPosts {
    posts: HashMap<String, Post>,
}

impl InMemoryPosts {
    pub fn new() -> Self {
        Self { posts: HashMap::new() }
    }

    // create_draft creates a new draft post and inserts it into the in-memory
    // store.
    pub fn create_draft(
        &mut self,
        title: &str,
        slug: &str,
        body: &str,
    ) -> Result<(), CreateDraftError> {
        let draft = Post::new_draft(title, slug, body)
            .map_err(CreateDraftError::InvalidPost)?;

        let key = slug.to_string();

        match self.posts.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(draft);
                Ok(())
            }
            Entry::Occupied(_) => Err(CreateDraftError::SlugAlreadyExists),
        }
    }

    // get_by_slug retrieves a post by its slug. It returns None if the post
    // does not.
    pub fn get_by_slug(&self, slug: &str) -> Option<&Post> {
        self.posts.get(slug)
    }
}

mod tests {
    use crate::app::posts::InMemoryPosts;

    #[test]
    fn can_create_and_retrieve_draft() {
        use super::*;

        let slug = "slug";
        let title = "title";

        let mut store = InMemoryPosts::new();
        let draft_created = store.create_draft(title, slug, "body");

        assert_eq!(draft_created, Ok(()));

        let Some(post) = store.get_by_slug(slug) else {
            panic!("draft should be retrievable by slug");
        };

        assert_eq!(post.title(), title);
        assert_eq!(post.slug().as_str(), slug);
    }

    #[test]
    fn cannot_duplicate_slug() {
        let slug = "slugerino";
        let mut store = InMemoryPosts::new();

        let d1_result = store.create_draft("title-1", slug, "body-1");

        assert!(d1_result.is_ok());

        let d2_result = store.create_draft("title-2", slug, "body-2");

        assert_eq!(
            d2_result,
            Err(crate::app::posts::CreateDraftError::SlugAlreadyExists)
        );

        assert_eq!(store.get_by_slug(slug).unwrap().title(), "title-1");
    }
}
