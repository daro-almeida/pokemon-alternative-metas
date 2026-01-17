import { Card, CardContent } from "@/components/ui/card";
import { home_centered_sprite, Pokemon } from "@/lib/utils";
import Image from "next/image";

export default async function Page() {
  const user = "sheesh";
  const res = await fetch(
    `http://localhost:3001/api/arena/${user}/options`,
  );

  if (!res.ok) {
    console.log(await res.text());
    throw new Error(res.statusText);
  }

  const pick = await res.json();
  console.log(pick);

  return (
    <main className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <h1 className="text-4xl sm:text-5xl md:text-6xl font-extrabold text-center mb-10">
        {`Pick no. ${pick.pick_num}/12`}
      </h1>

      {/* Responsive grid: 1 column on xs, 2 on sm, 3 on md+ */}
      <div className="grid gap-6 sm:grid-cols-2 md:grid-cols-3">
        {pick.options.map((pokemon: Pokemon, index: number) => (
          <Card
            key={index}
            className="flex flex-col items-center w-full h-full p-4 sm:p-6rounded-xl shadow-md hover:shadow-xl hover:scale-105 transition-transform duration-300 ease-ou"
          >
            <CardContent className="relative w-full aspect-square">
              <Image
                src={home_centered_sprite(pokemon)}
                alt={pokemon.name}
                fill
                style={{ objectFit: "contain" }}
                priority
              />
            </CardContent>

            <div className="text-center font-semibold text-lg sm:text-xl leading-tigh -mt-6">
              {pokemon.name}
            </div>
            <div className="flex gap-1 justify-center -mt-8">
              {pokemon.types.map((type, index) =>
                type ? (
                  <div
                    key={index}
                    className="relative w-8 h-8 sm:w-10 sm:h-10 lg:w-12 lg:h-12"
                  >
                    <Image
                      src={`https://play.pokemonshowdown.com/sprites/types/${type}.png`}
                      alt={type}
                      fill
                      style={{ objectFit: "contain" }}
                      priority
                    />
                  </div>
                ) : null,
              )}
            </div>
          </Card>
        ))}
      </div>
    </main>
  );
}
