# HelixQ
Rust based search engine

Logic: 
Youtube fetch → embed title+description → POST to Supabase REST → row in `videos`

## Ingest CLI

```bash
# one video
cargo run -- fetch --ids jNQXAC9IVRw

# several videos
cargo run -- fetch --ids jNQXAC9IVRw,dQw4w9WgXcQ,9bZkp7q19f0

# help
cargo run -- --help
cargo run -- fetch --help
```

Requires `.env`: `SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY`, `YOUTUBE_API_KEY`, `OPENROUTER_API_KEY`, `OPENROUTER_EMBED_MODEL`.



### To test the connection with openrouter models:
```bash
curl https://openrouter.ai/api/v1/embeddings \
  -H "Authorization: Bearer $OpenRouterAPI keys" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "nvidia/llama-nemotron-embed-vl-1b-v2:free", # pick any model you want 
    "input": [
      {
        "content": [
          {"type": "text", "text": "What is in this image?"},
          {"type": "image_url", "image_url": {"url": "https://live.staticflickr.com/3851/14825276609_098cac593d_b.jpg"}}
        ]
      }
    ],
    "encoding_format": "float"
  }' 
```


## e2e Search:: 
```
Supabase videos
      │
      ├─► cargo run -- index     → Tantivy folder (data/index)
      │
      └─► cargo run -- serve
              │
              ├─ BM25  (Tantivy)
              ├─ vector (embed query → match_videos RPC)
              └─ merge → JSON /search?q=
```


## Troubleshooting

If it fails:

| Error | Likely cause |
| --- | --- |
| 401 / JWT | Wrong `SUPABASE_SERVICE_ROLE_KEY` |
| 404 | Wrong `SUPABASE_URL` or table name |
| column ... does not exist | Migration not applied / column name mismatch |
| compile error `youtube_inject` | Module name must match your filename |
