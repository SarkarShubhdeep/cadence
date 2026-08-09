import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { formatDurationMs } from "@/lib/format";
import type { AppTotal } from "@/types/summary";

interface AppTotalsListProps {
  appTotals: AppTotal[];
}

export function AppTotalsList({ appTotals }: AppTotalsListProps) {
  const maxTotalMs = Math.max(...appTotals.map((total) => total.totalFocusedMs));

  return (
    <Card>
      <CardHeader>
        <CardTitle>Time per app</CardTitle>
        <CardDescription>Total focused time today, most-used first</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        {appTotals.map((total) => (
          <div key={total.appName} className="flex flex-col gap-1">
            <div className="flex items-center justify-between text-sm">
              <span className="font-medium">{total.appName}</span>
              <span className="text-muted-foreground">
                {formatDurationMs(total.totalFocusedMs)}
              </span>
            </div>
            <div className="h-1.5 w-full overflow-hidden rounded-full bg-muted">
              <div
                className="h-full rounded-full bg-primary"
                style={{
                  width: `${(total.totalFocusedMs / maxTotalMs) * 100}%`,
                }}
              />
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
