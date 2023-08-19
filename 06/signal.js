import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_6_input.txt';
const fileData = readFileSync(inputPath, 'utf-8');

const charStream = fileData.trim();

// Part 1
let isStartPacket = false;
let i = 3;
while (!isStartPacket) {
    isStartPacket = true;
    for (let j = i - 2; j <= i; ++j) {
        for (let k = i - 3; k < j; ++k) {
            isStartPacket = isStartPacket && (charStream[k] != charStream[j]);
        }
    }
    ++i;
}

console.log(i);

// Part 2
isStartPacket = false;
i = 13;
while (!isStartPacket) {
    isStartPacket = true;
    for (let j = i - 12; j <= i; ++j) {
        for (let k = i - 13; k < j; ++k) {
            isStartPacket = isStartPacket && (charStream[k] != charStream[j]);
        }
    }
    ++i;
}

console.log(i);


// Both solutions are O(n), since there are a constant number of comparisons made.
// A map/set should be used once the DS overhead outweighs the number of checks.

// Though, say the length of the delimiter is c. Then an array hashmap would bring
// the constant number of operations down from c^2 to 2c.


// Part 1 (again)
let hashArray = new Array(26).fill(-Infinity);
let seqlen = 4;
let start = 0;
i = 0;
while ((i - start) < seqlen) {
    const charIndex = charStream.charCodeAt(i) - 'a'.charCodeAt(0);
    if (hashArray[charIndex] >= start) start = hashArray[charIndex] + 1;
    hashArray[charIndex] = i++;
}

console.log(i);

// Part 2 (again)
hashArray = hashArray.fill(-Infinity);
seqlen = 14;
start = 0;
i = 0;
while ((i - start) < seqlen) {
    const charIndex = charStream.charCodeAt(i) - 'a'.charCodeAt(0);
    if (hashArray[charIndex] >= start) start = hashArray[charIndex] + 1;
    hashArray[charIndex] = i++;
}

console.log(i);
