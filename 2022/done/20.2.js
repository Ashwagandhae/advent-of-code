const fs = require('fs');
const path = require('path');

// open file
let nums = fs
  .readFileSync(path.join(__dirname, '../data/20.txt'), 'utf8')
  .split('\n')
  .map((line) => parseInt(line))
  .map((num, index) => {
    return {
      num: num * 811589153,
      index,
    };
  });

for (let j = 0; j < 10; j++) {
  console.log('mix: ', j);

  for (let i = 0; i < nums.length; i++) {
    // find current numObject by original index
    let index = nums.findIndex((num) => num.index == i);
    let num = nums[index];
    let offset = num.num;
    if (offset == 0) continue;
    let newIndex = index + offset;
    nums.splice(index, 1);
    // while (newIndex <= 0) {
    //   newIndex = nums.length + newIndex;
    // }
    // while (newIndex > nums.length + 1) {
    //   newIndex = newIndex - nums.length;
    // }
    newIndex = newIndex % nums.length;
    nums.splice(newIndex, 0, num);
    nums = nums.filter((val) => val !== null);
  }
}
let pos0 = nums.findIndex((num) => num.num == 0);
// get coords
let answer = [1000, 2000, 3000]
  .map((offset) => {
    let index = pos0 + offset;
    index = index % nums.length;
    return nums[index].num;
  })
  .reduce((acc, val) => acc + val);
console.log(answer);

// 0: 080
// 1: .080
// 2: 08.0
// 3: 0.80
// 4: .080
// 5: 08.0
// 6: 0.80
// 7: .080
// index: -1 or 3

// 0: 0.0
// 1: .00
// 2: 0.0
// 3: .00
// 4: 0.0
// 5: .00
// 6: 0.0
// 7: .00
// index: -1 or 3
