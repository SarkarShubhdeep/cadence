import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface UseCaptureResult {
  isCapturing: boolean;
  toggle: () => Promise<void>;
  error: string | null;
  clearError: () => void;
}

function toErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (typeof error === "string") return error;
  return "Something went wrong with capture";
}

export function useCapture(): UseCaptureResult {
  const [isCapturing, setIsCapturing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<boolean>("is_capturing")
      .then(setIsCapturing)
      .catch((err: unknown) => setError(toErrorMessage(err)));
  }, []);

  useEffect(() => {
    const unlisten = listen<string>("capture-error", (event) => {
      setError(event.payload);
    });

    return () => {
      unlisten.then((stopListening) => stopListening());
    };
  }, []);

  const toggle = useCallback(async () => {
    try {
      await invoke(isCapturing ? "stop_capture" : "start_capture");
      setIsCapturing((current) => !current);
    } catch (err) {
      setError(toErrorMessage(err));
    }
  }, [isCapturing]);

  const clearError = useCallback(() => setError(null), []);

  return { isCapturing, toggle, error, clearError };
}
