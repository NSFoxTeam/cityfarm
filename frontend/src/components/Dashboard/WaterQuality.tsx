import { Droplets, Beaker, Zap, Thermometer } from 'lucide-react';
import { SensorCard } from './SensorCard';
import type { Reading } from '@/types/sensor';

interface WaterQualityProps {
  readings: Reading[];
  loading?: boolean;
}

const sensors = [
  { type: 'ph' as const, title: 'pH', unit: 'pH', icon: Droplets, color: '#a855f7' },
  { type: 'tds' as const, title: 'TDS', unit: 'ppm', icon: Beaker, color: '#06b6d4' },
  { type: 'ec' as const, title: 'EC', unit: 'mS/cm', icon: Zap, color: '#14b8a6' },
  { type: 'solution_temp' as const, title: 'Solution Temp', unit: '°C', icon: Thermometer, color: '#f43f5e' },
] as const;

export function WaterQuality({ readings, loading }: WaterQualityProps) {
  function findReading(type: string) {
    return readings.find((r) => r.sensor_type === type);
  }

  return (
    <section>
      <h2 className="mb-4 text-lg font-semibold">Water Quality</h2>
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
