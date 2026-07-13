"use client";

import Image from "next/image";
import { useMemo, useState } from "react";
import {
  MOCK_VIDEOS,
  filterVideos,
  formatCount,
  type MockVideo,
} from "@/data/mock-videos";

function ResultRow({ video, index }: { video: MockVideo; index: number }) {
  return (
    <a
      href={video.url}
      target="_blank"
      rel="noreferrer"
      className="group grid grid-cols-[100px_1fr] gap-4 border-b border-white/15 py-4 transition-colors hover:bg-white/[0.03] sm:grid-cols-[140px_1fr] sm:gap-5"
    >
      <div className="relative aspect-video overflow-hidden border border-white/20 bg-black">
        <Image
          src={video.thumbnail_url}
          alt=""
          fill
          className="object-cover opacity-80 grayscale transition-[opacity,filter] group-hover:opacity-100 group-hover:grayscale-0"
          sizes="140px"
        />
      </div>
      <div className="min-w-0 font-mono">
        <p className="text-[11px] tracking-wide text-white/40">
          [{String(index + 1).padStart(2, "0")}] {video.id}
        </p>
        <h2 className="mt-1 text-sm font-normal leading-snug text-white group-hover:underline sm:text-[15px]">
          {video.title}
        </h2>
        <p className="mt-1.5 text-xs text-white/45">
          {video.channel_title}
          <span className="mx-2 text-white/20">|</span>
          {formatCount(video.view_count)} views
          <span className="mx-2 text-white/20">|</span>
          {formatCount(video.like_count)} likes
          <span className="mx-2 text-white/20">|</span>
          <span className="uppercase">{video.language}</span>
        </p>
        <p className="mt-2 line-clamp-2 text-xs leading-relaxed text-white/55">
          {video.description}
        </p>
      </div>
    </a>
  );
}

export default function SearchHome() {
  const [query, setQuery] = useState("");
  const [submitted, setSubmitted] = useState("");
  const [language, setLanguage] = useState("all");

  const results = useMemo(
    () => filterVideos(MOCK_VIDEOS, submitted, language),
    [submitted, language],
  );

  const hasSearched = submitted.length > 0 || language !== "all";

  function onSubmit(event: React.FormEvent) {
    event.preventDefault();
    setSubmitted(query.trim());
  }

  return (
    <div className="mx-auto flex min-h-[calc(100vh-3.5rem)] w-full max-w-[880px] flex-col px-5 sm:px-6">
      <section className="flex flex-col items-center pb-8 pt-16 text-center sm:pt-24">
        <p className="raycast-display text-balance">HelixQ</p>
        <p className="mt-3 max-w-md font-mono text-sm text-white/50">
          Search YouTube by title, description, and engagement — not transcripts.
        </p>

        <form id="search" onSubmit={onSubmit} className="mt-10 w-full max-w-[640px] scroll-mt-28">
          <label htmlFor="helixq-query" className="sr-only">
            Search
          </label>
          <div className="flex items-stretch border border-white/25 bg-black focus-within:border-white">
            <span
              aria-hidden
              className="hidden select-none items-center border-r border-white/15 px-3 font-mono text-sm text-white/40 sm:flex"
            >
              &gt;
            </span>
            <input
              id="helixq-query"
              value={query}
              onChange={(event) => setQuery(event.target.value)}
              placeholder="try: tantivy, pgvector, BM25, rust…"
              className="min-w-0 flex-1 bg-transparent px-3 py-3 font-mono text-sm text-white outline-none placeholder:text-white/30 sm:px-4"
              autoComplete="off"
              spellCheck={false}
            />
            <button
              type="submit"
              className="shrink-0 border-l border-white/25 bg-white px-4 py-3 font-mono text-xs font-medium uppercase tracking-wider text-black transition-colors hover:bg-white/90"
            >
              Search
            </button>
          </div>

          <div className="mt-4 flex items-center justify-center gap-3 font-mono text-xs text-white/45">
            <label htmlFor="language">lang</label>
            <select
              id="language"
              value={language}
              onChange={(event) => setLanguage(event.target.value)}
              className="border border-white/20 bg-black px-2 py-1.5 text-xs text-white outline-none focus:border-white"
            >
              <option value="all">all</option>
              <option value="en">en</option>
              <option value="fr">fr</option>
              <option value="zh">zh</option>
            </select>
          </div>
        </form>
      </section>

      {hasSearched && (
        <section className="pb-16 font-mono" aria-live="polite">
          <div className="mb-1 flex items-baseline justify-between gap-3 border-b border-white/20 pb-3">
            <h2 className="text-xs tracking-wide text-white/60">
              {results.length} result{results.length === 1 ? "" : "s"}
              {submitted ? (
                <>
                  {" "}
                  for <span className="text-white">&ldquo;{submitted}&rdquo;</span>
                </>
              ) : null}
            </h2>
            <p className="text-[11px] text-white/30">mock · client filter</p>
          </div>

          {results.length === 0 ? (
            <p className="py-12 text-center text-sm text-white/45">
              no matches. try another keyword or clear lang filter.
            </p>
          ) : (
            <div>
              {results.map((video, index) => (
                <ResultRow key={video.id} video={video} index={index} />
              ))}
            </div>
          )}
        </section>
      )}
    </div>
  );
}
