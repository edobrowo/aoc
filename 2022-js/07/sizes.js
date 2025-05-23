import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_7_input.txt';
const fileData = readFileSync(inputPath, 'utf-8');

const consoleOutput = fileData.trim().split('\n');

const EntryType = {
    File: 'file',
    Directory: 'directory',
};

class DirectoryEntry {
    constructor(name, type, size) {
        this.name = name;
        this.type = type;
        this.size = size;
        this.parent = null;
        this.children = [];
    }

    addEntry(entry) {
        entry.parent = this;
        this.children.push(entry);
    }

    getChild(name) {
        return this.children.find(dir => dir.name == name);
    }
}

// Part 1
const outputLines = consoleOutput.map(line => line.split(' '));
const root = new DirectoryEntry(outputLines[0][2], EntryType.Directory, 0);
outputLines.shift();

let current = root;
let i;

function lsHandler(line) {
    while (outputLines[++i] && outputLines[i][0] !== '$') {
        const line = outputLines[i];
        switch(line[0]) {
            case 'dir':
                current.addEntry(new DirectoryEntry(line[1], EntryType.Directory, 0));
                break;
            default:
                current.addEntry(new DirectoryEntry(line[1], EntryType.File, parseInt(line[0])));
                break;
        }
    }
    --i;
}

function cdHandler(line) {
    switch(line[2]) {
        case '/': current = root; break;
        case '..': current = current.parent; break;
        default: current = current.getChild(line[2]); break;
    }
}

const cmdMap = new Map([['ls', lsHandler], ['cd', cdHandler]]);

for (i = 0; i < outputLines.length; ++i) {
    const line = outputLines[i];
    const handler = cmdMap.get(line[1]);
    handler(line);
}

function calculateSizes(entry) {
    if (entry.type == EntryType.File) {
        return;
    }

    for (let j = 0; j < entry.children.length; ++j) {
        calculateSizes(entry.children[j]);
    }

    entry.size = entry.children.map(entry => entry.size).reduce((sum, val) => sum + val, 0);
}

function sizeUnder100000(entry) {
    if (entry.type == EntryType.File) {
        return 0;
    }

    let sum = 0;
    for (let j = 0; j < entry.children.length; ++j) {
        sum += sizeUnder100000(entry.children[j]);
    }

    if (entry.size <= 100000) {
        sum += entry.size;
    }

    return sum;
}

calculateSizes(root);

const sum = sizeUnder100000(root);
console.log(sum);

// Part 2
const totalDiskSpace = 70000000;
const updateDiskSpace = 30000000;
const freeDiskSpace = totalDiskSpace - root.size;
const requiredDiskSpace = updateDiskSpace - freeDiskSpace;

function minimumSizeOverRequired(entry) {
    if (entry.type == EntryType.File || entry.size < requiredDiskSpace) {
        return totalDiskSpace;
    }

    let minimum = totalDiskSpace;
    for (let j = 0; j < entry.children.length; ++j) {
        minimum = Math.min(minimum, minimumSizeOverRequired(entry.children[j]));
    }

    return Math.min(minimum, entry.size);
}

const minSize = minimumSizeOverRequired(root);
console.log(minSize);
