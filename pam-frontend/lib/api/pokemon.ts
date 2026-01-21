import { Pokemon } from "../types/pokemon";

function normalize(str: string) {
  return str
    .toLowerCase()
    .normalize("NFD")
    .replace(" ", "")
    .replace(/[\u0300-\u036f]/g, "")
    .replace(/[^a-z0-9 \-]/g, "");
  // wo-chien
}

export function home_centered_sprite(pokemon: Pokemon) {
  function process_exceptions(normalized_name: string) {
    switch (normalized_name) {
      case "ting-lu":
        return "tinglu";
      case "wo-chien":
        return "wochien";
      case "chi-yu":
        return "chiyu";
      case "chien-pao":
        return "chienpao";
      case "charizard-mega-x":
        return "charizard-megax";
      case "charizard-mega-y":
        return "charizard-megay";
      case "kommo-o":
        return "kommoo";
      case "tauros-paldea-blaze":
        return "tauros-paldeablaze";
      case "tauros-paldea-combat":
        return "tauros-paldeacombat";
      case "tauros-paldea-aqua":
        return "tauros-paldeaaqua";
      case "porygon-z":
        return "porygonz";
      case "urshifu-rapid-strike":
        return "urshifu-rapidstrike";
      case "hakamo-o":
        return "hakamoo";
      case "jangmo-o":
        return "jangmoo";
      case "nidoran-f":
        return "nidoranf";
      case "nidoran-m":
        return "nidoranm";
      default:
        return normalized_name;
    }
  }
  return `https://play.pokemonshowdown.com/sprites/home-centered/${process_exceptions(
    normalize(pokemon.name),
  )}.png`;
}
