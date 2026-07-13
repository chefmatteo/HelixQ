export type MockVideo = {
  id: string;
  title: string;
  description: string;
  channel_title: string;
  language: string;
  view_count: number;
  like_count: number;
  published_at: string;
  thumbnail_url: string;
  url: string;
};

export const MOCK_VIDEOS: MockVideo[] = [
  {
    id: "dQw4w9WgXcQ",
    title: "Rust for Systems Programming — Crash Course",
    description:
      "Ownership, borrowing, and lifetimes explained with small runnable examples for people coming from Python or C++.",
    channel_title: "Systems Lab",
    language: "en",
    view_count: 1_240_000,
    like_count: 48_200,
    published_at: "2024-11-02",
    thumbnail_url: "https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg",
    url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  },
  {
    id: "jNQXAC9IVRw",
    title: "Building a Search Index with Tantivy",
    description:
      "Walkthrough of inverted indexes, BM25 scoring, and how to ship a fast full-text search API in Rust.",
    channel_title: "Index Notes",
    language: "en",
    view_count: 86_400,
    like_count: 4_120,
    published_at: "2025-03-14",
    thumbnail_url: "https://i.ytimg.com/vi/jNQXAC9IVRw/hqdefault.jpg",
    url: "https://www.youtube.com/watch?v=jNQXAC9IVRw",
  },
  {
    id: "9bZkp7q19f0",
    title: "pgvector in Practice: Semantic Search on Postgres",
    description:
      "Embeddings, HNSW indexes, and hybrid retrieval patterns using Supabase and Postgres extensions.",
    channel_title: "Data Plane",
    language: "en",
    view_count: 210_500,
    like_count: 9_880,
    published_at: "2025-01-20",
    thumbnail_url: "https://i.ytimg.com/vi/9bZkp7q19f0/hqdefault.jpg",
    url: "https://www.youtube.com/watch?v=9bZkp7q19f0",
  },
  {
    id: "3JZ_D3ELwOQ",
    title: "YouTube Data API: Metadata Ingest Pipeline",
    description:
      "Fetch titles, descriptions, views, and likes with quota-aware batching — no transcript scraping required.",
    channel_title: "API Craft",
    language: "en",
    view_count: 42_300,
    like_count: 1_950,
    published_at: "2025-06-01",
    thumbnail_url: "https://i.ytimg.com/vi/3JZ_D3ELwOQ/hqdefault.jpg",
    url: "https://www.youtube.com/watch?v=3JZ_D3ELwOQ",
  },
  {
    id: "kJQP7kiw5Fk",
    title: "Moteurs de recherche : BM25 expliqué simplement",
    description:
      "Une introduction claire au ranking lexical, aux documents et aux requêtes pour débutants en IR.",
    channel_title: "Algo FR",
    language: "fr",
    view_count: 18_700,
    like_count: 890,
    published_at: "2024-09-12",
    thumbnail_url: "https://i.ytimg.com/vi/kJQP7kiw5Fk/hqdefault.jpg",
    url: "https://www.youtube.com/watch?v=kJQP7kiw5Fk",
  },
  {
    id: "fJ9rUzIMcZQ",
    title: "分布式搜索引擎入门：索引与查询路径",
    description:
      "从爬取到索引再到查询的完整链路概览，适合想自己做一个小型搜索引擎的开发者。",
    channel_title: "Infra CN",
    language: "zh",
    view_count: 95_200,
    like_count: 5_430,
    published_at: "2025-02-08",
    thumbnail_url: "https://i.ytimg.com/vi/fJ9rUzIMcZQ/hqdefault.jpg",
    url: "https://www.youtube.com/watch?v=fJ9rUzIMcZQ",
  },
  {
    id: "OPf0YbXqDm0",
    title: "Hybrid Search: Why Keyword + Vector Beats Either Alone",
    description:
      "Reciprocal rank fusion, when semantic search fails, and how engagement signals change ranking.",
    channel_title: "Retrieval Weekly",
    language: "en",
    view_count: 157_000,
    like_count: 7_640,
    published_at: "2025-04-22",
    thumbnail_url: "https://i.ytimg.com/vi/OPf0YbXqDm0/hqdefault.jpg",
    url: "https://www.youtube.com/watch?v=OPf0YbXqDm0",
  },
  {
    id: "hTWKbfoikeg",
    title: "Supabase Auth is Not Enough — Designing a Search Backend",
    description:
      "Schema design for video metadata, embedding columns, and keeping Tantivy in sync with Postgres.",
    channel_title: "Backend Brief",
    language: "en",
    view_count: 33_100,
    like_count: 1_420,
    published_at: "2025-05-17",
    thumbnail_url: "https://i.ytimg.com/vi/hTWKbfoikeg/hqdefault.jpg",
    url: "https://www.youtube.com/watch?v=hTWKbfoikeg",
  },
];

export function formatCount(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
  if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
  return String(n);
}

export function filterVideos(videos: MockVideo[], query: string, language: string): MockVideo[] {
  const q = query.trim().toLowerCase();
  return videos.filter((video) => {
    if (language !== "all" && video.language !== language) return false;
    if (!q) return true;
    const haystack = `${video.title} ${video.description} ${video.channel_title}`.toLowerCase();
    return haystack.includes(q);
  });
}
