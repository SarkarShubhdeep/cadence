import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

interface ContextSwitchStatProps {
  contextSwitches: number;
}

export function ContextSwitchStat({ contextSwitches }: ContextSwitchStatProps) {
  return (
    <Card>
      <CardHeader>
        <CardDescription>Context switches today</CardDescription>
        <CardTitle className="text-3xl">{contextSwitches}</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="text-sm text-muted-foreground">
          Number of times the active app changed.
        </p>
      </CardContent>
    </Card>
  );
}
