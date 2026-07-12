import { LightVM, Capability } from '../dist/index.min.mjs';

function main() {
  const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] })
    .withNightly(false)
    .withHint(true)
    .withExplain(false)
    .withBacktrace(false);
  const raw = [['push', 5], ['push', 5], ['add', 'int'], ['println']];
  vm.load(vm.tools().optimizeBytecode(raw));
  console.log(vm.tools().stringifyLTC(raw));
  const res = vm.run();
  console.log('===> Execution finished <===');
  console.log('Output: ', res);
}
main();
