package alerting

type AlertLevel string

const (
	AlertNone     AlertLevel = "normal"
	AlertWarning  AlertLevel = "warning"
	AlertCritical AlertLevel = "critical"
)

type Threshold struct {
	WarnLow  float64
	WarnHigh float64
	CritLow  float64
	CritHigh float64
}

var DefaultThresholds = map[string]Threshold{
	"ph":            {WarnLow: 5.5, WarnHigh: 6.5, CritLow: 5.0, CritHigh: 7.0},
	"tds":           {WarnLow: 400, WarnHigh: 800, CritLow: 300, CritHigh: 1000},
	"ec":            {WarnLow: 0.8, WarnHigh: 1.6, CritLow: 0.5, CritHigh: 2.0},
	"solution_temp": {WarnLow: 18, WarnHigh: 26, CritLow: 15, CritHigh: 30},
	"temperature":   {WarnLow: 15, WarnHigh: 35, CritLow: 10, CritHigh: 40},
	"humidity":      {WarnLow: 30, WarnHigh: 80, CritLow: 20, CritHigh: 90},
	"moisture":      {WarnLow: 20, WarnHigh: 80, CritLow: 10, CritHigh: 90},
}

func CheckAlert(sensorType string, value float64) AlertLevel {
	t, ok := DefaultThresholds[sensorType]
	if !ok {
		return AlertNone
	}

	if value < t.CritLow || value > t.CritHigh {
		return AlertCritical
	}
	if value < t.WarnLow || value > t.WarnHigh {
		return AlertWarning
	}

	return AlertNone
}
