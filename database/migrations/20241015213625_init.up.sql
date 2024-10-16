-- Add up migration script here
CREATE TABLE IF NOT EXISTS snippets (
    id int generated always as identity primary key,
    title varchar not null,
    description varchar not null,
    snippet varchar not null,
    language varchar not null,
    created_at timestamptz not null default current_timestamp,
    last_updated_at timestamptz not null default current_timestamp
);
