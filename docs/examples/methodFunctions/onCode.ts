import { VMEvent } from 'lightvm';

vm.on(VMEvent.Halt, (payload) => {
  console.log('Halted: ', payload);
});
