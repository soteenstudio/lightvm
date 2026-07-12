const raw = [
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
];
const stringify = tools.stringifyLTC(raw);
console.log(stringify);
