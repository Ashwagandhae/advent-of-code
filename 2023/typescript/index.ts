let text = await Bun.file('./input.txt').text();
let digits = '1234567890';
let answer = 0;
answer = text
  .split('\n')
  .map((line) => {
    let first = Array.from(line).find((c) => digits.includes(c))!;
    let second = Array.from(line)
      .reverse()
      .find((c) => digits.includes(c))!;
    return first + second;
  })
  .map((s) => parseInt(s))
  .reduce((acc, num) => acc + num, 0);
console.log(answer);
