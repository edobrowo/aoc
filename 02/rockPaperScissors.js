import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_2_input.txt';

const fileData = readFileSync(inputPath, 'utf-8');

const rounds = fileData.trim().split('\n').map((round) => round.split(' '));

const rpsScores = new Map([[2, 0], [0, 3], [1, 6]]);

const rpsWeights = new Map([['A', 0], ['B', 1], ['C', 2], ['X', 0], ['Y', 1], ['Z', 2]]);
const scores = rounds.map((round) => {
    const result = ((rpsWeights.get(round[1]) - rpsWeights.get(round[0])) % 3 + 3) % 3;
    return rpsScores.get(result) + rpsWeights.get(round[1]) + 1;
});

const totalScore = scores.reduce((sum, a) => sum + a);

const rpsShifts = new Map([['X', 2], ['Y', 0], ['Z', 1]]);
const scores2 = rounds.map((round) => {
    const choice = (rpsWeights.get(round[0]) + rpsShifts.get(round[1])) % 3;
    return rpsScores.get(rpsShifts.get(round[1])) + choice + 1;
});

const totalScore2 = scores2.reduce((sum, a) => sum + a);

// Part 1
console.log(totalScore);

// Part 2
console.log(totalScore2);
