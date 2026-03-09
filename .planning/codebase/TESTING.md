# Testing Patterns

**Analysis Date:** 2026-03-09

## Current State

**No tests exist in the codebase.** No test files, no test configuration, and no test commands are defined across any of the four project directories. This section documents the available tooling and recommended patterns for each language.

## Test Framework Availability

### Frontend (TypeScript/React) — `frontend/`

**Runner:** Not configured

**Available tooling:**
- No `vitest`, `jest`, or other test runner in `frontend/package.json`
- No test-related scripts in `frontend/package.json`
- ESLint is configured but covers linting only

**Recommended setup:**
- Vitest (natural fit with Vite) — add to devDependencies
- React Testing Library for component tests
- MSW (Mock Service Worker) for API mocking

**Recommended config file:** `frontend/vitest.config.ts`

**Recommended test file locations:**
- Co-located: `frontend/src/components/Dashboard/SensorCard.test.tsx`
- Or dedicated: `frontend/src/__tests__/`

**Recommended naming:** `{component}.test.tsx`, `{module}.test.ts`

**Run commands to add to `frontend/package.json`:**
```bash
npm run test               # Run all tests
npm run test -- --watch    # Watch mode
npm run test -- --coverage # Coverage
```

### Backend (Go) — `backend/`

**Runner:** Go built-in `go test`

**Available tooling:**
- Go's standard testing package is always available
- No `_test.go` files exist in the codebase
- pgx has test utilities for database mocking

**Recommended test file locations:**
- Co-located per Go convention: `backend/internal/store/readings_test.go`
- Table-driven tests for handlers: `backend/internal/api/handlers/readings_test.go`
- Model validation: `backend/internal/models/reading_test.go`

**Recommended naming:** `{file}_test.go` (Go standard)

**Run commands:**
```bash
cd backend && go test ./...           # Run all tests
cd backend && go test -v ./...        # Verbose
cd backend && go test -cover ./...    # Coverage
cd backend && go test -race ./...     # Race detector
```

**Recommended patterns:**

**Table-driven tests for model validation:**
```go
func TestReading_Validate(t *testing.T) {
    tests := []struct {
        name    string
        reading models.Reading
        wantErr bool
    }{
        {
            name:    "valid temperature reading",
            reading: models.Reading{SensorType: "temperature", Value: 25.0, Unit: "°C"},
            wantErr: false,
        },
        {
            name:    "invalid sensor type",
            reading: models.Reading{SensorType: "unknown", Value: 1.0},
            wantErr: true,
        },
        {
            name:    "out of range value",
            reading: models.Reading{SensorType: "ph", Value: 15.0},
            wantErr: true,
        },
    }
    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := tt.reading.Validate()
            if (err != nil) != tt.wantErr {
                t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
            }
        })
    }
}
```

**Handler tests with httptest:**
```go
func TestReadingsHandler_GetLatest(t *testing.T) {
    // Use httptest.NewRecorder() and httptest.NewRequest()
    // Mock store with interface
}
```

**Key observation:** The store layer uses concrete `*store.ReadingsStore` not an interface, making handler testing harder. Extract an interface for testability:
```go
type ReadingsRepository interface {
    InsertBatch(ctx context.Context, readings []models.Reading) error
    GetLatest(ctx context.Context) ([]models.Reading, error)
    GetHistory(ctx context.Context, sensorType string, from, to time.Time) ([]models.Reading, error)
}
```

### Firmware (Rust) — `firmware/`

**Runner:** `cargo test` (built-in)

**Available tooling:**
- Rust's built-in test framework
- No `#[cfg(test)]` modules or test files exist
- Hardware-dependent code makes unit testing challenging

**Constraints:**
- Most code depends on I2C hardware (`rppal`, `linux-embedded-hal`)
- Cross-compilation target (`aarch64-unknown-linux-gnu`) — tests must run on host or RPi
- `cargo test` will fail on macOS without mocking hardware

**Recommended approach:**
- Unit test pure logic: calibration math, threshold checking, buffer operations
- Mock hardware traits for sensor testing
- Integration tests only on RPi hardware

**Testable modules (no hardware dependency):**
- `firmware/src/calibration.rs` — `CalibrationData::from_two_point()` is pure math
- `firmware/src/transport/buffer.rs` — SQLite buffer (use temp file)
- `firmware/src/config.rs` — config parsing (use temp TOML files)

**Example for calibration:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_point_calibration() {
        let cal = CalibrationData::from_two_point(1.5, 2.0);
        // pH 7.0 at 1.5V, pH 4.0 at 2.0V
        // slope = (4.0 - 7.0) / (2.0 - 1.5) = -6.0
        assert!((cal.slope - (-6.0)).abs() < 0.01);
        // offset = 7.0 - (-6.0) * 1.5 = 16.0
        assert!((cal.offset - 16.0).abs() < 0.01);
    }
}
```

**Run commands:**
```bash
# On RPi (native)
cd firmware && cargo test

# Cross-compile check only (no test execution)
cd firmware && cross build --tests --target aarch64-unknown-linux-gnu
```

### ML (Python) — `ml/`

**Runner:** pytest (listed as dev dependency in `ml/pyproject.toml`)

**Available tooling:**
- `pytest>=8.0` in `[project.optional-dependencies.dev]`
- `ruff>=0.8` for linting
- No test files exist yet (only `__init__.py` stubs)

**Recommended test file locations:**
- `ml/tests/` directory
- `ml/tests/test_inference.py`, `ml/tests/test_dataset.py`

**Run commands:**
```bash
cd ml && pip install -e ".[dev]"  # Install with dev deps
cd ml && pytest                    # Run all tests
cd ml && pytest -v                 # Verbose
cd ml && pytest --cov=src          # Coverage
```

## Mocking

### Frontend

**No mocking framework configured.** When adding tests:
- Use MSW for API mocking (intercept fetch calls)
- Use `vi.mock()` (Vitest) for module mocking
- Mock `import.meta.env.VITE_API_URL` for API base URL

### Backend

**No mocking framework.** Go patterns to follow:
- Define interfaces for store layer (currently concrete types)
- Use `httptest.NewServer()` / `httptest.NewRecorder()` for HTTP tests
- Use `pgxpool` test containers or `pgxmock` for database tests

### Firmware

**No mocking framework.** Rust patterns to follow:
- Use `mockall` crate for trait mocking (add to `[dev-dependencies]`)
- Abstract hardware behind traits (already done with `Sensor` trait)
- Use `tempfile` crate for SQLite buffer tests

## Coverage

**Requirements:** None enforced across any project
**CI/CD:** No CI pipeline configured for tests

## Test Types

**Unit Tests:** None exist. Priority areas:
1. `backend/internal/models/reading.go` — validation logic (pure, easy to test)
2. `backend/internal/alerting/thresholds.go` — threshold logic (pure, easy to test)
3. `firmware/src/calibration.rs` — calibration math (pure, easy to test)
4. `frontend/src/api/readings.ts` — API fetch functions

**Integration Tests:** None exist. Priority areas:
1. Backend API handlers with real/mock database
2. Firmware store-and-forward buffer cycle

**E2E Tests:** Not applicable at current stage

## Priority Recommendations

1. **Highest value, lowest effort:** Add tests for `backend/internal/models/reading.go` and `backend/internal/alerting/thresholds.go` — pure functions, no dependencies, protect core business logic
2. **High value:** Add tests for `firmware/src/calibration.rs` — pH calibration math is critical for accuracy
3. **Medium value:** Add Vitest + React Testing Library for frontend component tests
4. **Structural improvement:** Extract store interface in backend for handler testability

---

*Testing analysis: 2026-03-09*
