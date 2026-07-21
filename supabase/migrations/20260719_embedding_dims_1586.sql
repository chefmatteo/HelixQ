-- Match Rust EMBED_DIMS = 1586
-- Run in Supabase SQL Editor.

drop function if exists public.match_videos(vector(1536), int, text);
drop function if exists public.match_videos(vector(2048), int, text);
drop function if exists public.match_videos(vector(1586), int, text);
drop index if exists videos_embedding_hnsw_idx;

alter table public.videos
  alter column embedding type vector(1586)
  using null;

create index if not exists videos_embedding_hnsw_idx
on public.videos
using hnsw (embedding vector_cosine_ops);

create or replace function public.match_videos(
  query_embedding vector(1586),
  match_count int default 10,
  filter_language text default null
)
returns table (
  id text,
  title text,
  description text,
  channel_title text,
  language text,
  view_count bigint,
  like_count bigint,
  published_at timestamptz,
  thumbnail_url text,
  url text,
  similarity float
)
language sql
stable
as $$
  select
    v.id,
    v.title,
    v.description,
    v.channel_title,
    v.language,
    v.view_count,
    v.like_count,
    v.published_at,
    v.thumbnail_url,
    v.url,
    1 - (v.embedding <=> query_embedding) as similarity
  from public.videos v
  where v.embedding is not null
    and (filter_language is null or v.language = filter_language)
  order by v.embedding <=> query_embedding
  limit greatest(match_count, 1);
$$;
