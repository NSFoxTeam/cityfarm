import type { LucideIcon } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import type { AlertLevel } from '@/types/sensor';

interface SensorCardProps {
  title: string;
  value: number | undefined;
  unit: string;
  icon: LucideIcon;
  color: string;
  alertLevel?: AlertLevel;
  loading?: boolean;
}

const alertStyles: Record<string, string> = {
  normal: 'border-l-green-500',
  warning: 'border-l-amber-500',
  critical: 'border-l-red-500',
};

const alertBadgeVariant: Record<string, string> = {
  warning: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
  critical: 'bg-red-500/20 text-red-400 border-red-500/30',
};

export function SensorCard({
  title,
  value,
  unit,
  icon: Icon,
  color,
  alertLevel = 'normal',
  loading,
}: SensorCardProps) {
  if (loading) {
    return (
      <Card className="border-l-4 border-l-muted">
        <CardHeader className="flex flex-row items-center justify-between pb-2">
          <CardTitle className="text-sm font-medium text-muted-foreground">
            {title}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="h-8 w-20 animate-pulse rounded bg-muted" />
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={`border-l-4 ${alertStyles[alertLevel]}`}>
      <CardHeader className="flex flex-row items-center justify-between pb-2">
        <CardTitle className="text-sm font-medium text-muted-foreground">
          {title}
        </CardTitle>
        <Icon className="h-4 w-4" style={{ color }} />
      </CardHeader>
      <CardContent>
        <div className="flex items-baseline gap-2">
          <span className="text-2xl font-bold" style={{ color }}>
            {value !== undefined ? value.toFixed(1) : '—'}
          </span>
          <span className="text-sm text-muted-foreground">{unit}</span>
        </div>
        {alertLevel !== 'normal' && (
          <Badge variant="outline" className={`mt-2 ${alertBadgeVariant[alertLevel]}`}>
            {alertLevel}
          </Badge>
        )}
      </CardContent>
    </Card>
  );
}
