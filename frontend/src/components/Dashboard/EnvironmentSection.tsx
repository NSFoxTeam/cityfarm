import { Thermometer, Droplets, Sun, Sprout } from 'lucide-react';
import { SensorCard } from './SensorCard';
import type { Reading } from '@/types/sensor';

interface EnvironmentSectionProps {
  readings: Reading[];
  loading?: boolean;
}

const sensors = [
  { type: 'temperature' as const, title: 'Temperature', unit: '°C', icon: Thermometer, color: '#f97316' },
  { type: 'humidity' as const, title: 'Humidity', unit: '%', icon: Droplets, color: '#3b82f6' },
  { type: 'light' as const, title: 'Light', unit: 'lux', icon: Sun, color: '#eab308' },
  { type: 'moisture' as const, title: 'Soil Moisture', unit: '%', icon: Sprout, color: '#22c55e' },
] as const;

export function EnvironmentSection({ readings, loading }: EnvironmentSectionProps) {
  function findReading(type: string) {
    return readings.find((r) => r.sensor_type === type);
  }

  return (
    <section>
      <h2 className="mb-4 text-lg font-semibold">Environment</h2>
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {sensors.map((s) => {
          const reading = findReading(s.type);
          return (
            <SensorCard
              key={s.type}
              title={s.title}
              value={reading?.value}
              unit={s.unit}
              icon={s.icon}
              color={s.color}
              alertLevel={reading?.alert_level}
              loading={loading}
            />
          );
        })}
      </div>
    </section>
  );
}
