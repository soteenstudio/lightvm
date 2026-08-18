import {
  LightVM,
  Capability,
  VMEvent,
  TimeBudget,
} from '../dist/index.min.mjs';

function main() {
  const vm = new LightVM({
    caps: [Capability.Observe, Capability.Control, Capability.Unsafe],
  })
    .setTimeBudget(TimeBudget.Normal)
    .withNightly(false)
    .withHint(true)
    .withExplain(false)
    .withBacktrace(false);
  const raw = [['push', 5], ['push', 5], ['add', 'int'], ['println']];
  const str = `
push 5; ;; IP=0
push 5; ;; IP=1
add Int; ;; IP=2
println; ;; IP=3
  `;
  console.log(vm.tools().parseLTCArray(str));
  vm.load(vm.tools().optimizeBytecode(raw));
  console.log('Using time budget: Normal');
  const res = vm.run();
  console.log('Res: ', res);
  vm.halt();
  vm.run();
  vm.on(VMEvent.Halt, (payload) => {
    console.log('Halted: ', payload);
  });
  /*console.log('===> Execution finished <===');
  console.log('Output: ', res);*/
}
main();
