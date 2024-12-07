import * as fs from "fs";

function main() {
  const content = fs.readFileSync("input.txt", "utf-8");

  const tests = content
    .split("\n")
    .filter((line) => line.length > 0)
    .map((line) => {
      const [result, numbers] = line.split(": ");
      return {
        result: Number(result),
        numbers: numbers.split(" ").map(Number),
      };
    });

  const answer = tests
    .filter(({ result, numbers }) => is_valid(result, numbers))
    .reduce((acc, curr) => {
      return acc + curr.result;
    }, 0);

  console.log(answer);
}

function is_valid(result: number, numbers: number[], acc?: number): boolean {
  if (numbers.length === 0) {
    return acc === result;
  }

  const [first, ...rest] = numbers;

  if (acc === undefined) {
    return is_valid(result, rest, first);
  }

  return (
    is_valid(result, rest, acc + first) ||
    is_valid(result, rest, acc * first) ||
    is_valid(result, rest, Number(`${acc}${first}`))
  );
}

main();
