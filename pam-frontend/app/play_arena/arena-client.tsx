"use client";

import { Card } from "@/components/ui/card";
import { PokemonSetsButton } from "@/components/PokemonSetsButton";
import { home_centered_sprite, Pokemon } from "@/lib/utils";
import Image from "next/image";
import { useState, useTransition } from "react";
import { getArenaRun, makePick } from "./actions";
import { Button } from "@/components/ui/button";

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
  const [isPending, startTransition] = useTransition();

  const handlePick = (optionNo: number) => {
    startTransition(async () => {
      const newPick = await makePick(username, optionNo);
      setPick(newPick);

      const [updatedRunInfo, updatedPick] = await getArenaRun(username);
      setRunInfo(updatedRunInfo);
      setPick(updatedPick);
    });
  };

  return (
    <div className="flex flex-col gap-3 lg:h-[calc(100vh-6rem)] lg:overflow-hidden">
      <h1 className="text-center text-3xl font-extrabold tracking-tight">
        Arena Draft
      </h1>

      <div className="flex flex-col lg:flex-row gap-4 flex-1 lg:min-h-0">
        {/* POOL */}
        <aside className="w-full lg:w-64 xl:w-72 shrink-0 lg:h-full">
          <Card className="h-full max-h-96 lg:max-h-none flex flex-col p-3 bg-linear-to-b from-card/90 to-card border-border/60 backdrop-blur-sm">
            <div className="flex justify-between items-center shrink-0 mb-1">
              <h2 className="text-lg font-bold tracking-tight">Pool</h2>
              {!pick && (
                <span className="text-sm font-medium text-muted-foreground">
                  {runInfo.wins}W – {runInfo.losses}L
                </span>
              )}
            </div>

            <div className="flex-1 overflow-y-auto space-y-2 pr-1 min-h-0">
              {runInfo.team.length === 0 ? (
                <p className="text-muted-foreground text-center py-6 text-sm">
                  No Pokémon drafted yet
                </p>
              ) : (
                runInfo.team.map((pokemon, index) => (
                  <div
                    key={index}
                    className="flex items-center gap-2 rounded-lg border border-border/60 bg-muted/40 p-2"
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
                      <div className="flex gap-1">
                        {pokemon.types.map(
                          (type, i) =>
                            type && (
                              <div key={i} className="relative w-10 h-6">
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
                ))
              )}
            </div>
          </Card>
        </aside>

        {/* PICKS */}
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
                      className={`flex flex-col border-border/60 bg-card/80 backdrop-blur-sm p-3 ${
                        isPending ? "opacity-50 pointer-events-none" : ""
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
                      className={`flex flex-col border-border/60 bg-card/80 backdrop-blur-sm p-3 ${
                        isPending ? "opacity-50 pointer-events-none" : ""
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
            <div className="flex items-center justify-center py-12">
              <Card className="p-8 text-center bg-card/80 backdrop-blur-sm">
                <h2 className="text-xl font-bold mb-2">Draft Complete!</h2>
                <p className="text-muted-foreground">
                  Your team is ready to battle.
                </p>
              </Card>
            </div>
          )}
        </section>
      </div>
    </div>
  );
}
