import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { formatClockTime, formatDurationMs } from "@/lib/format";
import type { FocusSession } from "@/types/summary";

interface FocusSessionTimelineProps {
  focusSessions: FocusSession[];
}

export function FocusSessionTimeline({ focusSessions }: FocusSessionTimelineProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Focus sessions</CardTitle>
        <CardDescription>
          Contiguous blocks of time in one app, at least 30s long
        </CardDescription>
      </CardHeader>
      <CardContent>
        {focusSessions.length === 0 ? (
          <p className="text-sm text-muted-foreground">
            No focus sessions yet today.
          </p>
        ) : (
          <TimelineTrack focusSessions={focusSessions} />
        )}
      </CardContent>
    </Card>
  );
}

function TimelineTrack({ focusSessions }: FocusSessionTimelineProps) {
  const rangeStartMs = Math.min(...focusSessions.map((session) => session.startedAtMs));
  const rangeEndMs = Math.max(...focusSessions.map((session) => session.endedAtMs));
  const rangeMs = Math.max(rangeEndMs - rangeStartMs, 1);

  return (
    <div className="flex flex-col gap-2">
      <div className="relative h-10 w-full overflow-hidden rounded-lg bg-muted">
        {focusSessions.map((session) => {
          const leftPercent = ((session.startedAtMs - rangeStartMs) / rangeMs) * 100;
          const widthPercent = (session.durationMs / rangeMs) * 100;

          return (
            <div
              key={`${session.appName}-${session.startedAtMs}`}
              title={`${session.appName} · ${formatDurationMs(session.durationMs)}`}
              className="absolute top-0 h-full min-w-[2px] rounded-sm bg-primary/70"
              style={{ left: `${leftPercent}%`, width: `${widthPercent}%` }}
            />
          );
        })}
      </div>
      <div className="flex items-center justify-between text-xs text-muted-foreground">
        <span>{formatClockTime(rangeStartMs)}</span>
        <span>{formatClockTime(rangeEndMs)}</span>
      </div>
    </div>
  );
}
