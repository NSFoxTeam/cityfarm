# Architecture

**Analysis Date:** 2026-03-09

## Pattern Overview

**Overall:** IoT Edge-Cloud Monorepo — a three-tier system with an edge agent (firmware), a REST API server (backend), and a web dashboard (frontend), plus a placeholder ML pipeline.

**Key Characteristics:**
- Edge agent on Raspberry Pi 5 collects sensor data and pushes to backend via HTTP REST
- Store-and-forward pattern: firmware buffers readings in SQLite, retries on network failure
- Backend is a stateless REST API backed by TimescaleDB (time-series optimized PostgreSQL)
- Frontend polls the backend API every 10 seconds for live sensor data
- Monorepo with four independent language ecosystems (Rust, Go, TypeScript, Python)
- No WebSocket or real-time push yet (directory exists as placeholder)

## Layers

**Edge Layer (Firmware):**
- Purpose: Read physical sensors, control actuators, buffer and transmit data
- Location: `firmware/src/`
- Contains: Sensor drivers, actuator control, scheduling loop, HTTP transport, SQLite buffer
- Depends on: Hardware (I2C, GPIO, 1-Wire), backend REST API
- Used by: Backend (receives POST /api/v1/readings)

**API Layer (Backend):**
- Purpose: Receive sensor readings, store in TimescaleDB, serve data to frontend
- Location: `backend/`
- Contains: HTTP handlers, middleware, data store, alerting logic, domain models
- Depends on: TimescaleDB (pgx connection pool)
- Used by: Firmware (POST readings), Frontend (GET latest/history)

**Presentation Layer (Frontend):**
- Purpose: Display real-time sensor dashboard with charts and alerts
- Location: `frontend/src/`
- Contains: React components, API client, TanStack Query hooks, types
- Depends on: Backend REST API
- Used by: End users (browser)

**ML Layer (Placeholder):**
- Purpose: YOLO11 plant health inference pipeline
- Location: `ml/src/`
- Contains: Empty `__init__.py` files in `dataset/`, `training/`, `export/`, `inference/`
- Depends on: Not yet implemented
- Used by: Not yet integrated

## Data Flow

**Sensor Reading Pipeline:**

1. Firmware scheduler (`firmware/src/scheduler.rs`) wakes every N seconds (default: 10s)
2. Reads all sensors sequentially: DS18B20 -> pH -> TDS/EC -> DHT22 -> BH1750 -> Moisture
3. Temperature compensation: DS18B20 solution temp feeds into pH and TDS calculations
4. Readings serialized to JSON and pushed to local SQLite buffer (`firmware/src/transport/buffer.rs`)
5. Buffer flushes up to 100 batches to backend via HTTP POST with retry/backoff (`firmware/src/transport/http.rs`)
6. Backend handler validates readings, batch-inserts into TimescaleDB (`backend/internal/api/handlers/readings.go`)
7. Backend checks alert thresholds per reading, logs warnings (`backend/internal/alerting/thresholds.go`)
8. Frontend polls GET `/api/v1/readings/latest` every 10s via TanStack Query (`frontend/src/hooks/useReadings.ts`)
9. Dashboard renders sensor cards and 24h water quality chart (`frontend/src/pages/Dashboard.tsx`)

**State Management:**
- Firmware: Stateless loop + SQLite buffer for durability. Sensor state held in memory structs.
- Backend: Stateless HTTP server. All state in TimescaleDB. Connection pool: min 2, max 10.
- Frontend: TanStack Query manages server state. `staleTime: 5000ms`, `refetchInterval: 10000ms`. No client-side global state store.

## Key Abstractions

**Sensor Trait (Firmware):**
- Purpose: Unified interface for all sensor drivers
- Definition: `firmware/src/sensors/mod.rs` — `trait Sensor: Send + Sync { async fn read(&self) -> Result<Vec<Reading>>; fn name(&self) -> &str; }`
- Implementations: `firmware/src/sensors/dht22.rs`, `firmware/src/sensors/bh1750.rs`, `firmware/src/sensors/moisture.rs`
- Note: pH and TDS sensors do NOT implement the `Sensor` trait — they use custom `read_with_temp()` for temperature compensation

**Reading (Firmware):**
- Purpose: Universal sensor data point
- Definition: `firmware/src/sensors/mod.rs` — struct with `sensor_type`, `value`, `unit`, `level`, `timestamp`
- Pattern: Created via `Reading::new(SensorType, f64, &'static str, u8)`

**Reading (Backend):**
- Purpose: Domain model for sensor data
- Definition: `backend/internal/models/reading.go` — struct with `Time`, `SensorType`, `Value`, `Unit`, `Level`
- Pattern: Includes `Validate()` method with range checks per sensor type

**SharedAds1115 (Firmware):**
- Purpose: Thread-safe wrapper for shared ADC hardware
- Definition: `firmware/src/sensors/ads1115.rs`
- Pattern: Wraps `Arc<Mutex<I2cdev>>` to allow multiple sensor drivers (pH, TDS, moisture) to share one ADC

**SqliteBuffer (Firmware):**
- Purpose: Durable local buffer for network resilience
- Definition: `firmware/src/transport/buffer.rs`
- Pattern: Push readings as JSON, pop in FIFO order, delete after confirmed delivery

## Entry Points

**Firmware CLI:**
- Location: `firmware/src/main.rs`
- Triggers: `cityfarm-agent run | calibrate ph | test-sensors`
- Responsibilities: Parse CLI args (clap), load TOML config, dispatch to `scheduler::run()`, `calibrate_ph()`, or `test_sensors()`

**Backend API Server:**
- Location: `backend/cmd/cityfarm-api/main.go`
- Triggers: `go run ./cmd/cityfarm-api/`
- Responsibilities: Connect to TimescaleDB, set up Chi router with middleware, register API routes, graceful shutdown on SIGTERM

**Frontend SPA:**
- Location: `frontend/src/main.tsx` -> `frontend/src/App.tsx`
- Triggers: Vite dev server or static build
- Responsibilities: Mount React app, set up QueryClient and BrowserRouter, render Dashboard

## API Routes

**Backend REST API (`/api/v1/`):**
- All routes require `X-API-Key` header (middleware: `backend/internal/api/middleware.go`)
- `POST /api/v1/readings` — Insert batch of sensor readings
- `GET /api/v1/readings/latest` — Get latest reading per sensor type (with alert levels)
- `GET /api/v1/readings/history?sensor_type=X&from=T&to=T` — Get time-range history for a sensor

**Health Check:**
- `GET /healthz` — No auth required, returns `{"status":"ok"}`

## Error Handling

**Firmware Strategy:** `anyhow::Result` throughout. Individual sensor failures are logged and skipped (scheduler continues). Network failures trigger retry with exponential backoff (max 3 attempts). 4xx errors from backend are not retried.

**Backend Strategy:** HTTP handler methods return JSON error responses with appropriate status codes. Store errors wrapped with `fmt.Errorf("store.readings.method: %w", err)` for traceability. Panics recovered by Chi `middleware.Recoverer`.

**Frontend Strategy:** TanStack Query handles retries (2 retries by default). API client throws on non-OK responses. No error boundary components yet.

## Cross-Cutting Concerns

**Logging:**
- Firmware: `tracing` crate with `EnvFilter` (default: info). JSON-compatible structured logs.
- Backend: `slog` with JSON handler. Logger injected into handlers.
- Frontend: No structured logging.

**Validation:**
- Firmware: Sensor type enum enforced at compile time. Readings carry typed `SensorType`.
- Backend: `Reading.Validate()` checks sensor type existence and value ranges per type.
- Frontend: TypeScript types for `SensorType` and `Reading` at `frontend/src/types/sensor.ts`.

**Authentication:**
- API key auth via `X-API-Key` header. Constant-time comparison in `backend/internal/api/middleware.go`.
- No user authentication. No session management.

**Alerting:**
- Server-side threshold checks in `backend/internal/alerting/thresholds.go` with warn/critical levels.
- Alert levels returned alongside readings in GET `/readings/latest`.
- Frontend displays alert banners and color-coded sensor cards.

## Database Schema

**TimescaleDB (single hypertable):**
- Migration: `backend/migrations/001_create_sensor_readings.sql`
- Table: `sensor_readings` with columns: `time TIMESTAMPTZ`, `sensor_type TEXT`, `value DOUBLE PRECISION`, `unit TEXT`, `level SMALLINT`
- Hypertable on `time` column for time-series optimization
- Index: `idx_sensor_readings_type_time` on `(sensor_type, time DESC)`

## Placeholder Modules (Not Yet Implemented)

- `backend/internal/websocket/` — Empty, intended for real-time push
- `backend/internal/ml/` — Empty, intended for ML inference proxy
- `backend/internal/storage/` — Empty, intended for file/image storage
- `ml/src/` — All `__init__.py` empty, intended for YOLO11 pipeline
- `frontend/src/components/Camera/` — Empty, intended for RTSP camera view
- `frontend/src/components/Controls/` — Empty, intended for pump/relay controls
- `frontend/src/components/Plants/` — Empty, intended for plant health display

---

*Architecture analysis: 2026-03-09*
