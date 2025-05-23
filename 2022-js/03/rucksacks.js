import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_3_input.txt'
const fileData = readFileSync(inputPath, 'utf-8');

function calcPriority(item) {
    const charCode = item.charCodeAt();
    const shift = (charCode >= 'a'.charCodeAt() ? -'a'.charCodeAt() : -'A'.charCodeAt() + 26) + 1;
    return charCode + shift;
}

function partitionEvenly(list, groupSize) {
    if (groupSize === 0) return list;
    let newList = [], i;
    for (i = 0; i < list.length - groupSize; i += groupSize) {
        newList.push(list.slice(i, i + groupSize));
    }
    newList.push(list.slice(i));
    return newList;
}

function findDuplicates(listOfArrays) {
    const duplicates = listOfArrays.reduce((dups, arr) => {
        arr = arr.filter((item) => dups.has(item));
        dups = new Set(arr);
        return dups;
    }, new Set(listOfArrays[0]));
    return Array.from(duplicates);
}

// Part 1
const rucksacks = fileData.trim().split('\n');

const compartments = rucksacks.map((rucksack) => {
    return [rucksack.substring(0, rucksack.length / 2),
            rucksack.substring(rucksack.length / 2)];
});

const duplicates = compartments.map((rucksack) => {
    return rucksack[0].split('').filter((item) => rucksack[1].includes(item))[0];
});

const sumPriorities1 = duplicates.reduce((sum, dup) => sum + calcPriority(dup), 0);

console.log(sumPriorities1);

// Part 2
const groups = partitionEvenly(rucksacks, 3).map((group) => group.map((rucksack) => rucksack.split('')));
const sumPriorities2 = groups.map((group) => calcPriority(findDuplicates(group)[0])).reduce((sum, prio) => sum + prio);

console.log(sumPriorities2);
