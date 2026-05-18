# fieldnotes

Fieldnotes is a personal publishing system built in Rust.

The project is intentionally small in shape but serious in intent: an SSR-first
monolith with explicit domain modeling, modest abstractions, and a preference
for libraries over framework-driven control flow. The target is a production-
usable writing tool, not a demo app.

## Direction

- SSR first, API second
- Markdown-backed posts
- Postgres persistence
- Self-built admin/CMS
- Local-first uploads, object storage later
- Light/dark theme support
- Tests around domain and application behavior

## Planned stack

- Rust
- axum
- sqlx + Postgres
- askama
- pulldown-cmark

## Current status

Current work is focused on the core domain: slug validation and post
draft/publish lifecycle rules.

## Roadmap

1. Domain model and invariants
2. Application layer with in-memory repository
3. HTTP read side
4. Postgres-backed persistence
5. Admin and authentication
6. Asset uploads
7. Public and admin JSON APIs
