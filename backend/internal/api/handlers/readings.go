package handlers

import (
	"encoding/json"
	"log/slog"
	"net/http"
	"time"

	"github.com/NSFoxTeam/cityfarm/backend/internal/alerting"
	"github.com/NSFoxTeam/cityfarm/backend/internal/models"
	"github.com/NSFoxTeam/cityfarm/backend/internal/store"
)

type ReadingsHandler struct {
	store  *store.ReadingsStore
	logger *slog.Logger
}

func NewReadingsHandler(s *store.ReadingsStore, logger *slog.Logger) *ReadingsHandler {
	return &ReadingsHandler{store: s, logger: logger}
}

type errorResponse struct {
	Error string `json:"error"`
}

type latestReading struct {
	models.Reading
	AlertLevel alerting.AlertLevel `json:"alert_level"`
}

func (h *ReadingsHandler) PostReadings(w http.ResponseWriter, r *http.Request) {
	var req models.BatchReadingRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		h.jsonError(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if len(req.Readings) == 0 {
		h.jsonError(w, "no readings provided", http.StatusBadRequest)
		return
	}

	for i := range req.Readings {
		if err := req.Readings[i].Validate(); err != nil {
			h.jsonError(w, err.Error(), http.StatusBadRequest)
			return
		}
	}

	if err := h.store.InsertBatch(r.Context(), req.Readings); err != nil {
		h.logger.Error("failed to insert readings", "error", err)
		h.jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	// Check alerts for each reading
	for _, rd := range req.Readings {
		if level := alerting.CheckAlert(rd.SensorType, rd.Value); level != alerting.AlertNone {
			h.logger.Warn("alert triggered",
				"sensor_type", rd.SensorType,
				"value", rd.Value,
				"level", level,
			)
		}
	}

	h.jsonResponse(w, map[string]int{"inserted": len(req.Readings)}, http.StatusCreated)
}

func (h *ReadingsHandler) GetLatest(w http.ResponseWriter, r *http.Request) {
	readings, err := h.store.GetLatest(r.Context())
	if err != nil {
		h.logger.Error("failed to get latest readings", "error", err)
		h.jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	result := make([]latestReading, len(readings))
	for i, rd := range readings {
		result[i] = latestReading{
			Reading:    rd,
			AlertLevel: alerting.CheckAlert(rd.SensorType, rd.Value),
		}
	}

	h.jsonResponse(w, result, http.StatusOK)
}

func (h *ReadingsHandler) GetHistory(w http.ResponseWriter, r *http.Request) {
	sensorType := r.URL.Query().Get("sensor_type")
	if sensorType == "" {
		h.jsonError(w, "sensor_type is required", http.StatusBadRequest)
		return
	}

	fromStr := r.URL.Query().Get("from")
	toStr := r.URL.Query().Get("to")

	from, err := time.Parse(time.RFC3339, fromStr)
	if err != nil {
		h.jsonError(w, "invalid 'from' time format, use RFC3339", http.StatusBadRequest)
		return
	}

	to, err := time.Parse(time.RFC3339, toStr)
	if err != nil {
		h.jsonError(w, "invalid 'to' time format, use RFC3339", http.StatusBadRequest)
		return
	}

	readings, err := h.store.GetHistory(r.Context(), sensorType, from, to)
	if err != nil {
		h.logger.Error("failed to get history", "error", err)
		h.jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	h.jsonResponse(w, readings, http.StatusOK)
}

func (h *ReadingsHandler) jsonResponse(w http.ResponseWriter, data any, status int) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(data)
}

func (h *ReadingsHandler) jsonError(w http.ResponseWriter, msg string, status int) {
	h.jsonResponse(w, errorResponse{Error: msg}, status)
}
