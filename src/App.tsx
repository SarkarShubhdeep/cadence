import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

function App() {
  return (
    <main className="flex min-h-screen items-center justify-center bg-background">
      <Card className="w-80">
        <CardHeader>
          <CardTitle className="text-2xl">Cadence</CardTitle>
          <CardDescription>Local-first developer telemetry</CardDescription>
        </CardHeader>
        <CardContent />
      </Card>
    </main>
  );
}

export default App;
