import { VMEvent } from 'lightvm';

vm.on(VMEvent.Halt, (data) => {
  console.log('Event: ', data.event);
  console.log('Payload: ', data.payload);
});
vm.halt();
vm.run(); // will not be executed
