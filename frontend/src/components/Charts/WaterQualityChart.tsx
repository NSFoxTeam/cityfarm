import { useMemo } from 'react';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from 'recharts';
import { format } from 'date-fns';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useReadingHistory } from '@/hooks/useReadings';
import { subHours } from 'date-fns';

export function WaterQualityChart() {
  const now = useMemo(() => new Date().toISOString(), []);
  const from = useMemo(() => subHours(new Date(), 24).toISOString(), []);

  const phQuery = useReadingHistory('ph', from, now);
  const tdsQuery = useReadingHistory('tds', from, now);
  const ecQuery = useReadingHistory('ec', from, now);
  const solTempQuery = useReadingHistory('solution_temp', from, now);

  const data = useMemo(() => {
    const timeMap = new Map<string, Record<string, number>>();

    function addSeries(
      readings: { time: string; value: number }[] | undefined,
      key: string,
    ) {
      readings?.forEach((r) => {
        const t = r.time;
        const entry = timeMap.get(t) ?? {};
        entry[key] = r.value;
        timeMap.set(t, entry);
      });
    }

    addSeries(phQuery.data?.readings, 'ph');
    addSeries(tdsQuery.data?.readings, 'tds');
    addSeries(ecQuery.data?.readings, 'ec');
    addSeries(solTempQuery.data?.readings, 'solTemp');

    return Array.from(timeMap.entries())
      .sort(([a], [b]) => a.localeCompare(b))
      .map(([time, values]) => ({ time, ...values }));
  }, [phQuery.data, tdsQuery.data, ecQuery.data, solTempQuery.data]);

  const loading =
    phQuery.isLoading || tdsQuery.isLoading || ecQuery.isLoading || solTempQuery.isLoading;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Water Quality — 24h</CardTitle>
      </CardHeader>
      <CardContent>
        {loading ? (
          <div className="h-64 animate-pulse rounded bg-muted" />
        ) : data.length === 0 ? (
          <div className="flex h-64 items-center justify-center text-muted-foreground">
            No data available
          </div>
        ) : (
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={data}>
              <CartesianGrid strokeDasharray="3 3" stroke="var(--color-border)" />
              <XAxis
                dataKey="time"
                tickFormatter={(t: string) => format(new Date(t), 'HH:mm')}
                stroke="var(--color-muted-foreground)"
                fontSize={12}
              />
              <YAxis
                yAxisId="ph"
                domain={[0, 14]}
                stroke="#a855f7"
                fontSize={12}
                label={{ value: 'pH', angle: -90, position: 'insideLeft', fill: '#a855f7' }}
              />
              <YAxis
                yAxisId="tds"
                orientation="right"
                domain={[0, 1000]}
                stroke="#06b6d4"
                fontSize={12}
                label={{ value: 'ppm', angle: 90, position: 'insideRight', fill: '#06b6d4' }}
              />
              <Tooltip
                contentStyle={{
                  backgroundColor: 'var(--color-card)',
                  border: '1px solid var(--color-border)',
                  borderRadius: '8px',
                }}
                labelFormatter={(t) => format(new Date(String(t)), 'MMM d, HH:mm')}
                formatter={(value, name) => [
                  Number(value).toFixed(1),
                  { ph: 'pH', tds: 'TDS (ppm)', ec: 'EC (mS/cm)', solTemp: 'Sol. Temp (°C)' }[String(name)] ?? name,
                ]}
              />
              <Legend />
              <Line yAxisId="ph" type="monotone" dataKey="ph" stroke="#a855f7" dot={false} name="pH" />
              <Line yAxisId="tds" type="monotone" dataKey="tds" stroke="#06b6d4" dot={false} name="TDS" />
              <Line yAxisId="ph" type="monotone" dataKey="ec" stroke="#14b8a6" dot={false} name="EC" />
              <Line yAxisId="ph" type="monotone" dataKey="solTemp" stroke="#f43f5e" dot={false} name="Sol. Temp" />
            </LineChart>
          </ResponsiveContainer>
        )}
      </CardContent>
    </Card>
  );
}
