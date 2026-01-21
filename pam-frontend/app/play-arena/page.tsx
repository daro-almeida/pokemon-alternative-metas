import ArenaClient from "./arena-client";
import { getArenaRun } from "./actions";

export default async function Page() {
  const username = "sheesh";

  const [runInfo, pick] = await getArenaRun(username);
  console.log(runInfo, pick);

  return (
    <ArenaClient
      initialRunInfo={runInfo}
      initialPick={pick}
      username={username}
    />
  );
}
