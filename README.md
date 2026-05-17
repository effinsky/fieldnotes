# fieldnotes

A blog with a small CMS, built in Rust from scratch. SSR-first monolith with
Postgres persistence, markdown content, image uploads, and a self-built admin
area. Dark/light mode included.

The goal is to learn Rust, architecture, and testing by building something real
— not by wiring framework features together.

## Roadmap

- [x] **Phase 1 — Domain** (current)
  - `Slug` value object with validation
  - `Post` with draft/publish lifecycle
  - `PostStatus`, `PostError`
  - unit tests for domain rules

- [ ] **Phase 2 — In-memory app layer**
  - `PostRepository` trait
  - `InMemoryPostRepository`
  - use cases: create draft, publish, list, edit

- [ ] **Phase 3 — HTTP read side**
  - `axum` router
  - public homepage + post page
  - SSR HTML templates
  - handler tests

- [ ] **Phase 4 — Persistence**
  - Postgres schema
  - `sqlx` repository implementation
  - DB integration tests

- [ ] **Phase 5 — Admin + auth**
  - login/logout
  - protected admin area
  - create/edit/publish UI

- [ ] **Phase 6 — Uploads**
  - cover and inline image upload
  - local storage first
  - storage abstraction

- [ ] **Phase 7 — JSON API**
  - public and admin API endpoints
  - DTOs

- [ ] **Theme support**
  - light/dark mode
  - system preference default
  - manual toggle

## Stack

- Rust
- axum
- sqlx + Postgres
- askama
- pulldown-cmark
