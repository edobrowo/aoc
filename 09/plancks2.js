import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_9_input.txt';
const fileData = readFileSync(inputPath, 'utf-8');

class Step {
    constructor(ins, num) {
        this.ins = ins;
        this.num = num;
    }
}

const steps = fileData.trim().split('\n').map(step => {
    step = step.split(' ');
    return new Step(step[0], parseInt(step[1]));
});

// Part 2
const positions = new Set();

class Cell {
    constructor(x, y, leader) {
        this.x = x;
        this.y = y;
        this.leader = leader;
    }

    update() {
        const distance = Math.max(Math.abs(this.leader.x - this.x), Math.abs(this.leader.y - this.y));
        if (distance < 2) return;
        const dirX = this.leader.x - this.x;
        this.x += Math.abs(dirX) == 2 ? dirX / 2 : dirX;
        const dirY = this.leader.y - this.y;
        this.y += Math.abs(dirY) == 2 ? dirY / 2 : dirY;
    }
}

function updateAll(cells) {
    for (let i = 1; i < cells.length; ++i) {
        cells[i].update();
    }
    positions.add((cells[cells.length - 1].x + 50000) * 100000 + (cells[cells.length - 1].y + 50000));
}

function process(cells, steps) {
    const dx = new Map([['U', 0], ['D', 0], ['L', -1], ['R', 1]]);
    const dy = new Map([['U', 1], ['D', -1], ['L', 0], ['R', 0]]);
    for (let i = 0; i < steps.length; ++i) {
        const step = steps[i];
        for (let j = 0; j < step.num; ++j) {
            cells[0].x += dx.get(step.ins);
            cells[0].y += dy.get(step.ins);
            updateAll(cells);
        }
    }
}

const head = new Cell(0, 0, undefined);
const cells = [head];
const numCells = 10;

for (let i = 1; i < numCells; ++i) {
    cells.push(new Cell(0, 0, cells[i - 1]));
}

process(cells, steps);

let uniquePositions = positions.size;

console.log(uniquePositions);
