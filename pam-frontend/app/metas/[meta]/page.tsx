import { Button } from "@/components/ui/button";
import { metas } from "@/data/metas";
import { notFound } from "next/navigation";
import Link from "next/link";

export default async function Page(props: { params: Promise<{ meta: string }> }) {
  const params = await props.params;
  const meta = metas.find((m) => m.meta === params.meta);

  if (!meta) return notFound();

  return (
    <section className="max-w-3xl mx-auto py-20 space-y-6">
      <h1 className="text-4xl font-bold tracking-tight">{meta.title}</h1>
      <p className="text-muted-foreground text-lg">{meta.large_description}</p>

      <Button asChild><Link href={`/play_${meta.meta}`}>Play</Link></Button>
    </section>
  );
}
