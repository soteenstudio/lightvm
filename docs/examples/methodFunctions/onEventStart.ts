import { VMEvent } from 'lightvm';

vm.on(VMEvent.Start, (data) => {
  console.log('Event: ', data.event);
  console.log('Payload: ', data.payload);
});
vm.run();
