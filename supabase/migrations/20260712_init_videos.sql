-- HelixQ v1: videos metadata + pgvector
-- Paste into Supabase SQL Editor and run once.

-- 1) Vector similarity support
create extension if not exists vector;

-- 2) One row = one YouTube video (metadata only, no transcripts)
create table if not exists public.videos (
  id text primary key,                          -- YouTube video id
  title text not null,
  description text not null default '',         -- truncated on ingest (~800 chars)
  channel_id text,
  channel_title text,
  language text not null default 'und',         -- e.g. en / zh / fr / und
  view_count bigint not null default 0,
  like_count bigint not null default 0,
  published_at timestamptz,
  thumbnail_url text,
  url text not null,
  -- text-embedding-3-small => 1536 dims
  embedding vector(1536),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

-- 3) Keep updated_at fresh on row changes
create or replace function public.set_updated_at()
returns trigger
language plpgsql
as $$
begin
  new.updated_at = now();
  return new;
end;
$$;

drop trigger if exists videos_set_updated_at on public.videos;
create trigger videos_set_updated_at
before update on public.videos
for each row
execute function public.set_updated_at();

-- 4) Useful filters / sorts for search + UI
create index if not exists videos_language_idx on public.videos (language);
create index if not exists videos_view_count_idx on public.videos (view_count desc);
create index if not exists videos_published_at_idx on public.videos (published_at desc);

-- 5) Vector index (builds fully once you have embeddings; fine empty for now)
-- cosine distance matches typical embedding search
create index if not exists videos_embedding_hnsw_idx
on public.videos
using hnsw (embedding vector_cosine_ops);

-- 6) Semantic match helper used later by the Rust search API
create or replace function public.match_videos(
  query_embedding vector(1536),
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

-- 7) Smoke-test row (optional). Delete after you confirm the table works.
insert into public.videos (
  id, title, description, channel_id, channel_title, language,
  view_count, like_count, published_at, thumbnail_url, url
) values (
  'smoke_test_001',
  'HelixQ schema smoke test',
  'Temporary row to verify insert/select. Safe to delete.',
  'channel_test',
  'HelixQ Dev',
  'en',
  0,
  0,
  now(),
  null,
  'https://www.youtube.com/watch?v=smoke_test_001'
)
on conflict (id) do nothing;
