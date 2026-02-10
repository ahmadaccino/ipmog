# Contributing to ipmog

## Getting started

1. Fork the repo and clone your fork
2. Create a branch for your change (`git checkout -b my-change`)
3. Make your changes
4. Open a pull request against `main`

## Running locally

### TUI app

```
cargo run
```

### Cloudflare Worker

```
cd worker
cp wrangler.toml.example wrangler.toml
# Add your Cloudflare account ID to wrangler.toml
npm install
npx wrangler dev
```

Then point the TUI at your local worker:

```
IPMOG_URL=http://localhost:8787 cargo run
```

## Guidelines

- Keep PRs focused — one change per PR
- Test that the TUI renders correctly before submitting
- Follow the existing code style
