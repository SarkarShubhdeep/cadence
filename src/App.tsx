import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

function App() {
  const [isCapturing, setIsCapturing] = useState(false);

  useEffect(() => {
    invoke<boolean>("is_capturing")
      .then(setIsCapturing)
      .catch((error: unknown) => {
        console.error("cadence: failed to read capture status", error);
      });
  }, []);

  const toggleCapture = async () => {
    try {
      await invoke(isCapturing ? "stop_capture" : "start_capture");
      setIsCapturing(!isCapturing);
    } catch (error) {
      console.error("cadence: failed to toggle capture", error);
    }
  };

  return (
    <main className="flex min-h-screen items-center justify-center bg-background">
      <Card className="w-80">
        <CardHeader>
          <CardTitle className="text-2xl">Cadence</CardTitle>
          <CardDescription>Local-first developer telemetry</CardDescription>
        </CardHeader>
        <CardContent className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">
            {isCapturing ? "Capturing" : "Idle"}
          </span>
          <Button
            variant={isCapturing ? "outline" : "default"}
            onClick={toggleCapture}
          >
            {isCapturing ? "Stop" : "Start"}
          </Button>
        </CardContent>
      </Card>
    </main>
  );
}

export default App;
