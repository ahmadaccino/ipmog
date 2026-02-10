# ipmog

does your friends ever mog their IP in front of you? Do they ever laugh at your screenshot of a shitty ip checker websites with with sex pill ads on it?

well, say goodbye to those days. because there is now the epic IP Mog Checker! mog your ip to your friends in a sexy tui that will make all the foids leave their men to flock to yyou

![screenshot](screenshot.png)
<!-- Replace with an actual terminal screenshot or gif -->

## Features

- World map rendered in the terminal, centered on your location
- Displays IP, city, region, country, ISP, ASN, timezone, and coordinates
- Animated loading screen with ASCII art
- Press `r` to reload, `q` to quit
- Powered by a Cloudflare Worker backend

## Install

From crates.io:

```
cargo install ipmog
```

Or build from source:

```
git clone https://github.com/ahmadaccino/ipmog.git
cd ipmog
cargo build --release
```

The binary will be at `target/release/ipmog`.

## Usage

```
ipmog
```

### Keybindings

| Key | Action  |
|-----|---------|
| `r` | Reload  |
| `q` | Quit    |

### Custom endpoint

By default ipmog hits `https://ip.shnitzel.org`. To use your own Worker:

```
IPMOG_URL=https://your-worker.example.com ipmog
```

## Worker

The `worker/` directory contains the Cloudflare Worker that serves the geolocation API. It uses Cloudflare's built-in `request.cf` object to extract IP info — no third-party APIs needed.

### Setup

```
cd worker
cp wrangler.toml.example wrangler.toml
# Edit wrangler.toml with your Cloudflare account ID
npm install
```

### Local development

```
npx wrangler dev
```

### Deploy

```
npx wrangler deploy
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
