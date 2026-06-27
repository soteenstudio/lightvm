import { LightVM, Capability } from '../dist/index.min.mjs';

function main() {
  const vm = new LightVM({
    caps: [Capability.Observe, Capability.Control],
    nightly: false,
    explain: true,
    hint: true,
  });
  const raw = [['push', 5], ['add', 'int'], ['println'], ['instantiate']];
  vm.load(vm.tools().optimizeBytecode(raw));
  const res = vm.run();
  console.log('===> Execution finished <===');
  console.log('Output: ', res);
}
main();
