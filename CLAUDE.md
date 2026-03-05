# CityFarm — Hydroponic IoT Platform

## Project Structure (monorepo)

| Directory | Language | Description |
|-----------|----------|-------------|
| `firmware/` | Rust | RPi 5 edge agent — sensors, camera, pump control |
| `backend/` | Go | API server on VPS — REST, WebSocket, TimescaleDB |
| `frontend/` | TypeScript/React | Dashboard — charts, camera, alerts |
| `ml/` | Python | YOLO11 pipeline — dataset, training, inference |
| `deploy/` | Docker/Ansible | Deployment configs for RPi and VPS |
| `docs/` | Markdown | Architecture, wiring, API docs |

## Hardware (Raspberry Pi 5)

- **SSH**: `ssh rpi` (192.168.1.175, user nsfox)
- **DHT22**: GPIO4 (temp+humidity, single-wire, NOT I2C)
- **TCA9548A**: I2C mux at 0x70 (bus 1)
- **BH1750**: light sensor at 0x23 via TCA9548A ch0-2
- **ADS1115**: 16-bit ADC at 0x48 via TCA9548A ch7
- **YL-69**: soil moisture (analog, through ADS1115 A0)
- **DFRobot pH V2**: analog pH sensor (through ADS1115 A1)
- **DFRobot TDS**: analog TDS/EC sensor (through ADS1115 A2)
- **DS18B20**: GPIO22 (1-Wire, solution temperature)
- **Relay**: GPIO17 → water pump (active LOW)
- **Camera**: Xiaomi Mijia 360 PTZ 2K (RTSP)

## Tech Stack

- **Backend**: Go + Chi router + pgx + TimescaleDB
- **Frontend**: React 19 + Vite + TanStack Query + Recharts + Tailwind + shadcn/ui
- **ML**: Ultralytics YOLO11 + FastAPI + ONNX Runtime
- **Firmware**: Rust + rppal + ads1x1x + tokio + linux-embedded-hal

## Sub-agents

Route tasks to the appropriate sub-agent:
- `firmware/` → `rpi-engineer`
- `backend/` → `backend-developer`
- `frontend/` → `frontend-developer`
- `ml/` → `yolo-developer`

## Common Commands

```bash
# Firmware (cross-compile for RPi)
cd firmware && cross build --release --target aarch64-unknown-linux-gnu
scp target/aarch64-unknown-linux-gnu/release/cityfarm-agent rpi:/opt/cityfarm/

# Backend
cd backend && go run ./cmd/cityfarm-api/

# Frontend
cd frontend && npm run dev

# ML inference server
cd ml && uvicorn src.inference.server:app --port 8000

# Deploy (VPS)
cd deploy/docker && docker compose up -d
```
