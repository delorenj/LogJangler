# LogJangler 🤠

A self-hosted terminal logging service with deep Zellij integration, powered by RabbitMQ.

## Quick Start

```bash
# Start services
make services

# Check compilation (once crates are complete)
cargo check --workspace

# Run tests
cargo test --workspace
```

## What's Been Built

✅ Workspace structure  
✅ Docker Compose (RabbitMQ + Meilisearch)  
✅ Makefile for development  
🚧 Core crate (in progress)  
🚧 Plugins crate (to do)  
🚧 Service crate (to do)

## Next Steps

The foundation is being set up. Core rust files are being generated now.

See GETTING_STARTED.md (to be created) for full instructions.

## Services

- RabbitMQ: http://localhost:15672 (logjangler/logjangler)
- Meilisearch: http://localhost:7700

