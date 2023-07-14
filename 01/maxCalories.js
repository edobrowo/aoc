import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_1_input.txt';

const fileData = readFileSync(inputPath, 'utf8').trim();

const elves = fileData.split('\n\n')
    .map((caloriesList) => caloriesList.split('\n')
    .map((item) => parseInt(item)));

const sums = elves.map((caloriesList) => caloriesList.reduce((sum, cal) => sum + cal));

const max = sums.reduce((max, cal) => max > cal ? max : cal);

const top3 = sums.sort().slice(-3);
// Or: const top3 = sums.sort((a, b) => (b - a) / Math.abs(b - a)).slice(0, 3)

const sumTop3 = top3.reduce((sum, cal) => sum + cal);

// Part 1
console.log(max)

// Part 2
console.log(sumTop3)
