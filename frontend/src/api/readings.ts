import type { Reading, ReadingHistory, SensorType } from '@/types/sensor';

const API_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:8080/api/v1';

async function apiFetch<T>(path: string): Promise<T> {
  const res = await fetch(`${API_URL}${path}`);
  if (!res.ok) throw new Error(`API error: ${res.status} ${res.statusText}`);
  return res.json() as Promise<T>;
}

export function fetchLatestReadings(): Promise<Reading[]> {
  return apiFetch('/readings/latest');
}

export function fetchReadingHistory(
  sensorType: SensorType,
  from: string,
  to: string,
): Promise<ReadingHistory> {
  return apiFetch(
    `/readings/history?sensor_type=${sensorType}&from=${from}&to=${to}`,
  );
}
