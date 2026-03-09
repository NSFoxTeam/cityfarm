# Technology Stack

**Analysis Date:** 2026-03-09

## Languages

**Primary:**
- **Rust** (edition 2021) - Firmware/edge agent (`firmware/`)
- **Go** 1.24.6 - Backend API server (`backend/`)
- **TypeScript** ~5.9.3 - Frontend dashboard (`frontend/`)

**Secondary:**
- **Python** >=3.10 - ML pipeline (`ml/`)
- **YAML/Docker/Ansible** - Deployment configs (`deploy/`)

## Runtime

**Firmware (Raspberry Pi 5):**
- Target: `aarch64-unknown-linux-gnu`
- Cross-compile via `cross` from macOS
- Config file: `/opt/cityfarm/config.toml` (TOML format)
- Local buffer DB: `/opt/cityfarm/buffer.db` (SQLite)

**Backend (VPS):**
- Go 1.24.6
- Standard `net/http` server with Chi router
- Default listen address: `:8080`

**Frontend:**
- Node.js (version not pinned, no `.nvmrc`)
- Package Manager: npm
- Lockfile: `frontend/package-lock.json` (present)

**ML:**
- Python >=3.10
- Package manager: pip (pyproject.toml, no lockfile detected)

## Frameworks

**Core:**
- **Chi** v5.2.5 - Go HTTP router (`backend/go.mod`)
- **React** 19.2.0 - Frontend UI (`frontend/package.json`)
- **Vite** 7.3.1 - Frontend build/dev server (`frontend/vite.config.ts`)
- **Tokio** 1.x (full features) - Rust async runtime (`firmware/Cargo.toml`)
- **FastAPI** >=0.115 - ML inference server (`ml/pyproject.toml`)

**Testing:**
- **pytest** >=8.0 - Python tests (`ml/pyproject.toml` dev dependency)
- No test framework configured for frontend, backend, or firmware

**Build/Dev:**
- **Vite** 7.3.1 + `@vitejs/plugin-react` 5.1.1 - Frontend bundler
- **cross** - Rust cross-compilation for aarch64
- **TypeScript** ~5.9.3 - Type checking (`tsc -b` before build)
- **ESLint** 9.39.1 + `typescript-eslint` 8.48.0 - Frontend linting

## Key Dependencies

**Backend (Go) - `backend/go.mod`:**
- `github.com/go-chi/chi/v5` v5.2.5 - HTTP routing
- `github.com/go-chi/cors` v1.2.2 - CORS middleware
- `github.com/jackc/pgx/v5` v5.8.0 - PostgreSQL driver (native, not database/sql)

**Frontend (TypeScript) - `frontend/package.json`:**
- `react` ^19.2.0 / `react-dom` ^19.2.0 - UI framework
- `@tanstack/react-query` ^5.90.21 - Server state management
- `recharts` ^3.7.0 - Charting library
- `tailwindcss` ^4.2.1 + `@tailwindcss/vite` ^4.2.1 - CSS framework (v4, Vite plugin)
- `radix-ui` ^1.4.3 - Headless UI primitives
- `class-variance-authority` ^0.7.1 + `clsx` ^2.1.1 + `tailwind-merge` ^3.5.0 - shadcn/ui utilities
- `lucide-react` ^0.577.0 - Icons
- `date-fns` ^4.1.0 - Date formatting
- `react-router-dom` ^7.13.1 - Client-side routing
- `shadcn` ^3.8.5 (devDependency) - Component CLI tool

**Firmware (Rust) - `firmware/Cargo.toml`:**
- `rppal` 0.19 - Raspberry Pi GPIO/I2C/SPI access
- `linux-embedded-hal` 0.4 + `embedded-hal` 1.0 - Hardware abstraction layer
- `ads1x1x` 0.3 - ADS1115 ADC driver
- `tokio` 1.x (full) - Async runtime
- `reqwest` 0.12 (json feature) - HTTP client for backend communication
- `rusqlite` 0.31 (bundled) - Local SQLite buffer for offline resilience
- `serde` 1.x + `serde_json` 1.x - Serialization
- `clap` 4.x (derive) - CLI argument parsing
- `tracing` 0.1 + `tracing-subscriber` 0.3 - Structured logging
- `toml` 0.8 - Config file parsing
- `chrono` 0.4 - Timestamps
- `anyhow` 1.x - Error handling

**ML (Python) - `ml/pyproject.toml`:**
- `ultralytics` >=8.3 - YOLO11 training/inference
- `fastapi` >=0.115 - Inference API server
- `uvicorn` >=0.34 - ASGI server
- `onnxruntime` >=1.19 - Optimized model inference
- `albumentations` >=1.4 - Image augmentation
- `httpx` >=0.28 - Async HTTP client
- `Pillow` >=10.0 - Image processing
- Optional: `wandb` >=0.19 (experiment tracking), `label-studio-sdk` >=1.0 (labeling)
- Dev: `ruff` >=0.8 (linting/formatting)

## Configuration

**Environment Variables (Backend) - `backend/cmd/cityfarm-api/main.go`:**
- `DATABASE_URL` - PostgreSQL connection string (default: `postgres://cityfarm:cityfarm@localhost:5432/cityfarm?sslmode=disable`)
- `API_KEY` - API key for firmware->backend auth (default: `dev-api-key`)
- `LISTEN_ADDR` - Server listen address (default: `:8080`)

**Environment Variables (Frontend) - `frontend/src/api/readings.ts`:**
- `VITE_API_URL` - Backend API URL (default: `http://localhost:8080/api/v1`)

**Config File (Firmware) - `firmware/src/config.rs`:**
- Path: `/opt/cityfarm/config.toml` (configurable via `-c` CLI flag)
- Fields: `backend_url`, `api_key`, `read_interval_secs` (default 10), `calibration_path`, `db_path`, `relay.gpio_pin`, `relay.max_duration_secs`, `relay.cooldown_secs`

**Build Configuration:**
- `frontend/vite.config.ts` - Vite config with React plugin, Tailwind CSS plugin, `@` path alias to `./src`
- `frontend/tsconfig.json` + `frontend/tsconfig.app.json` + `frontend/tsconfig.node.json` - TypeScript configs
- `frontend/eslint.config.js` - ESLint flat config

## Platform Requirements

**Development (macOS):**
- Go 1.24.6+
- Node.js + npm (for frontend)
- Rust toolchain + `cross` (for firmware cross-compilation)
- Python 3.10+ (for ML)
- Docker (for cross-compilation and deploy)

**Production - Raspberry Pi 5:**
- `aarch64-unknown-linux-gnu` target
- I2C bus enabled (`/dev/i2c-1`)
- 1-Wire enabled (GPIO4 for DHT22, GPIO22 for DS18B20)
- GPIO access (GPIO17 for relay)
- SQLite for local buffering
- Binary deployed to `/opt/cityfarm/`

**Production - VPS:**
- Go binary
- TimescaleDB (PostgreSQL extension)
- Docker Compose (`deploy/docker/`)
- Nginx reverse proxy (`deploy/nginx/`)
- systemd services (`deploy/systemd/`)

---

*Stack analysis: 2026-03-09*
