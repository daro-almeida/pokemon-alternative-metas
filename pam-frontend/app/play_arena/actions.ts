"use server";

export async function getArenaRun(username: string) {
  const res = await fetch(`http://localhost:3001/api/arena/${username}/run`, {
    cache: "no-store",
  });

  if (!res.ok) {
    throw new Error(`Failed to fetch run: ${res.statusText}`);
  }

  return res.json();
}

export async function makePick(username: string, optionNo: number) {
  const res = await fetch(`http://localhost:3001/api/arena/${username}/pick`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ option_no: optionNo }),
    cache: "no-store",
  });

  if (!res.ok) {
    throw new Error(`Failed to make pick: ${res.statusText}`);
  }

  return res.json();
}
