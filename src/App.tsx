import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardTitle } from "@/components/ui/card";
import { AppTotalsList } from "@/components/dashboard/AppTotalsList";
import { ContextSwitchStat } from "@/components/dashboard/ContextSwitchStat";
import { FocusSessionTimeline } from "@/components/dashboard/FocusSessionTimeline";
import { ErrorBanner } from "@/components/ErrorBanner";
import { useCapture } from "@/hooks/useCapture";
import { useTodaySummary } from "@/hooks/useTodaySummary";

function App() {
  const { isCapturing, toggle, error, clearError } = useCapture();
  const { state, refetch } = useTodaySummary();

  return (
    <main className="min-h-screen bg-background px-6 py-8">
      <div className="mx-auto flex max-w-3xl flex-col gap-6">
        <header className="flex items-center justify-between">
          <div>
            <h1 className="font-heading text-2xl font-medium">Cadence</h1>
            <p className="text-sm text-muted-foreground">
              Today's focus summary
            </p>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-sm text-muted-foreground">
              {isCapturing ? "Capturing" : "Idle"}
            </span>
            <Button
              variant={isCapturing ? "outline" : "default"}
              onClick={toggle}
            >
              {isCapturing ? "Stop" : "Start"}
            </Button>
            <Button variant="ghost" onClick={refetch}>
              Refresh
            </Button>
          </div>
        </header>

        {error && <ErrorBanner message={error} onDismiss={clearError} />}

        <DashboardBody state={state} />
      </div>
    </main>
  );
}

type TodaySummaryState = ReturnType<typeof useTodaySummary>["state"];

function DashboardBody({ state }: { state: TodaySummaryState }) {
  if (state.status === "loading") {
    return (
      <p className="text-sm text-muted-foreground">
        Loading today's summary…
      </p>
    );
  }

  if (state.status === "error") {
    return (
      <Card>
        <CardContent className="pt-4">
          <p className="text-sm text-destructive">{state.message}</p>
        </CardContent>
      </Card>
    );
  }

  const { summary } = state;

  if (summary.appTotals.length === 0) {
    return (
      <Card>
        <CardContent className="flex flex-col gap-1 pt-4">
          <CardTitle>No activity captured yet</CardTitle>
          <CardDescription>
            Start capture and work for a bit, then hit Refresh to see today's
            summary.
          </CardDescription>
        </CardContent>
      </Card>
    );
  }

  return (
    <div className="grid gap-4">
      <ContextSwitchStat contextSwitches={summary.contextSwitches} />
      <AppTotalsList appTotals={summary.appTotals} />
      <FocusSessionTimeline focusSessions={summary.focusSessions} />
    </div>
  );
}

export default App;
