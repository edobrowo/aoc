import { readFileSync } from 'fs';
import { CLIENT_RENEG_WINDOW } from 'tls';

const inputPath = 'adventofcode.com_2022_day_10_input.txt';
const fileData = readFileSync(inputPath, 'utf-8');

const program = fileData.trim().split('\n').map(ins => ins.split(' ').map(field => !isNaN(field) ? parseInt(field) : field));

// Part 1
class CPU {
    constructor() {
        this.cycle = 0;
        this.regX = 1;
        this.strength = 0;
    }

    nextCycle() {
        this.cycle++;
        if ((this.cycle + 20) % 40 == 0 && this.cycle <= 220) {
            this.strength += this.cycle * this.regX;
        }
    }

    run(program) {
        const actions = new Map(
            [
                ['noop', (v) => {
                    this.nextCycle();
                }],
                ['addx', (v) => {
                    this.nextCycle();
                    this.nextCycle();
                    this.regX += v;
                }]
            ]
        );

        for (let i = 0; i < program.length; ++i) {
            const instruction = program[i];
            const action = actions.get(instruction[0]);
            action(instruction[1]);
        }
    }
}

const cpu = new CPU();
cpu.run(program)

console.log(cpu.strength);

// Part 2
class CRT {
    constructor() {
        this.cycle = 0;
        this.regX = 1;
        this.screen = new Array(6).fill(0).map(() => new Array(40).fill('.'));
    }

    nextCycle() {
        this.cycle++;
        if (Math.abs((this.cycle - 1) % 40 - this.regX) <= 1) {
            this.screen[Math.floor((this.cycle - 1) / 40)][(this.cycle - 1) % 40] = '#';
        }
    }

    run(program) {
        const actions = new Map(
            [
                ['noop', (v) => {
                    this.nextCycle();
                }],
                ['addx', (v) => {
                    this.nextCycle();
                    this.nextCycle();
                    this.regX += v;
                }]
            ]
        );

        for (let i = 0; i < program.length; ++i) {
            const instruction = program[i];
            const action = actions.get(instruction[0]);
            action(instruction[1]);
        }
    }

    displayFrame() {
        const display = this.screen.map((line) => line.join('')).join('\n');
        console.log(display);
    }
}

const crt = new CRT();

crt.run(program);
crt.displayFrame()
