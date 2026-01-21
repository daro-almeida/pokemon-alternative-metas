import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import Image from "next/image";
import { PokemonSetsButton } from "@/components/PokemonSetsButton";
import { home_centered_sprite } from "@/lib/api/pokemon";
import { Pokemon } from "@/lib/types/pokemon";

interface PickCardProps {
  pokemon: Pokemon;
  onPick: () => void;
  disabled?: boolean;
}

export function PickCard({ pokemon, onPick, disabled }: PickCardProps) {
  return (
    <Card
      className={`flex flex-col p-3 ${disabled ? "opacity-50 pointer-events-none" : ""}`}
    >
      <div className="relative flex-1 w-full min-h-0 aspect-square">
        <Image
          src={home_centered_sprite(pokemon)}
          alt={pokemon.name}
          fill
          className="object-contain"
          priority
        />
      </div>

      <div className="text-center -mt-6">
        <div className="font-semibold text-base lg:text-lg leading-tight">
          {pokemon.name}
        </div>

        <div className="flex justify-center gap-1 mt-1">
          {pokemon.types.map(
            (type, i) =>
              type && (
                <div key={i} className="relative w-8 h-8 lg:w-10 lg:h-10">
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
          <Button className="cursor-pointer" size="sm" onClick={onPick}>
            Pick
          </Button>
          <PokemonSetsButton pokemon={pokemon} />
        </div>
      </div>
    </Card>
  );
}
