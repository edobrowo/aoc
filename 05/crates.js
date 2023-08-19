import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_5_input.txt';
const fileData = readFileSync(inputPath, 'utf-8');

const [ cratesData, movesData ] = fileData.trim().split('\n\n');

function arrayDeepCopy(arr) {
    if (!Array.isArray(arr)) return;
    return arr.map(item => Array.isArray(item) ? arrayDeepCopy(item) : item);
}

// Parse stacks
const numStacks = cratesData.split('\n').slice(-1)[0].trim().split(/\s+/).length;
const levels = cratesData.split('\n').slice(0, -1);

const stacks = new Array(numStacks).fill().map(() => []);
const crateWidth = 4;
for (let i = levels.length - 1; i >= 0; --i) {
    const level = levels[i];
    for (let j = 0; j < numStacks; ++j) {
        const crate = level[j * crateWidth + 1];
        if (crate != ' ') stacks[j].push(crate);
    }
}

// Parse moves
const movesDataList = movesData.split('\n');
const ignoreList = ['move', 'from', 'to'];

const moves = new Array();
for (let i = 0; i < movesDataList.length; ++i) {
    const move = movesDataList[i].split(' ')
        .filter(token => !ignoreList.includes(token))
        .map(token => parseInt(token));
    moves.push(move);
}

// Part 1
const stacksP1 = arrayDeepCopy(stacks);
for (let i = 0; i < moves.length; ++i) {
    const [ num, from, to ] = moves[i];
    const toMove = stacksP1[from - 1].splice(-num).reverse();
    stacksP1[to - 1] = stacksP1[to - 1].concat(toMove);
}

const topsP1 = [];
for (let i = 0; i < stacksP1.length; ++i) {
    topsP1.push(stacksP1[i].slice(-1));
}
console.log(topsP1.join(''));

// Part 2
const stacksP2 = arrayDeepCopy(stacks);
for (let i = 0; i < moves.length; ++i) {
    const [ num, from, to ] = moves[i];
    const toMove = stacksP2[from - 1].splice(-num);
    stacksP2[to - 1] = stacksP2[to - 1].concat(toMove);
}

const topsP2 = [];
for (let i = 0; i < stacksP2.length; ++i) {
    topsP2.push(stacksP2[i].slice(-1));
}
console.log(topsP2.join(''));
