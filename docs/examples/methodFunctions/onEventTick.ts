import { LightVM, VMEvent } from 'lightvm';

const vm = new LightVM({ caps: [] });

vm.on(VMEvent.Tick, (data) => {
  console.log('Event: ', data.event);
  console.log('Payload: ', data.payload);
});
vm.run();
