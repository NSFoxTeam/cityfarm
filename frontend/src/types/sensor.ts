export type SensorType =
  | 'temperature'
  | 'humidity'
  | 'light'
  | 'moisture'
  | 'ph'
  | 'tds'
  | 'ec'
  | 'solution_temp';

export type AlertLevel = 'normal' | 'warning' | 'critical';

export interface Reading {
  sensor_type: SensorType;
  value: number;
  unit: string;
  level: number;
  time: string;
  alert_level?: AlertLevel;
}

export interface ReadingHistory {
  sensor_type: SensorType;
  readings: { time: string; value: number }[];
}
