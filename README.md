# Framework LED Stat Controls

Display system metrics on a Framework Laptop
LED matrix hardware.

- **Plugin** system based on WebAssembly modules
- Built-in metrics for battery charge, CPU usage, memory usage and a binary GMT clock.
- Shipped as `systemd` daemon.

  
![output](https://github.com/user-attachments/assets/27f3c518-4535-4b34-a790-813fc15fbe49)

_My frail hands trying to hold the phone still_



## Getting started

### Fedora Linux

Download latest `.rpm` file from [Releases page](https://github.com/SzymonMakula/fw-led-stat-control/releases):

```
wget https://github.com/SzymonMakula/fw-led-stat-control/releases/download/v0.0.1/fw-led-stat-control-0.0.1-1.fc41.x86_64.rpm
```

Install it through DNF:

```
sudo dnf install fw-led-stat-control-0.0.1-1.fc41.x86_64.rpm
```

Enable systemd service:

```
sudo systemctl enable fw-led-stat-control
```

Start the systemd service:

```
sudo systemctl start fw-led-stat-control
```

### Other distros

You can find the binary at [Releases page](https://github.com/SzymonMakula/fw-led-stat-control/releases):

```
wget https://github.com/SzymonMakula/fw-led-stat-control/releases/download/v0.0.1/fw-led-stat-control-x86_64-unknown-linux-gnu.tar.xz
tar -xf fw-led-stat-control-x86_64-unknown-linux-gnu.tar.xz
./fw-led-stat-control
```

If you can't find a binary designated for your OS, take a look at "Local development" section to see how to compile a
release build yourself.

## Configuration

You can set and configure plugins through a `config.toml` file.
Have a look at `templates/config.toml` to get an idea on how should the file be
structured. There are 3 required fields per each plugin entry:

- `name` - Should match exactly the WASM plugin filename, i.e. `%name.wasm`
- `pos_x` - Horizontal position on the LED matrix board. Starts with 0, ends at 8.
- `pos_y` - Vertical position on the LED matrix board. Stars at 0, ends at 34.

Each plugin takes up a rectangular space on the 2D matrix. The `pos_x` and `pos_y` designate
a starting position of that space. By tweaking the plugin starting positions, you can re-arrange
the display position for each plugin, e.g. move binary clock to the bottom,
battery to the middle, etc. Plugins space can't intersect - malformed configuration files
will be rejected.

## Local development

### Building in debug mode

You can compile the project by running:

```
cargo build
```

You'll find the compiled artifacts in `target` directory. You'll need both the
configuration file and plugin modules copied there.

You can copy over the config template to the debug target:

```
cp templates/config.toml target/debug
```

And the associated plugins:

```
plugins/scripts/bundle_plugins.sh
tar -xf plugins/release/plugins.tar.xz -C target/debug
```

You may then run the project in a debug mode:

```
cargo run
```

### Building for release

See `scripts/bundle_release.sh` for how to compile and package
the application for a release build.

## Plugin development

Take a look at `plugins` directory for examples.
You'll also find basic documentation at plugins/README.md.
