package models

import (
	"fmt"
	"time"
)

const (
	SensorTemperature   = "temperature"
	SensorHumidity      = "humidity"
	SensorLight         = "light"
	SensorMoisture      = "moisture"
	SensorPH            = "ph"
	SensorTDS           = "tds"
	SensorEC            = "ec"
	SensorSolutionTemp  = "solution_temp"
)

var validSensorTypes = map[string]struct{}{
	SensorTemperature:  {},
	SensorHumidity:     {},
	SensorLight:        {},
	SensorMoisture:     {},
	SensorPH:           {},
	SensorTDS:          {},
	SensorEC:           {},
	SensorSolutionTemp: {},
}

type sensorRange struct {
	Min  float64
	Max  float64
	Unit string
}

var sensorRanges = map[string]sensorRange{
	SensorTemperature:  {-40, 80, "°C"},
	SensorHumidity:     {0, 100, "%"},
	SensorLight:        {0, 65535, "lux"},
	SensorMoisture:     {0, 100, "%"},
	SensorPH:           {0, 14, ""},
	SensorTDS:          {0, 2000, "ppm"},
	SensorEC:           {0, 5, "mS/cm"},
	SensorSolutionTemp: {-10, 85, "°C"},
}

type Reading struct {
	Time       time.Time `json:"time"`
	SensorType string    `json:"sensor_type"`
	Value      float64   `json:"value"`
	Unit       string    `json:"unit"`
	Level      int16     `json:"level"`
}

type BatchReadingRequest struct {
	Readings []Reading `json:"readings"`
}

func (r *Reading) Validate() error {
	if _, ok := validSensorTypes[r.SensorType]; !ok {
		return fmt.Errorf("models.reading.validate: unknown sensor_type %q", r.SensorType)
	}

	sr, ok := sensorRanges[r.SensorType]
	if !ok {
		return fmt.Errorf("models.reading.validate: no range defined for %q", r.SensorType)
	}

	if r.Value < sr.Min || r.Value > sr.Max {
		return fmt.Errorf("models.reading.validate: %s value %.2f out of range [%.2f, %.2f]",
			r.SensorType, r.Value, sr.Min, sr.Max)
	}

	if r.Unit == "" {
		r.Unit = sr.Unit
	}

	if r.Time.IsZero() {
		r.Time = time.Now().UTC()
	}

	return nil
}
