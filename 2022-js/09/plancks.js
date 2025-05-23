import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_9_input.txt';
const fileData = readFileSync(inputPath, 'utf-8');

// Convenience classes
class Step {
    constructor(ins, num) {
        this.ins = ins;
        this.num = num;
    }
}

class Pos2d {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}

const steps = fileData.trim().split('\n').map(step => {
    step = step.split(' ');
    return new Step(step[0], parseInt(step[1]));
});

// Part 1
const head = new Pos2d(0, 0);
const tail = new Pos2d(0, 0);

const positions = new Set();
// :(
positions.add(JSON.stringify(new Pos2d(tail.x, tail.y)));

for (let i = 0; i < steps.length; ++i) {
    const step = steps[i];
    // meh...
    switch(step.ins) {
        case 'D':
            head.y -= step.num;
            if (Math.abs(head.y - tail.y) > 1) tail.x = head.x;
            while (tail.y > head.y + 1) {
                --tail.y;
                positions.add(JSON.stringify(new Pos2d(tail.x, tail.y)));
            }
            break;
        case 'U':
            head.y += step.num;
            if (Math.abs(head.y - tail.y) > 1) tail.x = head.x;
            while (tail.y < head.y - 1) {
                ++tail.y;
                positions.add(JSON.stringify(new Pos2d(tail.x, tail.y)));
            }
            break;
        case 'L':
            head.x -= step.num;
            if (Math.abs(head.x - tail.x) > 1) tail.y = head.y;
            while (tail.x > head.x + 1) {
                --tail.x;
                positions.add(JSON.stringify(new Pos2d(tail.x, tail.y)));
            }
            break;
        case 'R':
            head.x += step.num;
            if (Math.abs(head.x - tail.x) > 1) tail.y = head.y;
            while (tail.x < head.x - 1) {
                ++tail.x;
                positions.add(JSON.stringify(new Pos2d(tail.x, tail.y)));
            }
            break;
    }
}

const uniquePositions = positions.size;

console.log(uniquePositions);

// Part 2
// >:)
