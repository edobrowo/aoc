import { readFileSync } from 'fs';

const inputPath = 'adventofcode.com_2022_day_8_input.txt';
const fileData = readFileSync(inputPath, 'utf-8');

const trees = fileData.trim().split('\n').map(line => line.split('').map(tree => parseInt(tree)));

const viewed = new Array(trees.length);
for (let i = 0; i < viewed.length; ++i) {
    viewed[i] = new Array(trees[0].length).fill(false);
}

// Part 1
let visibleTrees = trees.length * 2 + trees[0].length * 2 - 4;
// meh...
for (let i = 1; i < trees.length - 1; ++i) {
    // Right
    let j = 1, tree = trees[i][0];
    while (j < trees[i].length - 1) {
        if (trees[i][j] > tree) {
            tree = Math.max(tree, trees[i][j]);
            if (!viewed[i][j]) {
                ++visibleTrees;
                viewed[i][j] = true;
            }
        }
        if (trees[i][j] == 9) {
            break;
        }
        ++j;
    }

    // Left
    j = trees.length - 2, tree = trees[i][trees.length - 1];
    while (j > 0) {
        if (trees[i][j] > tree) {
            tree = Math.max(tree, trees[i][j]);
            if (!viewed[i][j]) {
                ++visibleTrees;
                viewed[i][j] = true;
            }
        }
        if (trees[i][j] == 9) {
            break;
        }
        --j;
    }

    // Down
    j = 1, tree = trees[0][i];
    while (j < trees.length - 1) {
        if (trees[j][i] > tree) {
            tree = Math.max(tree, trees[j][i]);
            if (!viewed[j][i]) {
                ++visibleTrees;
                viewed[j][i] = true;
            }
        }
        if (trees[j][i] == 9) {
            break;
        }
        ++j;
    }

    // Up
    j = trees.length - 2, tree = trees[trees.length - 1][i];
    while (j > 0) {
        if (trees[j][i] > tree) {
            tree = Math.max(tree, trees[j][i]);
            if (!viewed[j][i]) {
                ++visibleTrees;
                viewed[j][i] = true;
            }
        }
        if (trees[j][i] == 9) {
            break;
        }
        --j;
    }
}

console.log(visibleTrees);

// Part 2
const scores = new Array(trees.length);
for (let i = 0; i < scores.length; ++i) {
    scores[i] = new Array(trees[0].length).fill(1);
}

for (let i = 0; i < trees.length; ++i) {
    for (let j = 0; j < trees[i].length; ++j) {
        // Right
        let k = j + 1;
        while (k < trees[0].length && trees[i][k] < trees[i][j]) ++k;
        scores[i][j] *= ((k < trees[0].length) ? k - j : k - j - 1);

        // Left
        k = j - 1;
        while (k >= 0 && trees[i][k] < trees[i][j]) --k;
        scores[i][j] *= ((k >= 0) ? j - k : j - k - 1);

        // Up
        k = i + 1;
        while (k < trees.length && trees[k][j] < trees[i][j]) ++k;
        scores[i][j] *= ((k < trees.length) ? k - i : k - i -1);

        // Down
        k = i - 1;
        while (k >= 0 && trees[k][j] < trees[i][j]) --k;
        scores[i][j] *= ((k >= 0) ? i - k : i - k - 1);
    }
}

const maxScore = scores.map(row => row.reduce((a, b) => Math.max(a, b))).reduce((a, b) => Math.max(a, b));

console.log(maxScore);
