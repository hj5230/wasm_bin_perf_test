const { string_to_md5 } = require('./pkg/test_rust.js');
// const { createHash } = require('crypto');

let timer = 0;

// function stringToMd5(input) {
//     let inputBytes = Buffer.from(input, 'utf8');
//     const encryptedAppendix = Buffer.from([0x2c, 0x2c, 0x31, 0x26]);
//     const appendixBytes = Buffer.from(encryptedAppendix.map(b => b ^ 0x55));
//     const combinedBuffer = Buffer.concat([inputBytes, appendixBytes]);
//     const digest = createHash('md5').update(combinedBuffer).digest('hex');
//     return digest;
// }

for (let i = 0; i < 10000; i++) {
    timer += string_to_md5();
}

// for (let i = 0; i < 10000; i++) {
//     stringToMd5(inputStr8x)
// }

console.warn(`Execution time: ${timer} ms`);
