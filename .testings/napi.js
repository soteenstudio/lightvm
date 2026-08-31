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
  //console.log(tools);
  const raw = [
    ['jump', 7],
    ['func', 'add', 2, 2, 6, 'a', 'b'],
    ['get', 'a'],
    ['get', 'b'],
    ['add', 'int'],
    ['return'],
    ['stop'],
    ['export', 'add'],
    ['val', 'x'],
    ['push', 5],
    ['set', 'x'],
    ['export', 'x'],
    ['push', 5],
    ['println'],
  ];
  const optimized = tools.optimizeBytecode(raw);
  vm.load(optimized);
  const addFunc = vm.export('add');
  console.log(addFunc.call(5, 6));
  const xVar = vm.export('x');
  console.log(xVar.call());
  console.log(vm.info());

  vm.on(VMEvent.Tick, (data) => {
    console.log('Event: ', data.event);
    console.log('Payload: ', data.payload);
  });
  vm.run();
}
main();
