import { Capability, LightVM, VMEvent } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Debug] });
vm.on(VMEvent.Start, (data) => {
  console.log('Event: ', data.event);
  console.log('Payload: ', data.payload);
});
vm.run();
