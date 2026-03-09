# Codebase Structure

**Analysis Date:** 2026-03-09

## Directory Layout

```
cityfarm/
├── .claude/                # Claude Code agent configs
│   └── agents/             # Sub-agent definitions (rpi-engineer, backend-developer, etc.)
├── .github/
│   └── workflows/          # CI/CD workflows
├── .planning/
│   └── codebase/           # Architecture analysis docs (this file)
├── backend/                # Go API server
│   ├── cmd/
│   │   └── cityfarm-api/   # main.go entry point
│   ├── internal/           # Private packages (Go convention)
│   │   ├── alerting/       # Threshold-based alert logic
│   │   ├── api/            # Middleware (auth)
│   │   │   └── handlers/   # HTTP handler functions
│   │   ├── ml/             # [placeholder] ML inference proxy
│   │   ├── models/         # Domain models (Reading)
│   │   ├── storage/        # [placeholder] File storage
│   │   ├── store/          # Database access layer (pgx)
│   │   └── websocket/      # [placeholder] WebSocket support
│   ├── migrations/         # SQL migration files
│   ├── go.mod
│   └── go.sum
├── deploy/                 # Deployment configs
│   ├── ansible/
│   │   └── playbooks/      # [placeholder]
│   ├── docker/             # [placeholder]
│   ├── nginx/              # [placeholder]
│   └── systemd/            # [placeholder]
├── docs/                   # Documentation
│   ├── api/                # API docs
│   ├── hardware/           # Hardware wiring docs
│   └── setup/              # Setup guides
├── firmware/               # Rust edge agent for RPi 5
│   ├── .cargo/             # Cross-compilation config
│   ├── src/
│   │   ├── actuators/      # Hardware output control
│   │   │   ├── mod.rs
│   │   │   └── relay.rs    # GPIO relay for water pump
│   │   ├── sensors/        # Hardware input drivers
│   │   │   ├── mod.rs      # Sensor trait + Reading struct + SensorType enum
│   │   │   ├── ads1115.rs  # Shared 16-bit ADC driver
│   │   │   ├── bh1750.rs   # Light sensor (I2C)
│   │   │   ├── dht22.rs    # Temp/humidity (GPIO)
│   │   │   ├── ds18b20.rs  # Solution temperature (1-Wire)
│   │   │   ├── moisture.rs # Soil moisture (ADC)
│   │   │   ├── ph.rs       # pH sensor (ADC + calibration)
│   │   │   └── tds.rs      # TDS/EC sensor (ADC)
│   │   ├── transport/      # Data transmission
│   │   │   ├── mod.rs
│   │   │   ├── buffer.rs   # SQLite store-and-forward buffer
│   │   │   └── http.rs     # HTTP client with retry/backoff
│   │   ├── calibration.rs  # pH calibration data (load/save JSON)
│   │   ├── config.rs       # TOML config loading
│   │   ├── main.rs         # CLI entry point (clap)
│   │   └── scheduler.rs    # Main sensor read loop
│   ├── Cargo.toml
│   └── Cargo.lock
├── frontend/               # React dashboard
│   ├── public/             # Static assets
│   ├── src/
│   │   ├── api/            # API client functions
│   │   │   └── readings.ts # fetch wrappers for /readings endpoints
│   │   ├── assets/         # [empty]
│   │   ├── components/
│   │   │   ├── Alerts/
│   │   │   │   └── AlertBanner.tsx
│   │   │   ├── Camera/     # [placeholder]
│   │   │   ├── Charts/
│   │   │   │   └── WaterQualityChart.tsx
│   │   │   ├── Controls/   # [placeholder]
│   │   │   ├── Dashboard/
│   │   │   │   ├── EnvironmentSection.tsx
│   │   │   │   ├── SensorCard.tsx
│   │   │   │   └── WaterQuality.tsx
│   │   │   ├── Plants/     # [placeholder]
│   │   │   └── ui/         # shadcn/ui primitives
│   │   │       ├── badge.tsx
│   │   │       ├── card.tsx
│   │   │       └── skeleton.tsx
│   │   ├── hooks/
│   │   │   └── useReadings.ts  # TanStack Query hooks
│   │   ├── lib/
│   │   │   └── utils.ts    # cn() utility (clsx + tailwind-merge)
│   │   ├── pages/
│   │   │   └── Dashboard.tsx   # Main page component
│   │   ├── types/
│   │   │   └── sensor.ts   # TypeScript types (SensorType, Reading, AlertLevel)
│   │   ├── App.tsx          # Router + QueryClientProvider
│   │   ├── main.tsx         # React DOM entry point
│   │   ├── index.css        # Tailwind + theme styles
│   │   └── vite-env.d.ts   # Vite type declarations
│   ├── package.json
│   └── tsconfig.json
├── ml/                     # Python ML pipeline [placeholder]
│   ├── data/               # Dataset storage
│   └── src/
│       ├── dataset/        # Data preparation
│       ├── export/         # Model export (ONNX)
│       ├── inference/      # Inference server (FastAPI)
│       └── training/       # YOLO11 training
├── AGENTS.md               # Sub-agent routing instructions
├── CLAUDE.md               # Project instructions for Claude
└── .gitignore
```

## Directory Purposes

**`backend/cmd/cityfarm-api/`:**
- Purpose: Single binary entry point for the Go API server
- Contains: `main.go` with server setup, routing, graceful shutdown
- Key files: `backend/cmd/cityfarm-api/main.go`

**`backend/internal/`:**
- Purpose: Private Go packages (not importable outside module)
- Contains: All server logic organized by concern
- Key files: `backend/internal/api/handlers/readings.go`, `backend/internal/store/readings.go`, `backend/internal/models/reading.go`

**`firmware/src/sensors/`:**
- Purpose: One file per physical sensor hardware driver
- Contains: Sensor trait definition, Reading struct, individual sensor implementations
- Key files: `firmware/src/sensors/mod.rs` (trait), `firmware/src/sensors/ads1115.rs` (shared ADC)

**`firmware/src/transport/`:**
- Purpose: Network and persistence for sensor data delivery
- Contains: SQLite buffer and HTTP client
- Key files: `firmware/src/transport/buffer.rs`, `firmware/src/transport/http.rs`

**`frontend/src/components/`:**
- Purpose: UI components organized by feature domain
- Contains: Feature folders (Dashboard, Charts, Alerts) and ui/ primitives
- Key files: `frontend/src/components/Dashboard/SensorCard.tsx`, `frontend/src/components/Charts/WaterQualityChart.tsx`

**`frontend/src/api/`:**
- Purpose: HTTP client layer wrapping backend REST calls
- Contains: Typed fetch functions
- Key files: `frontend/src/api/readings.ts`

**`frontend/src/hooks/`:**
- Purpose: TanStack Query hooks that connect API calls to components
- Contains: Custom hooks per data domain
- Key files: `frontend/src/hooks/useReadings.ts`

## Key File Locations

**Entry Points:**
- `backend/cmd/cityfarm-api/main.go`: Go API server entry point
- `firmware/src/main.rs`: Rust CLI agent entry point
- `frontend/src/main.tsx`: React DOM mount point
- `frontend/src/App.tsx`: React app root (router + providers)

**Configuration:**
- `firmware/src/config.rs`: TOML config struct and loader (runtime config at `/opt/cityfarm/config.toml`)
- `firmware/src/calibration.rs`: pH calibration data (JSON at `/opt/cityfarm/calibration.json`)
- `backend/cmd/cityfarm-api/main.go`: Env vars `DATABASE_URL`, `API_KEY`, `LISTEN_ADDR`
- `frontend/src/api/readings.ts`: `VITE_API_URL` env var

**Core Logic:**
- `firmware/src/scheduler.rs`: Main sensor loop — orchestrates all reads and transport
- `backend/internal/api/handlers/readings.go`: All HTTP handlers (POST/GET readings)
- `backend/internal/store/readings.go`: Database queries (insert batch, get latest, get history)
- `backend/internal/alerting/thresholds.go`: Alert threshold definitions and check logic
- `backend/internal/models/reading.go`: Reading model with validation

**Testing:**
- No test files exist in the codebase yet

## Naming Conventions

**Files:**
- Firmware (Rust): `snake_case.rs` — e.g., `ads1115.rs`, `moisture.rs`
- Backend (Go): `snake_case.go` — e.g., `readings.go`, `middleware.go`
- Frontend (TypeScript): `PascalCase.tsx` for components, `camelCase.ts` for non-components — e.g., `SensorCard.tsx`, `readings.ts`, `useReadings.ts`

**Directories:**
- Firmware: `snake_case` — e.g., `sensors/`, `actuators/`, `transport/`
- Backend: `snake_case` — e.g., `handlers/`, `store/`, `models/`
- Frontend components: `PascalCase` feature folders — e.g., `Dashboard/`, `Charts/`, `Alerts/`
- Frontend non-components: `camelCase` — e.g., `api/`, `hooks/`, `lib/`, `types/`
- UI primitives: `frontend/src/components/ui/` (lowercase, shadcn/ui convention)

## Where to Add New Code

**New Sensor Driver (Firmware):**
- Create: `firmware/src/sensors/<sensor_name>.rs`
- Register: Add `pub mod <sensor_name>;` to `firmware/src/sensors/mod.rs`
- Integrate: Add sensor initialization and read calls to `firmware/src/scheduler.rs`
- Implement: The `Sensor` trait if no temperature compensation needed, or custom `read_with_temp()` method

**New Backend API Endpoint:**
- Handler: `backend/internal/api/handlers/<resource>.go`
- Store: `backend/internal/store/<resource>.go`
- Model: `backend/internal/models/<resource>.go`
- Route: Register in `backend/cmd/cityfarm-api/main.go` under the `/api/v1` group

**New Frontend Page:**
- Page component: `frontend/src/pages/<PageName>.tsx`
- Route: Add `<Route>` in `frontend/src/App.tsx`
- API client: `frontend/src/api/<resource>.ts`
- Query hook: `frontend/src/hooks/use<Resource>.ts`
- Types: `frontend/src/types/<resource>.ts`

**New Frontend Feature Component:**
- Create directory: `frontend/src/components/<FeatureName>/`
- Add component files: `frontend/src/components/<FeatureName>/<ComponentName>.tsx`

**New UI Primitive (shadcn/ui):**
- Place in: `frontend/src/components/ui/<component>.tsx`
- Follow shadcn/ui conventions (lowercase filenames)

**New Database Migration:**
- File: `backend/migrations/<NNN>_<description>.sql`
- Number sequentially after existing migrations

**New Actuator (Firmware):**
- Create: `firmware/src/actuators/<actuator_name>.rs`
- Register: Add `pub mod <actuator_name>;` to `firmware/src/actuators/mod.rs`

## Special Directories

**`backend/internal/`:**
- Purpose: Go convention for private packages — not importable by external modules
- Generated: No
- Committed: Yes

**`firmware/target/`:**
- Purpose: Rust build artifacts
- Generated: Yes
- Committed: No (in .gitignore)

**`frontend/node_modules/`:**
- Purpose: npm dependencies
- Generated: Yes
- Committed: No (in .gitignore)

**`frontend/dist/`:**
- Purpose: Vite production build output
- Generated: Yes
- Committed: No (in .gitignore)

**`deploy/`:**
- Purpose: Deployment configs for Docker, Ansible, Nginx, systemd
- Generated: No
- Committed: Yes (but all subdirs are placeholder .gitkeep only)

**`ml/`:**
- Purpose: Python ML pipeline for YOLO11 plant health
- Generated: No
- Committed: Yes (but only empty `__init__.py` files)

---

*Structure analysis: 2026-03-09*
