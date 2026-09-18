import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Debug] });
const tools = vm.tools();

tools
  .bench('test_bench')
  .bytes(512)
  .samples(20)
  .targetTime(100)
  .run(
    () => [0.5, 6.7, 8.9],
    (state) => tools.blackBox(state),
  );
