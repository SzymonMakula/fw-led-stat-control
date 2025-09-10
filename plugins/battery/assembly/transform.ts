// A compiler transform that runs after codegen and injects a custom section
import { Module } from "assemblyscript";
import { Transform } from "assemblyscript/transform";

export default class CustomSectionTransform extends Transform {
  afterCompile(module: Module): void {
    const meta = {
      name: "battery",
      width: 5,
      height: 10,
    };

    // Encode to UTF-8. (Binary data would work too—just provide a Uint8Array.)
    const payload = new TextEncoder().encode(JSON.stringify(meta));

    // Section name can be any string that doesn't collide with standard ones.
    module.addCustomSection("metadata", payload);
  }
}
