import { core } from "ext:core/mod.js";

const {
  op_register_callback,
} = core.ops

globalThis['Extensions'] = globalThis['Extensions'] || { ops: {} }
globalThis['Extensions'].register_callback = op_register_callback
