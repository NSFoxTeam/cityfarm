# Codebase Concerns

**Analysis Date:** 2026-03-09

## Tech Debt

**No Tests Anywhere:**
- Issue: Zero test files across all four modules (firmware, backend, frontend, ml). No unit tests, integration tests, or E2E tests exist.
- Files: Entire codebase
- Impact: Any refactoring or feature addition risks silent regressions. Sensor calibration math, alert thresholds, API validation, and data flow are all unverified.
- Fix approach: Prioritize tests for critical paths: (1) `backend/internal/store/readings.go` and `backend/internal/api/handlers/readings.go` — table-driven Go tests; (2) `firmware/src/sensors/ph.rs` and `firmware/src/calibration.rs` — unit tests for calibration math; (3) `frontend/src/hooks/useReadings.ts` — mock API tests with vitest.

**ML Module is Empty Skeleton:**
- Issue: All four ML subdirectories contain only empty `__init__.py` files. The `CLAUDE.md` references `uvicorn src.inference.server:app` but no server exists.
- Files: `ml/src/inference/__init__.py`, `ml/src/training/__init__.py`, `ml/src/dataset/__init__.py`, `ml/src/export/__init__.py`
- Impact: ML inference commands documented in `CLAUDE.md` will fail. Camera integration is non-functional.
- Fix approach: Either implement the YOLO11 inference pipeline or remove ML references from documentation until ready.

**Deploy Directory is Empty Stubs:**
- Issue: All deploy directories contain only `.gitkeep` files — no actual Docker Compose, Ansible playbooks, systemd units, or nginx configs.
- Files: `deploy/docker/.gitkeep`, `deploy/systemd/.gitkeep`, `deploy/ansible/playbooks/.gitkeep`, `deploy/nginx/.gitkeep`
- Impact: No reproducible deployment. Production setup is manual and undocumented.
- Fix approach: Create Docker Compose for backend + TimescaleDB, systemd unit for firmware agent, and nginx reverse proxy config.

**Empty Backend Modules (websocket, ml, storage):**
- Issue: Three backend internal packages contain only `.gitkeep` — placeholder directories with no implementation.
- Files: `backend/internal/websocket/.gitkeep`, `backend/internal/ml/.gitkeep`, `backend/internal/storage/.gitkeep`
- Impact: No real-time WebSocket push to frontend (currently polling every 10s). No ML integration from backend. File storage not implemented.
- Fix approach: Implement WebSocket hub in `backend/internal/websocket/` to push readings on POST, reducing frontend polling load.

**No Database Migration Runner:**
- Issue: SQL migration exists at `backend/migrations/001_create_sensor_readings.sql` but there is no migration runner or tooling. Migration must be applied manually.
- Files: `backend/migrations/001_create_sensor_readings.sql`, `backend/cmd/cityfarm-api/main.go`
- Impact: Schema changes require manual intervention. Risk of schema drift between environments.
- Fix approach: Integrate `golang-migrate` or `goose` and auto-run on startup.

**DHT22 Blocking Thread Sleep in Async Context:**
- Issue: `Dht22::read_raw()` uses `std::thread::sleep` for GPIO timing inside what is ultimately called from an async context. The single-wire protocol bit-banging is inherently blocking.
- Files: `firmware/src/sensors/dht22.rs` (lines 43-47)
- Impact: Blocks the tokio runtime thread during each DHT22 read (~6-8ms per attempt, up to 3 retries with 2s waits). With a 10s read interval this is manageable, but could cause timing issues if more async tasks are added.
- Fix approach: Use `tokio::task::spawn_blocking()` to wrap the `read_raw()` call, keeping the async runtime free. The current retry logic already handles this partially but the blocking call itself should be isolated.

**Calibration Reads pH Values Instead of Raw Voltages:**
- Issue: The calibration routine in `firmware/src/main.rs` first reads pH values (line 111) then opens a second I2C bus to read raw voltages (lines 133-136). The first set of samples is collected but not actually used for calibration.
- Files: `firmware/src/main.rs` (lines 89-201)
- Impact: Confusing UX — user sees pH readings, then is asked to press Enter again for the actual voltage readings used in calibration. Wastes time and creates redundant I2C bus instances.
- Fix approach: Remove the first round of pH readings. Read raw ADC voltages directly from the start using a single I2C bus/ADC instance.

## Security Considerations

**Hardcoded Default Credentials:**
- Risk: Backend falls back to `"dev-api-key"` when `API_KEY` env var is not set, and default database URL includes `cityfarm:cityfarm` credentials.
- Files: `backend/cmd/cityfarm-api/main.go` (lines 26-34)
- Current mitigation: A warning log is emitted for missing API_KEY.
- Recommendations: Refuse to start in production mode without `API_KEY` set. Use an environment check (e.g., `ENV=production`) that mandates all secrets.

**Frontend API Calls Have No Authentication:**
- Risk: Frontend `apiFetch()` does not send any `X-API-Key` header, but the backend requires it on all `/api/v1` routes.
- Files: `frontend/src/api/readings.ts` (lines 5-9), `backend/internal/api/middleware.go`
- Current mitigation: None — the frontend API calls will fail with 401 unless CORS or a proxy strips the auth requirement.
- Recommendations: Either add API key header to frontend requests (with the key injected via env var, not hardcoded), or create a separate unauthenticated read-only route for the dashboard, keeping POST authenticated.

**CORS Allows All Origins:**
- Risk: `AllowedOrigins: []string{"*"}` permits any domain to call the API.
- Files: `backend/cmd/cityfarm-api/main.go` (line 76)
- Current mitigation: API key auth protects write endpoints.
- Recommendations: Restrict to the actual frontend origin in production.

**No Rate Limiting:**
- Risk: The POST `/api/v1/readings` endpoint has no rate limiting. A compromised or malfunctioning firmware agent could flood the database.
- Files: `backend/cmd/cityfarm-api/main.go` (API route setup)
- Current mitigation: None.
- Recommendations: Add chi middleware rate limiter or per-IP/per-key rate limiting.

**SQLite Buffer on Firmware Has No Size Limit:**
- Risk: If backend is unreachable for extended periods, the SQLite buffer at `/opt/cityfarm/buffer.db` grows unbounded.
- Files: `firmware/src/transport/buffer.rs`
- Current mitigation: None — readings accumulate indefinitely.
- Recommendations: Add a max buffer size (e.g., 10,000 entries) with FIFO eviction, or a max age policy.

## Performance Bottlenecks

**GetLatest Query Scans Full Hypertable:**
- Problem: `DISTINCT ON (sensor_type, level)` with `ORDER BY sensor_type, level, time DESC` on TimescaleDB may result in a full table scan as data grows.
- Files: `backend/internal/store/readings.go` (lines 44-49)
- Cause: No `WHERE` clause limits the time range, so the query must examine all chunks.
- Improvement path: Add a time bound (e.g., `WHERE time > NOW() - INTERVAL '1 hour'`) or maintain a `latest_readings` materialized view / summary table.

**GetHistory Returns Unbounded Results:**
- Problem: History query has no `LIMIT` clause. A large time range could return millions of rows.
- Files: `backend/internal/store/readings.go` (lines 69-78), `backend/internal/api/handlers/readings.go` (lines 90-119)
- Cause: No pagination or aggregation.
- Improvement path: Add `LIMIT` parameter, or use TimescaleDB `time_bucket()` for downsampled aggregates over large ranges.

**Firmware Sends One Batch Per Buffer Entry:**
- Problem: `flush_buffer()` pops up to 100 entries but sends each one individually via HTTP, waiting for each to complete before the next.
- Files: `firmware/src/scheduler.rs` (lines 143-182)
- Cause: Sequential HTTP calls in the flush loop.
- Improvement path: Combine multiple buffered payloads into a single HTTP request, or send them concurrently with `tokio::join!`.

## Fragile Areas

**DHT22 Single-Wire Protocol Bit-Banging:**
- Files: `firmware/src/sensors/dht22.rs`
- Why fragile: Microsecond-level timing on a non-RTOS Linux system. Kernel scheduling jitter can cause checksum failures and timeouts. The busy-loop in `wait_for_level()` is CPU-intensive and timing-sensitive.
- Safe modification: Do not change timing constants without testing on actual hardware. The 3-retry mechanism is essential.
- Test coverage: None.

**I2C Bus Sharing via Arc<Mutex>:**
- Files: `firmware/src/sensors/ads1115.rs`, `firmware/src/scheduler.rs`
- Why fragile: All I2C sensors (BH1750 on channels 0-2, ADS1115 on channel 7) share a single `Arc<Mutex<I2cdev>>`. The TCA9548A mux channel is set per-read but not atomically verified — if a read fails mid-operation, the mux may be left on the wrong channel.
- Safe modification: Ensure all I2C operations within a single `bus.lock()` scope include both mux selection and device communication. Currently this is correct in `ads1115.rs` but must be maintained for any new I2C sensors.
- Test coverage: None.

**Relay Safety Timer Not Called:**
- Files: `firmware/src/actuators/relay.rs`, `firmware/src/scheduler.rs`
- Why fragile: `Relay::check_safety()` exists to enforce max pump duration, but the scheduler loop never instantiates or calls a Relay. The relay module is fully implemented but unused — there is no pump control in the main loop.
- Safe modification: When integrating relay control, `check_safety()` must be called every tick cycle. The `Drop` implementation ensures relay-off on panic, which is good.
- Test coverage: None.

## Scaling Limits

**Single Firmware Agent, Single Backend Instance:**
- Current capacity: One RPi reading ~8 sensor values every 10 seconds, one API server.
- Limit: Architecture assumes single-node. No device ID in readings — multi-device would require schema changes.
- Scaling path: Add `device_id` column to `sensor_readings` table, include in firmware payload, and add API filtering.

**TimescaleDB Without Retention Policy:**
- Current capacity: Unlimited data retention by default.
- Limit: Disk will fill over time. At ~8 readings per 10s = ~69,120 rows/day.
- Scaling path: Add TimescaleDB retention policy: `SELECT add_retention_policy('sensor_readings', INTERVAL '90 days');`

## Dependencies at Risk

**frontend/src/types/sensor.ts ReadingHistory Type Mismatch:**
- Risk: Frontend defines `ReadingHistory` as `{ sensor_type, readings: { time, value }[] }` but backend returns a flat array of `Reading` objects from `GetHistory`. The types do not match.
- Files: `frontend/src/types/sensor.ts` (lines 22-25), `backend/internal/api/handlers/readings.go` (line 119)
- Impact: `useReadingHistory` hook will receive data in unexpected format, likely causing runtime errors or empty charts.
- Migration plan: Either update backend to wrap history results in the expected structure, or update frontend type to match the flat array response.

## Missing Critical Features

**No Pump Control Loop:**
- Problem: Relay module is implemented but not integrated into the scheduler. There is no automated pump activation based on sensor thresholds.
- Blocks: Automated hydroponics operation — the core use case.

**No WebSocket Real-Time Updates:**
- Problem: Frontend polls every 10 seconds. Backend has an empty `websocket/` package.
- Blocks: Responsive dashboard, instant alert notification.

**No Alert Notifications:**
- Problem: Alerts are logged server-side but never delivered to the user (no push notifications, email, or Telegram).
- Blocks: Unattended operation — user won't know about critical conditions unless watching the dashboard.

**No Camera Integration:**
- Problem: RTSP camera is documented in hardware specs but no code exists for camera streaming or snapshots.
- Blocks: Visual plant monitoring and ML-based health detection.

## Test Coverage Gaps

**Entire Codebase (0% Coverage):**
- What's not tested: Everything. No test files exist in any module.
- Files: All source files across `firmware/src/`, `backend/internal/`, `frontend/src/`
- Risk: Calibration math errors, API validation bypasses, query bugs, and data format mismatches could go unnoticed.
- Priority: High — start with backend handlers and firmware calibration logic.

---

*Concerns audit: 2026-03-09*
