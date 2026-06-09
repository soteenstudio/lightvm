import { LightVM, Capability } from '../dist/index.min.mjs';

function main() {
  const vm = new LightVM([Capability.Observe, Capability.Control]);
  const raw = [
    ['val', 'x'],
    ['push', 5],
    ['set', 'x'],
    ['get', 'x'],
    ['println'],
  ];
  vm.load(vm.tools().optimizeBytecode(raw));
  const res = vm.run();
  console.log('===> Execution finished <===');
  console.log('Output: ', res);
}
main();
