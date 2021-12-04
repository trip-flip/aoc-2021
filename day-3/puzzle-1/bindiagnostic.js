const readline = require('readline');
const fs = require('fs');
const { exit } = require('process');

if (process.argv.length !== 3) return 1;
const fileName = process.argv[2];

const fileText = fs.readFileSync(fileName, 'utf-8');
const binNumbers = fileText.split('\n');

// Fill an array with...
const bitCount = Array.from({ 
    length: binNumbers[0].length // ...length of bits per number with...
}, () => { 
    return { zero: 0, one: 0 }; // ...this object.
});
binNumbers.forEach(binNumber => {
    for (let i = 0; i < bitCount.length; ++i) {
        if (binNumber[i] === '0') {
            ++bitCount[i].zero;
        } else {
            ++bitCount[i].one;
        }
    }
});

const mostCommonBits = bitCount.map(counter => counter.one > counter.zero ? 0 : 1);
const leastCommonBits = mostCommonBits.map(bit => bit ? 0 : 1);

const gamma = parseInt(mostCommonBits.join(''), 2);
const epsilon = parseInt(leastCommonBits.join(''), 2);
const result = gamma * epsilon;
console.log('The power consumption rate is:', result);