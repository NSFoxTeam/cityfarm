# External Integrations

**Analysis Date:** 2026-03-09

## APIs & External Services

**Firmware -> Backend API:**
- The firmware agent sends sensor readings to the backend via HTTP POST
- Endpoint: `POST /api/v1/readings` (batch insert)
- Auth: `X-API-Key` header
- Client: `reqwest` 0.12 (`firmware/src/transport/http.rs`)
- Retry: 3 attempts with exponential backoff, no retry on 4xx errors
- Offline resilience: readings buffered in local SQLite (`firmware/src/transport/buffer.rs`)

**Frontend -> Backend API:**
- The frontend fetches sensor data from the backend REST API
- Client: native `fetch` API (`frontend/src/api/readings.ts`)
- Base URL: `VITE_API_URL` env var (default `http://localhost:8080/api/v1`)
- Endpoints consumed:
  - `GET /api/v1/readings/latest` - Latest reading per sensor type
  - `GET /api/v1/readings/history?sensor_type=X&from=T&to=T` - Time-range history
- Polling: TanStack Query with 10-second `refetchInterval` (`frontend/src/hooks/useReadings.ts`)

**ML Inference Server (planned):**
- FastAPI server at port 8000 (`ml/src/inference/`)
- YOLO11 model for plant health detection
- Module stubs exist but implementation files are `__init__.py` only

## Backend REST API Endpoints

**Defined in `backend/cmd/cityfarm-api/main.go`:**

| Method | Path | Auth | Handler |
|--------|------|------|---------|
| GET | `/healthz` | None | Inline health check |
| POST | `/api/v1/readings` | API Key | `handlers.ReadingsHandler.PostReadings` |
| GET | `/api/v1/readings/latest` | API Key | `handlers.ReadingsHandler.GetLatest` |
| GET | `/api/v1/readings/history` | API Key | `handlers.ReadingsHandler.GetHistory` |

**Middleware stack (`backend/cmd/cityfarm-api/main.go`):**
- `middleware.RequestID` - Request ID generation
- `middleware.RealIP` - Real IP extraction
- `middleware.Recoverer` - Panic recovery
- `cors.Handler` - CORS (all origins allowed)
- `api.APIKeyAuth` - API key validation on `/api/v1` routes (`backend/internal/api/middleware.go`)

## Data Storage

**Primary Database - TimescaleDB (PostgreSQL):**
- Driver: `pgx/v5` v5.8.0 (native PostgreSQL driver, not database/sql)
- Connection: `DATABASE_URL` env var
- Pool: pgxpool with min 2, max 10 connections (`backend/cmd/cityfarm-api/main.go`)
- Table: `sensor_readings` with columns: `time`, `sensor_type`, `value`, `unit`, `level`
- Queries use `DISTINCT ON` for latest readings (PostgreSQL-specific)
- Batch inserts via `pgx.Batch` (`backend/internal/store/readings.go`)
- No ORM - raw SQL queries

**Local Buffer - SQLite (Firmware):**
- Driver: `rusqlite` 0.31 (bundled SQLite)
- Path: `/opt/cityfarm/buffer.db` (configurable)
- Table: `readings_buffer` with columns: `id`, `payload` (JSON), `created_at`
- Purpose: Offline resilience when backend is unreachable
- Flush strategy: pop up to 100 entries per cycle, stop on first send failure (`firmware/src/scheduler.rs`)

**Calibration Data (Firmware):**
- Format: JSON file at `/opt/cityfarm/calibration.json`
- Contains: pH sensor slope and offset values
- Module: `firmware/src/calibration.rs`

**File Storage:**
- No cloud file storage
- ML datasets stored locally in `ml/data/`

**Caching:**
- None (no Redis, no in-memory cache)

## Authentication & Identity

**API Key Auth (machine-to-machine only):**
- Implementation: `backend/internal/api/middleware.go`
- Method: `X-API-Key` HTTP header
- Validation: constant-time comparison via `crypto/subtle.ConstantTimeCompare`
- Config: `API_KEY` env var on backend, `api_key` in firmware config.toml
- No user authentication system exists
- No session management, no JWT, no OAuth

## Monitoring & Observability

**Backend Logging:**
- Framework: Go `log/slog` with JSON handler (`backend/cmd/cityfarm-api/main.go`)
- Level: Info (default)
- Output: stdout (JSON structured logs)

**Firmware Logging:**
- Framework: `tracing` + `tracing-subscriber` with `EnvFilter` (`firmware/src/main.rs`)
- Level: Info (default), configurable via `RUST_LOG` env var
- Output: stdout (formatted)

**Frontend Logging:**
- None (no error tracking, no analytics)

**Error Tracking:**
- None (no Sentry, no Datadog, no external monitoring)

**Alerting:**
- In-process only: `backend/internal/alerting/thresholds.go`
- Threshold-based alerts logged as warnings (no external notification)
- Sensor types monitored: pH, TDS, EC, solution_temp, temperature, humidity, moisture
- Alert levels: normal, warning, critical
- No push notifications, no email, no webhook alerts

## CI/CD & Deployment

**GitHub Actions - `.github/workflows/`:**
- `claude.yml` - Claude Code automation
- `dispatch.yml` - Manual dispatch workflows
- `auto-project.yml` - Auto-project management

**Deployment Infrastructure - `deploy/`:**
- `deploy/docker/` - Docker Compose for VPS deployment (directory exists, contents not populated)
- `deploy/nginx/` - Nginx reverse proxy config (empty)
- `deploy/systemd/` - systemd service files (empty)
- `deploy/ansible/playbooks/` - Ansible playbooks (empty directory)

**Firmware Deployment:**
- Manual: cross-compile on macOS, SCP binary to RPi
- Command: `cross build --release --target aarch64-unknown-linux-gnu`
- Deploy path: `/opt/cityfarm/` on Raspberry Pi

**Hosting:**
- Backend + DB: VPS (provider not specified)
- Firmware: Raspberry Pi 5 (on-premise, SSH accessible at 192.168.1.175)
- Frontend: Not deployed yet (dev server only)

## Hardware Integrations (Firmware)

**I2C Bus (`/dev/i2c-1`):**
- TCA9548A I2C multiplexer at address 0x70
- BH1750 light sensors at 0x23 via mux channels 0-2
- ADS1115 16-bit ADC at 0x48 via mux channel 7
- Shared bus pattern: `Arc<Mutex<I2cdev>>` (`firmware/src/scheduler.rs`)

**ADC Channels (via ADS1115):**
- A0: YL-69 soil moisture sensor (`firmware/src/sensors/moisture.rs`)
- A1: DFRobot pH V2 sensor (`firmware/src/sensors/ph.rs`)
- A2: DFRobot TDS sensor (`firmware/src/sensors/tds.rs`)
- Shared ADC: `SharedAds1115` abstraction (`firmware/src/sensors/ads1115.rs`)

**GPIO:**
- GPIO4: DHT22 temperature/humidity (single-wire protocol) (`firmware/src/sensors/dht22.rs`)
- GPIO17: Relay for water pump (active LOW) (`firmware/src/actuators/relay.rs`)
- GPIO22: DS18B20 temperature (1-Wire protocol) (`firmware/src/sensors/ds18b20.rs`)

**Camera (planned):**
- Xiaomi Mijia 360 PTZ 2K via RTSP
- Not yet integrated in code

## Webhooks & Callbacks

**Incoming:**
- None

**Outgoing:**
- None

## Environment Configuration

**Required env vars (Backend):**
- `DATABASE_URL` - TimescaleDB connection string
- `API_KEY` - Shared secret for firmware auth

**Optional env vars (Backend):**
- `LISTEN_ADDR` - Server bind address (default `:8080`)

**Required env vars (Frontend):**
- `VITE_API_URL` - Backend API base URL (for production builds)

**Required config (Firmware):**
- `/opt/cityfarm/config.toml` with `backend_url` and `api_key`

**Secrets location:**
- No `.env` files detected in repo
- Backend: env vars (likely set in systemd unit or Docker)
- Firmware: config.toml on Raspberry Pi filesystem

## Optional External Services (ML)

**Weights & Biases (optional):**
- Package: `wandb` >=0.19 (`ml/pyproject.toml` tracking extra)
- Purpose: ML experiment tracking
- Not required for runtime

**Label Studio (optional):**
- Package: `label-studio-sdk` >=1.0 (`ml/pyproject.toml` label extra)
- Purpose: Dataset labeling
- Not required for runtime

---

*Integration audit: 2026-03-09*
