import { useQuery } from '@tanstack/react-query';
import { fetchLatestReadings, fetchReadingHistory } from '@/api/readings';
import type { SensorType } from '@/types/sensor';

export function useLatestReadings() {
  return useQuery({
    queryKey: ['readings', 'latest'],
    queryFn: fetchLatestReadings,
    refetchInterval: 10_000,
  });
}

export function useReadingHistory(
  sensorType: SensorType,
  from: string,
  to: string,
) {
  return useQuery({
    queryKey: ['readings', 'history', sensorType, from, to],
    queryFn: () => fetchReadingHistory(sensorType, from, to),
    enabled: !!from && !!to,
  });
}
