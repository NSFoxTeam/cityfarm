# CityFarm — Hydroponic IoT Platform

## Project Structure (monorepo)

| Directory | Language | Description |
|-----------|----------|-------------|
| `firmware/` | Go | RPi 5 edge agent — sensors, camera, pump control |
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
- **YL-69**: soil moisture (analog, through ADS1115)
- **Relay**: GPIO17 → water pump
- **Camera**: Xiaomi Mijia 360 PTZ 2K (RTSP)

## Tech Stack

- **Backend**: Go + Chi router + pgx + TimescaleDB
- **Frontend**: React 19 + Vite + TanStack Query + Recharts + Tailwind + shadcn/ui
- **ML**: Ultralytics YOLO11 + FastAPI + ONNX Runtime
- **Firmware**: Go + periph.io + go-dht

## Sub-agents

Route tasks to the appropriate sub-agent:
- `firmware/` → `rpi-engineer`
- `backend/` → `backend-developer`
- `frontend/` → `frontend-developer`
- `ml/` → `yolo-developer`

## Common Commands

```bash
# Firmware (cross-compile for RPi)
cd firmware && GOOS=linux GOARCH=arm64 go build -o cityfarm-agent ./cmd/cityfarm-agent/
scp cityfarm-agent rpi:/opt/cityfarm/

# Backend
cd backend && go run ./cmd/cityfarm-api/

# Frontend
cd frontend && npm run dev

# ML inference server
cd ml && uvicorn src.inference.server:app --port 8000

# Deploy (VPS)
cd deploy/docker && docker compose up -d
```
