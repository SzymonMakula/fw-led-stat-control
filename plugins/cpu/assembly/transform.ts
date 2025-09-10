import { Module } from "assemblyscript";
import { Transform } from "assemblyscript/transform";

export default class CustomSectionTransform extends Transform {
  afterCompile(module: Module): void {
    const meta = {
      name: "cpu",
      width: 2,
      height: 10,
    };

    const payload = new TextEncoder().encode(JSON.stringify(meta));

    module.addCustomSection("metadata", payload);
  }
}
