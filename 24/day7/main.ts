import * as fs from "fs";

function main() {
  const content = fs.readFileSync("input.txt", "utf-8");

  const matrix = content
    .split("\n")
    .filter((line) => line.length > 0)
    .map((line) => line.split(""));

  console.log(matrix);
}

main();
