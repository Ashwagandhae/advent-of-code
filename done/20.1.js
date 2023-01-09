const fs = require('fs');
const path = require('path');

// open file
let nums = fs
  .readFileSync(path.join(__dirname, '../data/20.txt'), 'utf8')
  .split('\n')
  .map((line) => parseInt(line))
  .map((num, index) => {
    return {
      num,
      index,
    };
  });

for (let i = 0; i < nums.length; i++) {
  // find current numObject by original index
  let index = nums.findIndex((num) => num.index == i);
  let num = nums[index];
  let offset = num.num;
  let newIndex = index + offset;
  if (newIndex <= 0) {
    newIndex = (newIndex % nums.length) - 1;
  }
  if (newIndex > nums.length + 1) {
    newIndex = (newIndex % nums.length) + 1;
  }
  nums.splice(index, 1);
  nums.splice(newIndex, 0, num);
  nums = nums.filter((val) => val !== null);
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
