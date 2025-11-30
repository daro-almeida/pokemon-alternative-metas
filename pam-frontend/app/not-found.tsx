"use client";

import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";

export default function NotFound() {
  const router = useRouter();

  return (
    <main className="max-w-3xl mx-auto p-6 pt-16 text-center">
      {/* Heading */}
      <h1 className="text-6xl font-bold mb-4">404</h1>
      <p className="text-lg text-muted-foreground mb-6">
        Sorry :(
      </p>

      {/* Buttons */}
      <div className="flex gap-4 justify-center">
        <Button onClick={() => router.back()}>
          Go back
        </Button>
      </div>
    </main>
  );
}
