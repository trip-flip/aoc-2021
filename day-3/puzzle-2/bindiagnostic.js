const readline = require('readline');
const fs = require('fs');
const { exit } = require('process');

if (process.argv.length !== 3) return 1;
const fileName = process.argv[2];

const fileText = fs.readFileSync(fileName, 'utf-8');
const binNumbers = fileText.split('\n');

const oxygenRating = parseInt(findOxygenRating(binNumbers), 2);
const CO2Rating = parseInt(findCO2Rating(binNumbers), 2);
const result = oxygenRating * CO2Rating; 
console.log('The power consumption rate is:', result);

function findOxygenRating(data) {
    let binNumbers = data;
    const binLength = binNumbers[0].length;
    for(let i = 0; i < binLength; ++i) {
        if (binNumbers.length === 1) return binNumbers[0];
        let bitCount = { zero: 0, one: 0 };
        binNumbers.forEach(binNumber => {
            if (binNumber[i] === '0') {
                ++bitCount.zero;
            } else {
                ++bitCount.one;
            }
        });
        let mostCommonBit = 1;
        if (bitCount.zero > bitCount.one) mostCommonBit = 0;
        binNumbers = binNumbers.filter(binNumber => {
            if (binNumber[i] == mostCommonBit) {
                return true;
            } else return false;
        });
    }

    return binNumbers[0];
}

function findCO2Rating(data) {
    let binNumbers = data;
    const binLength = binNumbers[0].length;
    for(let i = 0; i < binLength; ++i) {
        if (binNumbers.length === 1) return binNumbers[0];
        let bitCount = { zero: 0, one: 0 };
        binNumbers.forEach(binNumber => {
            if (binNumber[i] === '0') {
                ++bitCount.zero;
            } else {
                ++bitCount.one;
            }
        });
        let leastCommonBit = 0;
        if (bitCount.zero > bitCount.one) leastCommonBit = 1;
        binNumbers = binNumbers.filter(binNumber => {
            if (binNumber[i] == leastCommonBit) {
                return true;
            } else return false;
        });
    }
}