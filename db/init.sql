create table if not exists users (
    id          uuid primary key default gen_random_uuid(),
    username    varchar(20) unique not null,
    email       varchar(254) unique not null,
    password    text not null,
    score       integer default 0,
    created_at  timestamptz not null default now()
);

create table if not exists follows (
    followee_user_id    uuid not null references users(id),
    follower_user_id    uuid not null references users(id),
    created_at  timestamptz not null default now()
);