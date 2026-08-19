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
    .withNightly(true)
    .withHint(true)
    .withExplain(false)
    .withBacktrace(false);
  const raw = [
    ['jump', 5],
    ['func', 'say', 0, 2, 4],
    ['push', 'Hello from LightVM'],
    ['println'],
    ['stop'],
    ['export', 'say'],
  ];
  vm.load(vm.tools().optimizeBytecode(raw));
  const sayFunc = vm.export('say');
  console.log(sayFunc);
}
main();
