import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Debug] });
const tools = vm.tools();

const strVal = `
  push 5; ;; IP=0
  val x; ;; IP=1
  set x; ;; IP=2
`;
const parsed = tools.parseLTCArray(strVal);
console.log(parsed);
