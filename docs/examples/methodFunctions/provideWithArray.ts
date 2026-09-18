import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Debug] });
vm.provide({
  name: 'John Doe',
  force: 2021,
});
const raw = [['get', 'name'], ['println'], ['get', 'force'], ['println']];
