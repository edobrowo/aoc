const set0 = new Set([1, 2, 3]);
console.log(set0);

set0.add(1);
set0.add(2);
set0.add(4);
console.log(set0);

const list = [1, 2, 3, 4, 5];
const set1 = new Set(list);
console.log(set1);

set1.delete(1);
set1.delete(5);
console.log(set1);

console.log(set1.has(1));
console.log(set1.has(2));
console.log(set1.has(3));
console.log(set1.has(4));
console.log(set1.has(5));
