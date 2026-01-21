import { Badge } from "@/components/ui/badge";

interface MatchHistoryItemProps {
  opponent: string;
  result: string;
  won: boolean;
}

export function MatchHistoryItem({
  opponent,
  result,
  won,
}: MatchHistoryItemProps) {
  return (
    <div
      className={`flex items-center justify-between p-3 rounded-lg border ${
        won
          ? "bg-green-500/10 border-green-500/30"
          : "bg-red-500/10 border-red-500/30"
      }`}
    >
      <div className="flex items-center gap-3">
        <div
          className={`w-2 h-2 rounded-full ${
            won ? "bg-green-500" : "bg-red-500"
          }`}
        />
        <span className="font-medium">vs {opponent}</span>
      </div>
      <Badge
        variant={won ? "default" : "destructive"}
        className={won ? "bg-green-500 hover:bg-green-600" : ""}
      >
        {result}
      </Badge>
    </div>
  );
}
