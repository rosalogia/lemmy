-- Your SQL goes here
create table draft (
    id serial primary key,
    name varchar(200) not null,
    url varchar(512),
    body text,
    creator_id integer not null,
    community_id integer not null,
    nsfw boolean default false not null,
    embed_title text,
    embed_description text,
    thumbnail_url text,
    embed_video_url text,
    language_id integer default 0 not null
);