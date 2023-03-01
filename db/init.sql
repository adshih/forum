create table if not exists users (
    id          uuid primary key default gen_random_uuid(),
    username    varchar(50) unique not null,
    password    text not null,
    score       integer default 0,
    created_at  timestamptz default now()
);