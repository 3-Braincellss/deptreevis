import ForceGraph2D from "react-force-graph-2d";
import { Center, Stack, Title } from "@mantine/core";

export function genRandomTree(N = 300, reverse = false) {
  return {
    nodes: [...Array(N).keys()].map((i) => ({ id: i })),
    links: [...Array(N).keys()]
      .filter((id) => id)
      .map((id) => ({
        [reverse ? "target" : "source"]: id,
        [reverse ? "source" : "target"]: Math.round(Math.random() * (id - 1)),
      })),
  };
}

function Home() {
  return (
    <Stack>
      <Center>
        <Title variant="h1">DPV-ui</Title>
      </Center>
      <ForceGraph2D graphData={genRandomTree(50)} />
    </Stack>
  );
}

export default Home;
