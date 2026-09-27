import { LightVM, VMEvent } from 'lightvm';

const vm = new LightVM({ caps: [] });

vm.on(VMEvent.Halt, (data) => {
  console.log('Event: ', data.event);
  console.log('Payload: ', data.payload);
});
vm.halt();
vm.run(); // will not be executed
