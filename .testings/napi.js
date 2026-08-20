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
  const tools = vm.tools();
  const raw = [
    ['jump', 7],
    ['func', 'add', 2, 2, 6, 'a', 'b'],
    ['get', 'a'],
    ['get', 'b'],
    ['add', 'int'],
    ['return'],
    ['stop'],
    ['export', 'add'],
  ];
  const optimized = tools.optimizeBytecode(raw);
  vm.load(optimized);
  const addFunc = vm.export('add');
  console.log(addFunc(5, 6));
}
main();
