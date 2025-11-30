import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { metas } from "@/data/metas";
import Link from "next/link";

export default function Home() {
  return (
    <section className="text-center space-y-8 py-20">

      {/* HERO TEXT */}
      <div className="space-y-4">
        <h1 className="text-5xl font-bold tracking-tight">
          Pokémon Alternative Metas
        </h1>
        <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
          A collection of Pokémon alternative metas I came up with, including matchmaking system and ladders.
        </p>
      </div>

      {/* CARD GRID */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 max-w-4xl mx-auto">
        {metas.map((m) => (
          <Link key={m.meta} href={`/metas/${m.meta}`} className="group">
            <Card className="h-full cursor-pointer transition-all group-hover:shadow-md group-hover:-translate-y-1">
              <CardHeader>
                <CardTitle>{m.title}</CardTitle>
              </CardHeader>
              <CardContent className="text-muted-foreground">
                {m.description}
              </CardContent>
            </Card>
          </Link>
        ))}
      </div>
    </section>
  );
}
