use std::time::SystemTime;

use super::slug::{Slug, SlugError};

#[derive(Debug, PartialEq)]
pub struct Post {
    title: String,
    slug: Slug,
    body_md: String,
    status: PostStatus,
    published_at: Option<SystemTime>,
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
        if title.trim().is_empty() {
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

    pub fn publish(&mut self, now: SystemTime) -> Result<(), PostError> {
        match self.status {
            PostStatus::Published => Err(PostError::AlreadyPublished),
            PostStatus::Draft if self.title.trim().is_empty() => {
                Err(PostError::EmptyTitle)
            }
            PostStatus::Draft if self.body_md.trim().is_empty() => {
                Err(PostError::EmptyBody)
            }
            PostStatus::Draft => {
                self.status = PostStatus::Published;
                self.published_at = Some(now);
                Ok(())
            }
        }
    }

    pub fn update_title(&mut self, new_title: &str) -> Result<(), PostError> {
        if new_title.trim().is_empty() {
            return Err(PostError::EmptyTitle);
        }
        if self.status == PostStatus::Published {
            return Err(PostError::AlreadyPublished);
        }
        self.title = new_title.to_string();
        Ok(())
    }

    pub fn update_body(&mut self, new_body: &str) -> Result<(), PostError> {
        if self.status == PostStatus::Published {
            return Err(PostError::AlreadyPublished);
        }
        self.body_md = new_body.to_string();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_transition_from_draft_to_published() {
        let now = SystemTime::now();
        let mut post = Post::new_draft("title", "slug", "body")
            .expect("draft creation should succeed");

        post.update_title("gonna publish this tit")
            .expect("title should update");
        post.update_body("gonna publish with this bod")
            .expect("body should update");

        post.publish(now).expect("publish should succeed");

        assert_eq!(
            post,
            Post {
                title: "gonna publish this tit".to_string(),
                slug: Slug::new("slug").expect("slug should be valid"),
                body_md: "gonna publish with this bod".to_string(),
                status: PostStatus::Published,
                published_at: Some(now),
            }
        );
    }

    #[test]
    fn cannot_be_edited_once_published() {
        let now = SystemTime::now();
        let mut post = Post::new_draft("title", "slug", "body")
            .expect("draft creation should succeed");

        post.publish(now).expect("publish should succeed");

        let title_update_result = post.update_title("new title");
        assert_eq!(title_update_result, Err(PostError::AlreadyPublished));

        let body_update_result = post.update_body("new body");
        assert_eq!(body_update_result, Err(PostError::AlreadyPublished));
    }

    #[test]
    fn cannot_publish_invalid_post() {
        let now = SystemTime::now();
        let mut post = Post::new_draft("title", "slug", "body")
            .expect("draft creation should succeed");

        // clear title to make post unpublishable
        let _ = post.update_body("");

        let publish_result = post.publish(now);
        assert_eq!(publish_result, Err(PostError::EmptyBody));
    }
}
