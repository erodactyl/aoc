import * as fs from "fs";

function main() {
  const antinodes = new Set<string>();

  const content = fs.readFileSync("input.txt", "utf-8");

  const matrix = content
    .split("\n")
    .filter((line) => line.length > 0)
    .map((line) => line.split(""));

  const letters = new Set<string>();

  for (let i = 0; i < matrix.length; i++) {
    for (let j = 0; j < matrix[i].length; j++) {
      const cell = matrix[i][j];
      if (cell !== "." && !letters.has(cell)) {
        letters.add(cell);
        mark_antinodes(matrix, cell, antinodes);
      }
    }
  }

  console.log(antinodes);
  console.log(antinodes.size);
}

function mark_antinodes(
  matrix: string[][],
  char: string,
  antinodes: Set<string>
) {
  let nodes: { i: number; j: number }[] = [];

  for (let i = 0; i < matrix.length; i++) {
    for (let j = 0; j < matrix[i].length; j++) {
      const cell = matrix[i][j];
      if (cell === char) {
        nodes.push({ i, j });
      }
    }
  }

  if (nodes.length <= 1) {
    return;
  }

  const mark = ({ i, j }: { i: number; j: number }) => {
    if (i < matrix.length && i >= 0) {
      if (matrix[0].length > j && j >= 0) {
        antinodes.add(`${i}:${j}`);
      }
    }
  };

  for (let i = 0; i < nodes.length; i++) {
    for (let j = i; j < nodes.length; j++) {
      if (i === j) {
        continue;
      }

      const node_1 = nodes[i];
      const node_2 = nodes[j];

      const node_i_diff = node_2.i - node_1.i;
      const node_j_diff = node_2.j - node_1.j;

      let multiplier = 0;
      while (true) {
        const i = node_1.i - node_i_diff * multiplier;
        const j = node_1.j - node_j_diff * multiplier;

        if (i < 0 || i >= matrix.length || j < 0 || j >= matrix[0].length) {
          break;
        }

        mark({ i, j });

        multiplier++;
      }

      multiplier = 0;
      while (true) {
        const i = node_2.i + node_i_diff * multiplier;
        const j = node_2.j + node_j_diff * multiplier;

        if (i < 0 || i >= matrix.length || j < 0 || j >= matrix[0].length) {
          break;
        }

        mark({ i, j });

        multiplier++;
      }

      //const antinode_1: { i: number; j: number } = {
      //  j: node_1.j - node_j_diff,
      //  i: node_1.i - node_i_diff,
      //};
      //const antinode_2: { i: number; j: number } = {
      //  j: node_2.j + node_j_diff,
      //  i: node_2.i + node_i_diff,
      //};
      //
      //mark(antinode_1);
      //mark(antinode_2);
    }
  }
}

main();
