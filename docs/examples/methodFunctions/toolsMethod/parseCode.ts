const strVal = `
  push 5; ;; IP=0
  val x; ;; IP=1
  set x; ;; IP=2
`;
const parsed = tools.parseLTC(strVal);
console.log(parsed);
