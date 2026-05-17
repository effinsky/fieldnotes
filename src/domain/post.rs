use std::time::SystemTime;

use super::slug::{Slug, SlugError};

#[derive(Debug, PartialEq)]
pub struct Post {
    pub title: String,
    pub slug: Slug,
    pub body_md: String,
    pub status: PostStatus,
    pub published_at: Option<SystemTime>,
}

#[derive(Debug, PartialEq)]
pub enum PostStatus {
    Draft,
    Published,
}

#[derive(Debug, PartialEq)]
pub enum PostError {
    EmptyTitle,
    EmptyBody,        // only when publishing; draft empty bod is fine
    AlreadyPublished, // check when publishing
    InvalidSlug(SlugError),
}

impl Post {
    pub fn new_draft(title: &str, slug: &str, body: &str) -> Result<Self, PostError> {
        if title.is_empty() {
            return Err(PostError::EmptyTitle);
        }
        let slug = Slug::new(slug).map_err(PostError::InvalidSlug)?;

        Ok(Self {
            title: title.to_string(),
            body_md: body.to_string(),
            slug,
            status: PostStatus::Draft,
            published_at: None,
        })
    }

    pub fn update_title(mut self, new_title: &str) -> Result<(), PostError> {
        if self.status == PostStatus::Draft {
            self.title = new_title.to_string();
            Ok(())
        } else {
            // reconsider enabling edits on published material later
            Err(PostError::AlreadyPublished)
        }
    }

    pub fn update_body(mut self, new_body: &str) -> Result<(), PostError> {
        if self.status == PostStatus::Draft {
            self.body_md = new_body.to_string();
            Ok(())
            // reconsider enabling edits on published material later
        } else {
            // reconsider enabling edits on published material later
            Err(PostError::AlreadyPublished)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_post() {
        let title = "title";
        let body = "";
        let slug = "good-slug";

        let draft_result = Post::new_draft(title, slug, body);
        assert_eq!(
            draft_result,
            Ok(Post {
                title: title.to_string(),
                slug: Slug::new(slug).expect("test slug creation to succeed"),
                body_md: body.to_string(),
                status: PostStatus::Draft,
                published_at: None,
            })
        )
    }

    #[test]
    fn rejects_empty_title() {
        let draft_result = Post::new_draft("", "slug", "body");
        assert_eq!(draft_result, Err(PostError::EmptyTitle))
    }

    #[test]
    fn rejects_invalid_slug_with_specific_error() {
        let draft_result = Post::new_draft("title", "☺-smiley-slug", "body");
        assert_eq!(
            draft_result,
            Err(PostError::InvalidSlug(SlugError::InvalidCharacter))
        );
    }
}
