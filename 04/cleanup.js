import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_4_input.txt';
const fileData = readFileSync(inputPath, 'utf-8');

const intervalPairs = fileData.trim().split('\n').map((pair) => {
    return pair.split(',').map((interval) => interval.split('-').map((ep) => parseInt(ep)));
});

// Part 1

// Generalized to any number of intervals
function fullyContains(intervals) {
    const magn = (interval) => interval[1] - interval[0];
    intervals.sort((int1, int2) => (magn(int2) - magn(int1)));
    const result = intervals.slice(0, -1).reduce((acc, interval, ind) => {
        return acc && interval[0] <= intervals[ind + 1][0] && interval[1] >= intervals[ind + 1][1];
    }, true);
    return result;
}

const containedPairs = intervalPairs.reduce((sum, p) => sum + (fullyContains(p) ? 1 : 0), 0);

console.log(containedPairs);

// Part 2

// Not generalized :)
function overlaps(intervals) {
    return !(intervals[1][1] < intervals[0][0] || intervals[1][0] > intervals[0][1]);
}

const overlappingPairs = intervalPairs.reduce((sum, p) => sum + (overlaps(p) ? 1 : 0), 0);

console.log(overlappingPairs);
