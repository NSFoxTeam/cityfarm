import { Leaf } from 'lucide-react';
import { useLatestReadings } from '@/hooks/useReadings';
import { EnvironmentSection } from '@/components/Dashboard/EnvironmentSection';
import { WaterQuality } from '@/components/Dashboard/WaterQuality';
import { WaterQualityChart } from '@/components/Charts/WaterQualityChart';
import { AlertBanner } from '@/components/Alerts/AlertBanner';

export function Dashboard() {
  const { data: readings = [], isLoading } = useLatestReadings();

  return (
    <div className="min-h-screen bg-background p-4 md:p-8">
      <header className="mb-8 flex items-center gap-3">
        <Leaf className="h-8 w-8 text-green-500" />
        <h1 className="text-2xl font-bold">CityFarm Dashboard</h1>
      </header>

      <div className="mx-auto flex max-w-7xl flex-col gap-8">
        <AlertBanner readings={readings} />
        <EnvironmentSection readings={readings} loading={isLoading} />
        <WaterQuality readings={readings} loading={isLoading} />
        <WaterQualityChart />
      </div>
    </div>
  );
}
