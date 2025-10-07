# modelica-rust-ffi
### Overview

Provides safe Rust bindings to compiled Modelica components. Creates a component-based architecture where each Modelica model becomes a reusable Rust component.

### Directory Structure

```
modelica-rust-ffi/
├── Cargo.toml
├── build.rs                      # Build script (compiles C, generates bindings)
├── src/
│   ├── lib.rs                   # Public API
│   ├── component.rs             # SimulationComponent trait
│   ├── registry.rs              # ComponentRegistry
│   └── components/
│       ├── mod.rs
│       └── simple_thermal.rs    # SimpleThermalMVP wrapper
├── space-colony-modelica-core/  # Git submodule (source of truth)
├── include/                      # Generated headers (build artifacts)
└── target/                       # Compiled output
```

### Architecture

#### Component Trait

Every Modelica model implements `SimulationComponent`:

```rust
pub trait SimulationComponent: Send + Sync {
    fn component_type(&self) -> &str;
    fn initialize(&mut self) -> ComponentResult<()>;
    fn set_input(&mut self, name: &str, value: f64) -> ComponentResult<()>;
    fn set_bool_input(&mut self, name: &str, value: bool) -> ComponentResult<()>;
    fn get_output(&self, name: &str) -> ComponentResult<f64>;
    fn step(&mut self, dt: f64) -> ComponentResult<()>;
    fn reset(&mut self) -> ComponentResult<()>;
    fn get_all_outputs(&self) -> HashMap<String, f64>;
    fn metadata(&self) -> ComponentMetadata;
}
```

#### Component Registry

Manages multiple active components:

```rust
pub struct ComponentRegistry {
    components: HashMap<Uuid, Box<dyn SimulationComponent>>,
    name_to_id: HashMap<String, Uuid>,
}
```

**Key Methods**:

- `add()` - Add component with auto-generated ID
- `add_component()` - Add component with specific ID
- `get()` / `get_mut()` - Access by ID
- `get_by_name()` / `get_mut_by_name()` - Access by name
- `step_all()` - Step all components forward in time
- `remove()` - Remove component

### Usage

#### Basic Example

```rust
use modelica_rust_ffi::{SimpleThermalComponent, SimulationComponent};

// Create component
let mut thermal = SimpleThermalComponent::new();
thermal.initialize()?;

// Set inputs
thermal.set_bool_input("heaterOn", true)?;

// Run simulation
for _ in 0..100 {
    thermal.step(0.1)?;  // 100ms timestep
}

// Read outputs
let temp = thermal.get_output("temperature")?;
println!("Temperature: {} K", temp);
```

#### Using Registry

```rust
use modelica_rust_ffi::{ComponentRegistry, SimpleThermalComponent};

let mut registry = ComponentRegistry::new();

// Add components
let id1 = registry.add("habitat_1".to_string(), Box::new(SimpleThermalComponent::new()))?;
let id2 = registry.add("habitat_2".to_string(), Box::new(SimpleThermalComponent::new()))?;

// Control individual components
if let Some(habitat) = registry.get_mut_by_name("habitat_1") {
    habitat.set_bool_input("heaterOn", true)?;
}

// Step all simulations
registry.step_all(0.1)?;

// Read outputs
for name in registry.list_names() {
    if let Some(comp) = registry.get_by_name(&name) {
        let temp = comp.get_output("temperature")?;
        println!("{}: {} K", name, temp);
    }
}
```

### Build Process

The `build.rs` script performs:

1. **Links OpenModelica runtime libraries**:
    
    - `libSimulationRuntimeC`
    - `libOpenModelicaRuntimeC`
    - `libomcgc` (garbage collector)
    - `liblapack`, `libblas` (linear algebra)
2. **Compiles Modelica-generated C code**:
    
    - Reads from `space-colony-modelica-core/build/ComponentName/`
    - Compiles all `*.c` files (except main)
    - Creates static library
3. **Generates Rust bindings**:
    
    - Uses `bindgen` on component headers
    - Creates type-safe Rust interfaces
    - Outputs to `target/debug/build/.../out/`

### Adding New Components

When you add a component to `space-colony-modelica-core`:

1. **Update `build.rs`**:

```rust
fn main() {
    // ... existing code ...
    
    // Add new component
    compile_component(&modelica_core, "SolarPanel", &omc_include, &omc_gc_include);
    generate_bindings(&modelica_core, "SolarPanel", &omc_include, &omc_gc_include);
}
```

2. **Create Rust wrapper** in `src/components/solar_panel.rs`:

```rust
use crate::component::*;

include!(concat!(env!("OUT_DIR"), "/solarpanel_bindings.rs"));

pub struct SolarPanelComponent {
    irradiance: f64,
    is_day: bool,
    power_output: f64,
}

impl SimulationComponent for SolarPanelComponent {
    // Implement trait methods...
}
```

3. **Export in `src/components/mod.rs`**:

```rust
pub mod simple_thermal;
pub mod solar_panel;  // Add this
```

4. **Export in `src/lib.rs`**:

```rust
pub use components::solar_panel::SolarPanelComponent;
```

### Dependencies

```toml
[dependencies]
libc = "0.2"
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4"] }

[build-dependencies]
bindgen = "0.70"
cc = "1.0"
```

### Platform Requirements

**macOS** (Apple Silicon or Intel):

- OpenModelica installed at `/Applications/OpenModelica/`
- Xcode Command Line Tools
- Clang/LLVM for bindgen

**Linux** (future):

- Update paths in `build.rs` to `/usr/lib/openmodelica/`

**Windows** (future):

- Update paths and linking for MSVC or MinGW

### Configuration

All paths are in `build.rs`:

```rust
let omc_base = "/Applications/OpenModelica/build_cmake/install_cmake";
let omc_lib = format!("{}/lib", omc_base);
let omc_include = format!("{}/include/omc/c", omc_base);
let omc_gc_include = format!("{}/include/omc/gc", omc_base);
```

Update these if your OpenModelica is installed elsewhere.

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_simple_thermal

# With output
cargo test -- --nocapture
```

### Integration with Parent Projects

This library is used as a **Git submodule** by:

- `modelica-rust-modbus-server` - Exposes simulation via Modbus TCP
- `godot-modelica-rust-integration` - Godot GDExtension for visualization

**Setup in parent project**:

```bash
cd your-project
git submodule add ../modelica-rust-ffi modelica-rust-ffi
git submodule update --init --recursive
```

**Update submodule**:

```bash
cd modelica-rust-ffi
git pull origin main
cd ..
git add modelica-rust-ffi
git commit -m "Update FFI submodule"
```

### Troubleshooting

**Problem**: `SimpleThermalMVP.c not found`

**Solution**: Build the Modelica component first:

```bash
cd space-colony-modelica-core
./scripts/build_component.sh SimpleThermalMVP
cd ../
cargo build
```

**Problem**: `gc.h file not found`

**Solution**: Check GC include path in `build.rs`:

```bash
ls /Applications/OpenModelica/build_cmake/install_cmake/include/omc/gc/gc.h
```

**Problem**: Linking errors with OpenModelica libraries

**Solution**: Verify libraries exist:

```bash
ls /Applications/OpenModelica/build_cmake/install_cmake/lib/omc/lib*.dylib
```

**Problem**: Lifetime errors in registry

**Solution**: Already fixed in current version. Use `&mut Box<dyn SimulationComponent>` return type.

### Performance Notes

- Components use simple Euler integration (currently in Rust)
- Future: Call actual OpenModelica solver for accuracy
- Each `step()` call advances simulation by `dt` seconds
- Registry steps all components sequentially (future: parallel)

### Future Enhancements

- [ ] Call actual OpenModelica ODE solvers
- [ ] Parallel component stepping
- [ ] Component dependency graph
- [ ] Resource flow between components
- [ ] State serialization/deserialization
- [ ] FMU export support
- [ ] Cross-platform builds (Linux, Windows)
---

## Development Workflow
### Full Build from Scratch

```bash
# 1. Build Modelica components
cd space-colony-modelica-core
./scripts/build_all.sh

# 2. Build Rust FFI
cd ../modelica-rust-ffi
cargo build

# 3. Run tests
cargo test

# 4. Use in parent project
cd ../your-modbus-server
cargo build
```

### Updating After Model Changes

```bash
# 1. Edit Modelica model
cd space-colony-modelica-core
vim models/SimpleThermalMVP.mo

# 2. Rebuild component
./scripts/build_component.sh SimpleThermalMVP

# 3. Rust picks up changes automatically
cd ../modelica-rust-ffi
cargo build
```

### Version Control Best Practices

**space-colony-modelica-core**:

- ✅ Commit: `.mo` files, scripts, README
- ❌ Don't commit: `build/` directory (gitignored)

**modelica-rust-ffi**:

- ✅ Commit: Rust source, Cargo.toml, build.rs
- ❌ Don't commit: `target/`, generated bindings
- ✅ Commit: Submodule reference (but not contents)

### License

Both submodules should inherit the license from the parent project. Ensure OpenModelica runtime usage complies with OSMC-PL and GPL licenses.