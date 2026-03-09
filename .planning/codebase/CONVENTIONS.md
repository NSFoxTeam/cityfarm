# Coding Conventions

**Analysis Date:** 2026-03-09

## Project-Wide Patterns

This is a polyglot monorepo with four languages. Each directory follows its language's idiomatic conventions. The shared pattern is **snake_case for data interchange** — sensor types, API fields, and database columns all use `snake_case` (e.g., `sensor_type`, `solution_temp`, `alert_level`).

## Frontend (TypeScript/React) — `frontend/`

### Naming Patterns

**Files:**
- React components: PascalCase (`SensorCard.tsx`, `WaterQualityChart.tsx`)
- Hooks: camelCase with `use` prefix (`useReadings.ts`)
- API modules: camelCase (`readings.ts`)
- Types: camelCase (`sensor.ts`)
- UI components (shadcn): lowercase (`card.tsx`, `badge.tsx`, `skeleton.tsx`)
- Utilities: camelCase (`utils.ts`)

**Functions:**
- Components: PascalCase named exports (`export function Dashboard()`, `export function SensorCard()`)
- Hooks: camelCase with `use` prefix (`useLatestReadings`, `useReadingHistory`)
- API functions: camelCase with verb prefix (`fetchLatestReadings`, `fetchReadingHistory`)
- Helpers: camelCase (`cn`)

**Variables:**
- camelCase for local variables and props
- UPPER_SNAKE_CASE not used — constants use `const` at module level with camelCase or inline objects
- Sensor config arrays use `as const` assertion for type narrowing

**Types:**
- PascalCase for interfaces and type aliases (`Reading`, `SensorType`, `AlertLevel`)
- Use `type` imports: `import type { Reading } from '@/types/sensor'`
- Props interfaces named `{ComponentName}Props` (`SensorCardProps`, `AlertBannerProps`)

### Code Style

**Formatting:**
- No Prettier configured — default Vite/ESLint formatting
- Single quotes for strings in TS/TSX
- Semicolons used
- 2-space indentation
- Trailing commas in function parameters

**Linting:**
- ESLint 9 flat config at `frontend/eslint.config.js`
- Extends: `@eslint/js` recommended, `typescript-eslint` recommended, `react-hooks`, `react-refresh`
- TypeScript strict mode enabled in `frontend/tsconfig.app.json`
- `noUnusedLocals: true`, `noUnusedParameters: true`
- `verbatimModuleSyntax: true` — requires explicit `type` keyword for type-only imports

### Import Organization

**Order:**
1. React/external library imports (`react`, `recharts`, `date-fns`, `lucide-react`)
2. Internal absolute imports using `@/` alias (`@/components/...`, `@/hooks/...`, `@/types/...`)
3. Relative imports for sibling files (`./SensorCard`)

**Path Aliases:**
- `@/*` maps to `./src/*` (configured in `frontend/tsconfig.json` and `frontend/vite.config.ts`)

### Component Patterns

**Structure:**
- Named function exports (not default exports, except `App.tsx`)
- Props destructured in function signature
- Interface for props defined above component
- No `React.FC` — plain function declarations

**Example pattern from `frontend/src/components/Dashboard/SensorCard.tsx`:**
```tsx
interface SensorCardProps {
  title: string;
  value: number | undefined;
  // ...
}

export function SensorCard({ title, value, ...rest }: SensorCardProps) {
  // ...
}
```

**State management:**
- TanStack Query for server state (`useQuery` with `queryKey`/`queryFn`)
- React `useState` for local UI state (e.g., dismissed alerts)
- `useMemo` for derived/computed data
- No global state management library

**Data fetching pattern:**
- API functions in `frontend/src/api/readings.ts` — pure async functions returning typed data
- Custom hooks in `frontend/src/hooks/useReadings.ts` — wrap `useQuery` with domain-specific config
- Components consume hooks, never call API directly

**UI framework:**
- shadcn/ui components in `frontend/src/components/ui/` (generated, not hand-written)
- Tailwind 4 for styling — utility classes directly in JSX
- `cn()` helper from `frontend/src/lib/utils.ts` for conditional class merging (clsx + tailwind-merge)
- Lucide icons for all iconography

### Error Handling

**API layer:** Throws on non-OK responses (`throw new Error(...)`)
**Query layer:** TanStack Query handles retries (2 retries configured) and error states
**Components:** Loading states with skeleton placeholders, no explicit error boundaries observed

## Backend (Go) — `backend/`

### Naming Patterns

**Files:**
- Lowercase single-word or snake_case (`readings.go`, `middleware.go`, `thresholds.go`)
- One file per primary type/concern

**Functions:**
- PascalCase for exported (`NewReadingsHandler`, `PostReadings`, `GetLatest`)
- camelCase for unexported (`jsonResponse`, `jsonError`)
- Constructor pattern: `New{Type}(deps...) *Type`

**Variables:**
- camelCase for locals
- PascalCase for exported constants (`SensorTemperature`, `AlertWarning`)

**Types:**
- PascalCase structs (`ReadingsHandler`, `Reading`, `ReadingsStore`)
- unexported helper types: camelCase (`errorResponse`, `latestReading`, `sensorRange`)

### Package Organization

**Layout follows Go standard project layout:**
- `backend/cmd/cityfarm-api/main.go` — entry point, wiring
- `backend/internal/` — private application code
  - `api/` — HTTP middleware
  - `api/handlers/` — request handlers
  - `models/` — domain types and validation
  - `store/` — database access (repository pattern)
  - `alerting/` — business logic for thresholds

### Import Organization

**Order:**
1. Standard library (`context`, `encoding/json`, `log/slog`, `net/http`)
2. Third-party (`github.com/go-chi/...`, `github.com/jackc/pgx/...`)
3. Internal (`github.com/NSFoxTeam/cityfarm/backend/internal/...`)

Groups separated by blank lines. Use `goimports` ordering.

### Error Handling

**Pattern:** Return errors with context prefix using `fmt.Errorf("store.readings.insert_batch: %w", err)`
- Error prefix format: `package.type.method`
- Wrap with `%w` for error chain preservation
- In handlers: log error + return generic JSON error to client
- In main: log error + `os.Exit(1)` for fatal startup errors

**Validation:** Model-level validation in `backend/internal/models/reading.go` — `Validate()` method on struct

### Logging

**Framework:** `log/slog` (stdlib structured logging)
- JSON handler in production: `slog.NewJSONHandler(os.Stdout, ...)`
- Structured key-value pairs: `logger.Error("msg", "error", err, "key", value)`
- Logger injected via constructor into handlers

### JSON Serialization

- Struct tags with `json:"snake_case"` format
- Consistent with frontend type names

## Firmware (Rust) — `firmware/`

### Naming Patterns

**Files:**
- Lowercase snake_case (`ads1115.rs`, `ph.rs`, `ds18b20.rs`)
- `mod.rs` for module roots

**Functions:**
- snake_case (`read_with_temp`, `check_safety`, `send_readings`)
- Constructor pattern: `new(deps...) -> Result<Self>` or `new(deps...) -> Self`

**Types:**
- PascalCase for structs and enums (`PhSensor`, `SharedAds1115`, `SensorType`)
- Variants: PascalCase (`SensorType::SolutionTemp`)

**Constants:**
- UPPER_SNAKE_CASE (`ADC_CHANNEL`, `MAX_RETRIES`, `INITIAL_BACKOFF`)

### Module Organization

- `firmware/src/main.rs` — CLI parsing, command dispatch
- `firmware/src/sensors/` — sensor drivers (one file per sensor)
- `firmware/src/actuators/` — relay/pump control
- `firmware/src/transport/` — HTTP client + SQLite buffer
- `firmware/src/config.rs` — TOML config deserialization
- `firmware/src/calibration.rs` — pH calibration data
- `firmware/src/scheduler.rs` — main read loop

### Error Handling

**Framework:** `anyhow` for application errors
- Functions return `anyhow::Result<T>`
- Use `anyhow::bail!()` for early returns with context
- `?` operator for propagation
- No custom error types — `anyhow` throughout

**Resilience pattern:**
- Sensor init failures: log warning, set to `None`, skip in loop
- Sensor read failures: log warning, continue to next sensor
- HTTP failures: retry with exponential backoff (3 attempts), 4xx errors fail immediately
- SQLite buffer: store-and-forward pattern for network resilience

### Logging

**Framework:** `tracing` + `tracing-subscriber` with `env-filter`
- Macros: `info!()`, `warn!()`, `error!()`, `debug!()`
- Default level: `info`, overridable via `RUST_LOG` env var

### Shared Resource Pattern

- I2C bus: `Arc<Mutex<I2cdev>>` — shared across sensors
- ADS1115 ADC: `Arc<SharedAds1115>` — wraps bus mutex internally
- Async trait: `#[async_trait]` on `Sensor` trait for dynamic dispatch

### Serialization

- `serde` derive macros: `#[derive(Serialize, Deserialize)]`
- `#[serde(rename_all = "snake_case")]` on enums for API compatibility
- Config: TOML format with serde defaults

## ML (Python) — `ml/`

### Naming Patterns

- Package structure: `ml/src/{module}/__init__.py`
- Modules: `dataset`, `training`, `export`, `inference`
- Linter: `ruff` (configured as dev dependency in `ml/pyproject.toml`)

### Style

- Python 3.10+ required
- `pyproject.toml` for project config (PEP 621)
- No source files with logic found yet — module stubs only (`__init__.py`)

## Cross-Language Conventions

### API Contract

- All sensor data uses snake_case field names across all layers
- Sensor types enum is consistent: `temperature`, `humidity`, `light`, `moisture`, `ph`, `tds`, `ec`, `solution_temp`
- Time format: RFC3339/ISO8601 strings
- API versioning: `/api/v1/` prefix
- Auth: `X-API-Key` header

### Environment Configuration

- Backend: env vars with fallback defaults (`DATABASE_URL`, `API_KEY`, `LISTEN_ADDR`)
- Frontend: Vite env vars with `VITE_` prefix (`VITE_API_URL`)
- Firmware: TOML config file at `/opt/cityfarm/config.toml`

---

*Convention analysis: 2026-03-09*
