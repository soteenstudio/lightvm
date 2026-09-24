import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Control] });
vm.provide({
  name: 'John Doe',
  force: 2021,
});
const raw = [['get', 'name'], ['println'], ['get', 'force'], ['println']];
