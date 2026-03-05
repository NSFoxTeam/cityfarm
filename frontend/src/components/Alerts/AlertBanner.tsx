import { useState } from 'react';
import { AlertTriangle, X } from 'lucide-react';
import type { Reading } from '@/types/sensor';

interface AlertBannerProps {
  readings: Reading[];
}

const sensorLabels: Record<string, string> = {
  temperature: 'Temperature',
  humidity: 'Humidity',
  light: 'Light',
  moisture: 'Soil Moisture',
  ph: 'pH',
  tds: 'TDS',
  ec: 'EC',
  solution_temp: 'Solution Temp',
};

export function AlertBanner({ readings }: AlertBannerProps) {
  const [dismissed, setDismissed] = useState<Set<string>>(new Set());

  const alerts = readings.filter(
    (r) =>
      r.alert_level &&
      r.alert_level !== 'normal' &&
      !dismissed.has(r.sensor_type),
  );

  if (alerts.length === 0) return null;

  function dismiss(sensorType: string) {
    setDismissed((prev) => new Set(prev).add(sensorType));
  }

  return (
    <div className="flex flex-col gap-2">
      {alerts.map((alert) => (
        <div
          key={alert.sensor_type}
          className={`flex items-center justify-between rounded-lg px-4 py-3 ${
            alert.alert_level === 'critical'
              ? 'bg-red-500/10 border border-red-500/30 text-red-400'
              : 'bg-amber-500/10 border border-amber-500/30 text-amber-400'
          }`}
        >
          <div className="flex items-center gap-3">
            <AlertTriangle className="h-5 w-5 shrink-0" />
            <span className="text-sm font-medium">
              {sensorLabels[alert.sensor_type] ?? alert.sensor_type}:{' '}
              {alert.value.toFixed(1)} {alert.unit} — {alert.alert_level}
            </span>
          </div>
          <button
            onClick={() => dismiss(alert.sensor_type)}
            className="rounded p-1 hover:bg-white/10"
          >
            <X className="h-4 w-4" />
          </button>
        </div>
      ))}
    </div>
  );
}
