use std::env;
use std::path::PathBuf;

fn main() {
    // OLD - looking in wrong place
    // let modelica_core = PathBuf::from("space-colony-modelica-core");
    
    // NEW - point to lunco-sim apps structure
    let modelica_core = PathBuf::from("../../../lunco-sim/apps/modelica");
    
    // Rest of build script...
    let build_dir = modelica_core.join("build");
    
    // Check if SimpleThermalMVP exists
    let component_dir = build_dir.join("SimpleThermalMVP");
    if !component_dir.exists() {
        panic!(
            "SimpleThermalMVP.c not found in {}\n\
            Please run: cd lunco-sim/apps/modelica && ./build_models.sh",
            component_dir.display()
        );
    }
    
    // Continue with compilation...
    compile_component(&modelica_core, "SimpleThermalMVP", &omc_include, &omc_gc_include);
    generate_bindings(&modelica_core, "SimpleThermalMVP", &omc_include, &omc_gc_include);
}

fn compile_component(
    modelica_core: &Path,
    component: &str,
    omc_include: &str,
    omc_gc_include: &str,
) {
    println!("cargo:warning=Compiling Modelica component: {}", component);
    
    // Point to lunco-sim structure
    let build_dir = modelica_core.join("build").join(component);
    
    if !build_dir.exists() {
        panic!(
            "Component directory not found: {}\n\
            Build the models first: cd lunco-sim/apps/modelica && ./build_models.sh",
            build_dir.display()
        );
    }
    
    // Find all .c files except the main
    let c_files: Vec<_> = std::fs::read_dir(&build_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "c" 
                && !path.file_name()?.to_str()?.contains("_main.c") 
            {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    if c_files.is_empty() {
        panic!(
            "No C files found in {}\n\
            The model may not be compiled. Run: cd lunco-sim/apps/modelica && ./build_models.sh",
            build_dir.display()
        );
    }
    
    println!("cargo:warning=Found {} C files to compile", c_files.len());
    
    // Compile all C files
    let mut build = cc::Build::new();
    build
        .include(omc_include)
        .include(omc_gc_include)
        .include(&build_dir);
    
    for file in c_files {
        println!("cargo:warning=Compiling: {}", file.display());
        build.file(file);
    }
    
    build.compile(&format!("{}_modelica", component.to_lowercase()));
}

fn generate_bindings(
    modelica_core: &Path,
    component: &str,
    omc_include: &str,
    omc_gc_include: &str,
) {
    println!("cargo:warning=Generating bindings for: {}", component);
    
    let build_dir = modelica_core.join("build").join(component);
    let header_file = build_dir.join(format!("{}_model.h", component));
    
    if !header_file.exists() {
        panic!(
            "Header file not found: {}\n\
            Build the models first: cd lunco-sim/apps/modelica && ./build_models.sh",
            header_file.display()
        );
    }
    
    let bindings = bindgen::Builder::default()
        .header(header_file.to_str().unwrap())
        .clang_arg(format!("-I{}", omc_include))
        .clang_arg(format!("-I{}", omc_gc_include))
        .clang_arg(format!("-I{}", build_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(format!("{}_bindings.rs", component.to_lowercase())))
        .expect("Couldn't write bindings!");
    
    println!("cargo:warning=Bindings generated successfully");
}