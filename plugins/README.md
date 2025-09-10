# Built-in Plugins

Repository for built-in WASM plugins.

## Plugin API

Each plugin is a WebAssembly module. There's a hard-requirement for each
module to implement following APIs:

### 1. Export a `draw` function.

Each module needs to export a "draw" function with the following signature:

```ts
declare function draw(): Unit8Array
// Or to be more precise:
declare function draw(): pointer
```

#### Memory layout

The function MUST return a valid pointer to modules `memory`. The pointer
MUST point to the first item of an `Unit8Array`. Take a look at
[AssemblyScript Memory Layout for ArrayBuffer](https://www.assemblyscript.org/runtime.html#memory-layout) for
an example. The application will read `WIDTH*HEIGHT` in bytes length from the module `memory`,
starting at the returned pointer.

#### Array content

- Each element in the array can take a value ranging from 0-255. The value
  directly corresponds to the LED light brightness.
- The array is a contiguous representation of a 2D array
  and should be indexed using row-major order.
- Row/column correspond to module `height` and `width`, respectively.

### 2. Include a custom `metadata` section

Plugins MUST include a `"metadata"` [custom section](https://webassembly.github.io/spec/core/appendix/custom.html).
The section should be JSON-formatted string with the following format:

```json
{
  name: "module-name",
  // Module identifier
  width: 2,
  // Module height 
  height: 10
  // Module width
}
```

Each plugin can reserve a 2D space on the matrix of size `heigth * width`.
The `height` corresponds to the amount of rows a given plugin needs. The `width` and `height` correspond
to the amount of columns and rows the plugin will take, respectively.

## Building plugins

Have a look at `time`, `battery`, `cpu` and `memory` AssemblyScript npm packages.
The plugins can be built through anything that compiles to WASM, as long as Plugin API
is properly implemented.

To compile built-in plugins, install workspace dependencies:

```
pnpm install
```

And recursively run the `asbuild` script:

```
pnpm run -r asbuild
```

You'll find compiled WASM modules in `%package/build` directory.