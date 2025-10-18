use std::env;
use std::path::PathBuf;

fn main() {
    // Detect OS and set paths
    let omc_lib_search = if cfg!(target_os = "linux") {
        // Check multiple possible locations in Linux
        let arch = if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            "x86_64"
        };
        
        vec![
            "/usr/lib/omc".to_string(),
            format!("/usr/lib/{}-linux-gnu/omc", arch),
            "/usr/lib".to_string(),
            format!("/usr/lib/{}-linux-gnu", arch),
        ]
    } else if cfg!(target_os = "macos") {
        let omc_base = "/Applications/OpenModelica/build_cmake/install_cmake";
        vec![format!("{}/lib/omc", omc_base), format!("{}/lib", omc_base)]
    } else {
        panic!("Unsupported OS");
    };
    
    let (omc_include, omc_gc_include) = if cfg!(target_os = "linux") {
        ("/usr/include/omc/c".to_string(), "/usr/include/omc/gc".to_string())
    } else {
        let omc_base = "/Applications/OpenModelica/build_cmake/install_cmake";
        (format!("{}/include/omc/c", omc_base), format!("{}/include/omc/gc", omc_base))
    };
    
    // Add all library search paths
    for path in &omc_lib_search {
        if PathBuf::from(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
            println!("cargo:warning=Added library search path: {}", path);
        }
    }
    
    // Link libraries
    println!("cargo:rustc-link-lib=dylib=SimulationRuntimeC");
    println!("cargo:rustc-link-lib=dylib=OpenModelicaRuntimeC");
    println!("cargo:rustc-link-lib=dylib=omcgc");
    
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=lapack");
        println!("cargo:rustc-link-lib=blas");
    } else {
        println!("cargo:rustc-link-lib=dylib=lapack");
        println!("cargo:rustc-link-lib=dylib=blas");
    }
    println!("cargo:rustc-link-lib=pthread");
    
    println!("cargo:warning=Using OpenModelica libraries");
    
    let modelica_core = PathBuf::from("space-colony-modelica-core");
    
    // Compile components
    compile_component(&modelica_core, "SimpleThermalMVP", &omc_include, &omc_gc_include);
    
    // Generate bindings
    generate_bindings(&modelica_core, "SimpleThermalMVP", &omc_include, &omc_gc_include);
}

// ... rest stays the same
fn compile_component(
    modelica_core: &PathBuf, 
    component_name: &str,
    omc_include: &str,
    omc_gc_include: &str
) {
    let component_dir = modelica_core.join("build").join(component_name);
    
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
    
    let c_files: Vec<PathBuf> = std::fs::read_dir(&component_dir)
        .expect(&format!("Failed to read {}", component_dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let filename = path.file_name()?.to_str()?;
            
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
    
    let mut build = cc::Build::new();
    build
        .include(&component_dir)
        .include(omc_include)
        .include(omc_gc_include)
        .define("OPENMODELICA_XML_FROM_FILE_AT_RUNTIME", None)
        .warnings(false);
    
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