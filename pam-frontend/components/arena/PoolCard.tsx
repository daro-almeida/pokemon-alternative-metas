import Image from "next/image";
import { PokemonSetsButton } from "@/components/PokemonSetsButton";
import { home_centered_sprite } from "@/lib/api/pokemon";
import { Pokemon } from "@/lib/types/pokemon";

interface PoolCardProps {
  pokemon: Pokemon;
}

export function PoolCard({ pokemon }: PoolCardProps) {
  return (
    <div className="grid grid-cols-[auto_1fr_auto] items-center gap-2 rounded-lg border p-2 bg-muted/40">
      <div className="relative w-12 h-12 shrink-0">
        <Image
          src={home_centered_sprite(pokemon)}
          alt={pokemon.name}
          fill
          className="object-contain"
        />
      </div>

      <div className="min-w-0 overflow-hidden">
        <div className="font-semibold text-sm truncate" title={pokemon.name}>
          {pokemon.name}
        </div>
        <div className="flex flex-wrap gap-1">
          {pokemon.types.map(
            (type, i) =>
              type && (
                <div key={i} className="relative w-8 h-5 shrink-0">
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

      <div className="shrink-0">
        <PokemonSetsButton pokemon={pokemon} />
      </div>
    </div>
  );
}
