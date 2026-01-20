"use client";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { PokemonSetsButton } from "@/components/PokemonSetsButton";
import { home_centered_sprite, Pokemon } from "@/lib/utils";
import Image from "next/image";
import { useState, useTransition } from "react";
import { abandonRun, getArenaRun, makePick } from "./actions";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { useRouter } from "next/navigation";

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

export default function ArenaClient({
  initialRunInfo,
  initialPick,
  username,
}: ArenaClientProps) {
  const [runInfo, setRunInfo] = useState(initialRunInfo);
  const [pick, setPick] = useState<Pick | null>(initialPick);
  const [isPendingPick, startPickTransition] = useTransition();
  const router = useRouter();

  const handlePick = (optionNo: number) => {
    startPickTransition(async () => {
      const newPick = await makePick(username, optionNo);
      setPick(newPick);

      const [updatedRunInfo, updatedPick] = await getArenaRun(username);
      setRunInfo(updatedRunInfo);
      setPick(updatedPick);
    });
  };

  const handleAbandon = async () => {
    startPickTransition(async () => {
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

  // Placeholder match data
  const matches = [
    { opponent: "Trainer_Alex", result: "2-1", won: true },
    { opponent: "Champion_Blake", result: "2-0", won: true },
    { opponent: "Master_Riley", result: "1-2", won: false },
  ];

  console.log(runInfo.created_at);

  return (
    <div className="flex flex-col gap-3 lg:h-[calc(100vh-6rem)] lg:overflow-hidden">
      <h1 className="text-center text-3xl font-extrabold tracking-tight">
        Arena
      </h1>

      <div className="flex flex-col lg:flex-row gap-4 flex-1 lg:min-h-0">
        {/* POOL */}
        <aside className="w-full lg:w-64 xl:w-72 shrink-0 lg:h-full">
          <Card className="h-full max-h-96 lg:max-h-none flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-center">
                <CardTitle className="text-lg">Pool</CardTitle>
                <span className="text-sm text-muted-foreground">
                  <span className="text-sm text-muted-foreground">
                    {new Date(runInfo.created_at).toLocaleString(undefined, {
                      dateStyle: "medium",
                    })}
                  </span>
                </span>
              </div>
            </CardHeader>
            <CardContent className="flex-1 min-h-0 p-0 px-3 pb-3">
              <ScrollArea className="h-full pr-3">
                {runInfo.team.length === 0 ? (
                  <p className="text-muted-foreground text-center py-6 text-sm">
                    No Pokémon drafted yet
                  </p>
                ) : (
                  <div className="space-y-2">
                    {runInfo.team.map((pokemon, index) => (
                      <div
                        key={index}
                        className="flex items-center gap-2 rounded-lg border p-2 bg-muted/40"
                      >
                        <div className="relative w-12 h-12 shrink-0">
                          <Image
                            src={home_centered_sprite(pokemon)}
                            alt={pokemon.name}
                            fill
                            className="object-contain"
                          />
                        </div>

                        <div className="min-w-0 flex-1">
                          <div className="font-semibold truncate text-sm">
                            {pokemon.name}
                          </div>
                          <div className="flex gap-1 flex-wrap">
                            {pokemon.types.map(
                              (type, i) =>
                                type && (
                                  <div
                                    key={i}
                                    className="relative w-10 h-6 shrink-0"
                                  >
                                    <Image
                                      src={`https://play.pokemonshowdown.com/sprites/types/${type}.png`}
                                      alt={type}
                                      fill
                                      className="object-contain"
                                    />
                                  </div>
                                ),
                            )}
                          </div>
                        </div>

                        <PokemonSetsButton pokemon={pokemon} />
                      </div>
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
              <h2 className="text-center text-xl font-bold tracking-tight mb-1 shrink-0">
                Pick {pick.pick_num} / 12
              </h2>

              {/* Desktop */}
              <div className="hidden lg:flex flex-1 items-center justify-center min-h-0 p-1">
                <div className="w-full h-full max-w-4xl grid grid-cols-3 grid-rows-3 gap-5">
                  {pick.options.map((pokemon, index) => (
                    <Card
                      key={index}
                      className={`flex flex-col p-3 ${
                        isPendingPick ? "opacity-50 pointer-events-none" : ""
                      }`}
                    >
                      <div className="relative flex-1 w-full min-h-0">
                        <Image
                          src={home_centered_sprite(pokemon)}
                          alt={pokemon.name}
                          fill
                          className="object-contain"
                          priority
                        />
                      </div>

                      <div className="text-center -mt-6">
                        <div className="font-semibold text-lg leading-tight">
                          {pokemon.name}
                        </div>

                        <div className="flex justify-center gap-1 mt-1">
                          {pokemon.types.map(
                            (type, i) =>
                              type && (
                                <div key={i} className="relative w-10 h-10">
                                  <Image
                                    src={`https://play.pokemonshowdown.com/sprites/types/${type}.png`}
                                    alt={type}
                                    fill
                                    className="object-contain"
                                  />
                                </div>
                              ),
                          )}
                        </div>

                        <div className="flex gap-2 justify-center mt-3">
                          <Button
                            className="cursor-pointer"
                            size="sm"
                            onClick={() => handlePick(index)}
                          >
                            Pick
                          </Button>
                          <PokemonSetsButton pokemon={pokemon} />
                        </div>
                      </div>
                    </Card>
                  ))}
                </div>
              </div>

              {/* Mobile / Tablet */}
              <div className="lg:hidden">
                <div className="w-full grid grid-cols-2 sm:grid-cols-3 gap-5 p-2 pb-4">
                  {pick.options.map((pokemon, index) => (
                    <Card
                      key={index}
                      className={`flex flex-col p-3 ${
                        isPendingPick ? "opacity-50 pointer-events-none" : ""
                      }`}
                    >
                      <div className="relative aspect-square w-full">
                        <Image
                          src={home_centered_sprite(pokemon)}
                          alt={pokemon.name}
                          fill
                          className="object-contain"
                          priority
                        />
                      </div>

                      <div className="text-center -mt-6">
                        <div className="font-semibold text-lg leading-tight">
                          {pokemon.name}
                        </div>

                        <div className="flex justify-center gap-1.5 mt-1">
                          {pokemon.types.map(
                            (type, i) =>
                              type && (
                                <div key={i} className="relative w-10 h-10">
                                  <Image
                                    src={`https://play.pokemonshowdown.com/sprites/types/${type}.png`}
                                    alt={type}
                                    fill
                                    className="object-contain"
                                  />
                                </div>
                              ),
                          )}
                        </div>

                        <div className="flex gap-2 justify-center mt-3">
                          <Button
                            className="cursor-pointer"
                            size="sm"
                            onClick={() => handlePick(index)}
                          >
                            Pick
                          </Button>
                          <PokemonSetsButton pokemon={pokemon} />
                        </div>
                      </div>
                    </Card>
                  ))}
                </div>
              </div>
            </>
          ) : (
            <div className="flex flex-col h-full gap-4 lg:overflow-y-auto p-2">
              {/* Instructions Card */}
              <Card>
                <CardHeader>
                  <CardTitle>Draft Complete!</CardTitle>
                  <CardDescription>
                    Build teams from your drafted pool and compete on the
                    ladder. <strong>3 losses and you&apos;re out!</strong> Each
                    match is <strong>Bo3</strong>.
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex gap-3">
                    <Button className="flex-1" size="lg">
                      Queue for Match
                    </Button>
                    <Button
                      variant="destructive"
                      size="lg"
                      onClick={handleAbandon}
                    >
                      Abandon Run
                    </Button>
                  </div>
                </CardContent>
              </Card>

              {/* Match History */}
              <Card className="flex-1 min-h-0 flex flex-col">
                <CardHeader>
                  <div className="flex justify-between items-center">
                    <CardTitle className="text-lg">Match History</CardTitle>
                    <Badge variant="secondary" className="text-xs">
                      {runInfo.wins} – {runInfo.losses}
                    </Badge>
                  </div>
                </CardHeader>
                <Separator />
                <CardContent className="flex-1 min-h-0 p-0">
                  <ScrollArea className="h-full p-4">
                    {matches.length === 0 ? (
                      <p className="text-muted-foreground text-center py-8 text-sm">
                        No matches played yet
                      </p>
                    ) : (
                      <div className="space-y-2">
                        {matches.map((match, index) => (
                          <div
                            key={index}
                            className={`flex items-center justify-between p-3 rounded-lg border ${
                              match.won
                                ? "bg-green-500/10 border-green-500/30"
                                : "bg-red-500/10 border-red-500/30"
                            }`}
                          >
                            <div className="flex items-center gap-3">
                              <div
                                className={`w-2 h-2 rounded-full ${
                                  match.won ? "bg-green-500" : "bg-red-500"
                                }`}
                              />
                              <span className="font-medium">
                                vs {match.opponent}
                              </span>
                            </div>
                            <Badge
                              variant={match.won ? "default" : "destructive"}
                              className={
                                match.won
                                  ? "bg-green-500 hover:bg-green-600"
                                  : ""
                              }
                            >
                              {match.result}
                            </Badge>
                          </div>
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
