package store

import (
	"context"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/NSFoxTeam/cityfarm/backend/internal/models"
)

type ReadingsStore struct {
	pool *pgxpool.Pool
}

func NewReadingsStore(pool *pgxpool.Pool) *ReadingsStore {
	return &ReadingsStore{pool: pool}
}

func (s *ReadingsStore) InsertBatch(ctx context.Context, readings []models.Reading) error {
	batch := &pgx.Batch{}
	for _, r := range readings {
		batch.Queue(
			"INSERT INTO sensor_readings (time, sensor_type, value, unit, level) VALUES ($1, $2, $3, $4, $5)",
			r.Time, r.SensorType, r.Value, r.Unit, r.Level,
		)
	}

	br := s.pool.SendBatch(ctx, batch)
	defer br.Close()

	for range readings {
		if _, err := br.Exec(); err != nil {
			return fmt.Errorf("store.readings.insert_batch: %w", err)
		}
	}

	return nil
}

func (s *ReadingsStore) GetLatest(ctx context.Context) ([]models.Reading, error) {
	query := `
		SELECT DISTINCT ON (sensor_type, level)
			time, sensor_type, value, unit, level
		FROM sensor_readings
		ORDER BY sensor_type, level, time DESC
	`

	rows, err := s.pool.Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("store.readings.get_latest: %w", err)
	}
	defer rows.Close()

	var readings []models.Reading
	for rows.Next() {
		var r models.Reading
		if err := rows.Scan(&r.Time, &r.SensorType, &r.Value, &r.Unit, &r.Level); err != nil {
			return nil, fmt.Errorf("store.readings.get_latest.scan: %w", err)
		}
		readings = append(readings, r)
	}

	return readings, rows.Err()
}

func (s *ReadingsStore) GetHistory(ctx context.Context, sensorType string, from, to time.Time) ([]models.Reading, error) {
	query := `
		SELECT time, sensor_type, value, unit, level
		FROM sensor_readings
		WHERE sensor_type = $1
		  AND time >= $2
		  AND time <= $3
		ORDER BY time ASC
	`

	rows, err := s.pool.Query(ctx, query, sensorType, from, to)
	if err != nil {
		return nil, fmt.Errorf("store.readings.get_history: %w", err)
	}
	defer rows.Close()

	var readings []models.Reading
	for rows.Next() {
		var r models.Reading
		if err := rows.Scan(&r.Time, &r.SensorType, &r.Value, &r.Unit, &r.Level); err != nil {
			return nil, fmt.Errorf("store.readings.get_history.scan: %w", err)
		}
		readings = append(readings, r)
	}

	return readings, rows.Err()
}
