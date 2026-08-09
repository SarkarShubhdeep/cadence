import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import type { TodaySummary } from "@/types/summary";

type TodaySummaryState =
  | { status: "loading" }
  | { status: "ready"; summary: TodaySummary }
  | { status: "error"; message: string };

interface UseTodaySummaryResult {
  state: TodaySummaryState;
  refetch: () => void;
}

function toErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (typeof error === "string") return error;
  return "Failed to load today's summary";
}

export function useTodaySummary(): UseTodaySummaryResult {
  const [state, setState] = useState<TodaySummaryState>({ status: "loading" });
  const [attempt, setAttempt] = useState(0);

  useEffect(() => {
    let cancelled = false;

    invoke<TodaySummary>("get_today_summary")
      .then((summary) => {
        if (!cancelled) setState({ status: "ready", summary });
      })
      .catch((error: unknown) => {
        if (!cancelled) setState({ status: "error", message: toErrorMessage(error) });
      });

    return () => {
      cancelled = true;
    };
  }, [attempt]);

  return {
    state,
    refetch: () => setAttempt((current) => current + 1),
  };
}
