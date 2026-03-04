"use client";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useState, useTransition } from "react";
import { abandonRun, getArenaRun, makePick } from "./actions";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { useRouter } from "next/navigation";
import { PoolCard } from "@/components/arena/PoolCard";
import { PickCard } from "@/components/arena/PickCard";
import { MatchHistoryItem } from "@/components/arena/MatchHistoryItem";
import { Pokemon } from "@/lib/types/pokemon";
import { ARENA_NUM_PICKS } from "@/lib/constants";

interface Pick {
  pick_num: number;
  options: Pokemon[];
}

interface ArenaRunInfo {
  username: string;
  created_at: string;
  wins: number;
  losses: number;
  team: Pokemon[];
  team_buckets: number[];
}

interface ArenaClientProps {
  initialRunInfo: ArenaRunInfo;
  initialPick: Pick | null;
  username: string;
}

// Placeholder match data - TODO: fetch from backend
const PLACEHOLDER_MATCHES = [
  { opponent: "Trainer_Alex", result: "2-1", won: true },
  { opponent: "Champion_Blake", result: "2-0", won: true },
  { opponent: "Master_Riley", result: "1-2", won: false },
];

export default function ArenaClient({
  initialRunInfo,
  initialPick,
  username,
}: ArenaClientProps) {
  const [runInfo, setRunInfo] = useState(initialRunInfo);
  const [pick, setPick] = useState<Pick | null>(initialPick);
  const [isPending, startTransition] = useTransition();
  const router = useRouter();

  const handlePick = (optionNo: number) => {
    startTransition(async () => {
      const newPick = await makePick(username, optionNo);
      setPick(newPick);

      const [updatedRunInfo, updatedPick] = await getArenaRun(username);
      setRunInfo(updatedRunInfo);
      setPick(updatedPick);
    });
  };

  const handleAbandon = () => {
    startTransition(async () => {
      try {
        await abandonRun(username);
        const [updatedRunInfo, updatedPick] = await getArenaRun(username);
        setRunInfo(updatedRunInfo);
        setPick(updatedPick);
        router.refresh();
      } catch (error) {
        console.error("Failed to abandon run:", error);
      }
    });
  };

  const formattedDate = new Date(runInfo.created_at).toLocaleDateString(
    undefined,
    {
      month: "short",
      day: "numeric",
    },
  );

  return (
    <div className="flex flex-col gap-3 lg:h-[calc(100vh-6rem)] lg:overflow-hidden">
      <h1 className="text-center text-2xl sm:text-3xl font-extrabold tracking-tight">
        Arena
      </h1>

      <div className="flex flex-col lg:flex-row gap-4 flex-1 lg:min-h-0">
        {/* POOL */}
        <aside className="w-full lg:w-64 xl:w-72 shrink-0 lg:h-full">
          <Card className="h-full lg:max-h-none flex flex-col overflow-hidden">
            <CardHeader className="pb-3 shrink-0">
              <div className="flex justify-between items-center gap-2">
                <CardTitle className="text-lg">Pool</CardTitle>
                <span className="text-xs text-muted-foreground whitespace-nowrap">
                  {formattedDate}
                </span>
              </div>
            </CardHeader>

            <CardContent className="flex-1 min-h-0 p-0 px-3 pb-3 overflow-hidden">
              <ScrollArea className="h-full pr-2">
                {runInfo.team.length === 0 ? (
                  <p className="text-muted-foreground text-center py-8 text-sm">
                    No Pokémon drafted yet
                  </p>
                ) : (
                  <div className="space-y-2">
                    {runInfo.team.map((pokemon, index) => (
                      <PoolCard key={index} pokemon={pokemon} />
                    ))}
                  </div>
                )}
              </ScrollArea>
            </CardContent>
          </Card>
        </aside>

        {/* PICKS OR LADDER */}
        <section className="flex-1 lg:h-full flex flex-col lg:min-h-0 lg:overflow-hidden">
          {pick ? (
            <>
              <h2 className="text-center text-lg sm:text-xl font-bold tracking-tight mb-2 shrink-0">
                Pick {pick.pick_num} / {ARENA_NUM_PICKS}
              </h2>

              <div className="flex-1 lg:flex items-center justify-center min-h-0 p-1">
                <div className="w-full h-full max-w-4xl grid grid-cols-2 sm:grid-cols-3 lg:grid-rows-3 gap-3 sm:gap-4 lg:gap-5">
                  {pick.options.map((pokemon, index) => (
                    <PickCard
                      key={index}
                      pokemon={pokemon}
                      onPick={() => handlePick(index)}
                      disabled={isPending}
                    />
                  ))}
                </div>
              </div>
            </>
          ) : (
            <div className="flex flex-col h-full gap-4 lg:overflow-y-auto p-2">
              {/* Instructions */}
              <Card>
                <CardHeader className="pb-3">
                  <CardTitle>Draft Complete!</CardTitle>
                  <CardDescription>
                    Build teams from your drafted pool and compete on the
                    ladder. <strong>3 losses and you&apos;re out!</strong> Each
                    match is <strong>Bo3</strong>.
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex flex-col sm:flex-row gap-3">
                    <Button className="flex-1 w-full sm:w-auto cursor-pointer">
                      Queue for Match
                    </Button>
                    <Button
                      variant="destructive"
                      className="w-full sm:w-auto cursor-pointer"
                      onClick={handleAbandon}
                      disabled={isPending}
                    >
                      Abandon Run
                    </Button>
                  </div>
                </CardContent>
              </Card>

              {/* Match History */}
              <Card className="flex-1 min-h-0 flex flex-col">
                <CardHeader className="pb-1">
                  <div className="flex justify-between items-center gap-2">
                    <CardTitle className="text-lg">Match History</CardTitle>
                    <Badge variant="secondary" className="text-xs shrink-0">
                      {runInfo.wins}W – {runInfo.losses}L
                    </Badge>
                  </div>
                </CardHeader>
                <Separator />
                <CardContent className="flex-1 min-h-0 p-0">
                  <ScrollArea className="h-full p-4">
                    {PLACEHOLDER_MATCHES.length === 0 ? (
                      <p className="text-muted-foreground text-center py-8 text-sm">
                        No matches played yet
                      </p>
                    ) : (
                      <div className="space-y-2">
                        {PLACEHOLDER_MATCHES.map((match, index) => (
                          <MatchHistoryItem key={index} {...match} />
                        ))}
                      </div>
                    )}
                  </ScrollArea>
                </CardContent>
              </Card>
            </div>
          )}
        </section>
      </div>
    </div>
  );
}
