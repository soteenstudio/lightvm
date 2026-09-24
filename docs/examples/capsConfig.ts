import { LightVM, Capability } from "lightvm";

const vm = new LightVM({
  caps: [Capability.Control, Capability.Observe],
});
