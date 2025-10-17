use std::env;
use std::path::PathBuf;

fn main() {
    let omc_base = "/Applications/OpenModelica/build_cmake/install_cmake";
    let omc_lib = format!("{}/lib", omc_base);
    let omc_lib_omc = format!("{}/lib/omc", omc_base);  // ADD THIS - where .dylib files are
    let omc_include = format!("{}/include/omc/c", omc_base);
    let omc_gc_include = format!("{}/include/omc/gc", omc_base);
    
    // Link OpenModelica runtime - search in BOTH lib and lib/omc
    println!("cargo:rustc-link-search=native={}", omc_lib_omc);  // PRIMARY - .dylib location
    println!("cargo:rustc-link-search=native={}", omc_lib);      // SECONDARY - for other libs
    
    println!("cargo:rustc-link-lib=dylib=SimulationRuntimeC");
    println!("cargo:rustc-link-lib=dylib=OpenModelicaRuntimeC");
    println!("cargo:rustc-link-lib=dylib=omcgc");
    println!("cargo:rustc-link-lib=dylib=lapack");
    println!("cargo:rustc-link-lib=dylib=blas");
    println!("cargo:rustc-link-lib=pthread");

    // Add rpath for BOTH directories
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", omc_lib_omc);
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", omc_lib);
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", omc_lib_omc);
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", omc_lib);
    }
    
    // Path to Modelica core (submodule)
    let modelica_core = PathBuf::from("space-colony-modelica-core");
    
    // Compile components
    compile_component(&modelica_core, "SimpleThermalMVP", &omc_include, &omc_gc_include);
    
    // Generate bindings
    generate_bindings(&modelica_core, "SimpleThermalMVP", &omc_include, &omc_gc_include);
}

// Rest of the code stays the same...
fn compile_component(
    modelica_core: &PathBuf, 
    component_name: &str,
    omc_include: &str,
    omc_gc_include: &str
) {
    let component_dir = modelica_core.join("build").join(component_name);
    
    // Check if component is built
    let main_c = component_dir.join(format!("{}.c", component_name));
    if !main_c.exists() {
        panic!(
            "{}.c not found in {}\n\
             Please run: cd space-colony-modelica-core && ./scripts/build_component.sh {}",
            component_name,
            component_dir.display(),
            component_name
        );
    }
    
    println!("cargo:warning=Compiling component: {}", component_name);
    
    // Find all C files EXCEPT the main file (which has main() function)
    let c_files: Vec<PathBuf> = std::fs::read_dir(&component_dir)
        .expect(&format!("Failed to read {}", component_dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let filename = path.file_name()?.to_str()?;
            
            // Include all C files EXCEPT:
            // - The main .c file (has main() and threading)
            // - Any main.c files
            if filename.starts_with(component_name) 
                && filename.ends_with(".c")
                && !filename.contains("main.c")
                && filename != format!("{}.c", component_name) {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    println!("cargo:warning=  Found {} C files for library (no main)", c_files.len());
    
    // Compile only the library functions (no main)
    let mut build = cc::Build::new();
    build
        .include(&component_dir)
        .include(omc_include)
        .include(omc_gc_include)
        .define("OPENMODELICA_XML_FROM_FILE_AT_RUNTIME", None)
        .warnings(false);
    
    // Only add the library C files (NOT the main file)
    for file in c_files {
        build.file(file);
    }
    
    build.compile(&format!("component_{}", component_name.to_lowercase()));
    println!("cargo:warning=  ✓ Compiled successfully");
}

fn generate_bindings(
    modelica_core: &PathBuf, 
    component_name: &str,
    omc_include: &str,
    omc_gc_include: &str
) {
    let component_dir = modelica_core.join("build").join(component_name);
    
    let model_header = component_dir.join(format!("{}_model.h", component_name));
    let functions_header = component_dir.join(format!("{}_functions.h", component_name));
    
    if !model_header.exists() {
        println!("cargo:warning=Header not found: {}", model_header.display());
        panic!("Headers not found for {}", component_name);
    }
    
    if !functions_header.exists() {
        println!("cargo:warning=Header not found: {}", functions_header.display());
        panic!("Headers not found for {}", component_name);
    }
    
    println!("cargo:warning=Generating bindings for {}", component_name);
    println!("cargo:warning=  Model header: {}", model_header.display());
    println!("cargo:warning=  Functions header: {}", functions_header.display());
    
    let bindings = bindgen::Builder::default()
        .header(model_header.to_str().unwrap())
        .header(functions_header.to_str().unwrap())
        .clang_arg(format!("-I{}", omc_include))
        .clang_arg(format!("-I{}", omc_gc_include))
        .clang_arg(format!("-I{}", component_dir.display()))
        .clang_arg("-DOPENMODELICA_XML_FROM_FILE_AT_RUNTIME")
        .allowlist_function(format!("{}_.*", component_name))
        .allowlist_type("DATA")
        .allowlist_type("threadData_t")
        .allowlist_type("MODEL_DATA")
        .allowlist_type("SIMULATION_INFO")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join(format!("{}_bindings.rs", component_name.to_lowercase()));
    
    println!("cargo:warning=Writing bindings to: {}", bindings_file.display());
    
    bindings
        .write_to_file(&bindings_file)
        .expect("Couldn't write bindings!");
    
    println!("cargo:warning=✓ Bindings generated successfully");
}