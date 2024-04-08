import { core } from "ext:core/mod.js";

const {
  op_hello_world,
} = core.ops

globalThis['Extensions'] = globalThis['Extensions'] || { ops: {} }
globalThis['Extensions'].hello_world = op_hello_world
