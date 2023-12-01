import fs from 'fs';
import path from 'path';

const directory = path.resolve('../calibration-document.txt');
const file = fs.readFileSync(directory, 'utf8');
const lines = file.split('\n');

function doTheThing() {
	const replacedLines = lines.map(line => {
		return line
			.replace('one', 'one1one')
			.replace('two', 'two2two')
			.replace('three', 'three3three')
			.replace('four', 'four4four')
			.replace('five', 'five5five')
			.replace('six', 'six6six')
			.replace('seven', 'seven7seven')
			.replace('eight', 'eight8eight')
			.replace('nine', 'nine9nine')
			.split('')
			.filter(c => !isNaN(parseInt(c, 10)))
			.map(c => parseInt(c, 10))
			.join('');
	});

	const multipliedLines = replacedLines.filter(Boolean).map(line => {
		const firstNumber = parseInt(line[0], 10);
		const lastNumber = parseInt(line[line.length - 1], 10);
		return firstNumber * 10 + lastNumber;
	});

	return multipliedLines.reduce((a, b) => a + b, 0);
}

console.log(doTheThing());
