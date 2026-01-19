"use client";

import { Button } from "@/components/ui/button";
import { Pokemon } from "@/lib/utils";

interface PokemonInfoButtonProps {
  pokemon: Pokemon;
  size?: "sm" | "default";
  variant?: "secondary" | "outline";
}

export function PokemonSetsButton({
  pokemon,
  size = "sm",
  variant = "secondary",
}: PokemonInfoButtonProps) {
  const handleClick = () => {
    window.open(
      `https://www.smogon.com/dex/sv/pokemon/${pokemon.name.toLowerCase()}/`,
      "_blank",
      "noopener,noreferrer",
    );
  };

  return (
    <Button
      className="cursor-pointer"
      size={size}
      variant={variant}
      onClick={handleClick}
    >
      Sets
    </Button>
  );
}
