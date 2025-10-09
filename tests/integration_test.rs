use modelica_rust_ffi::*;

#[test]
fn test_runtime_creation() {
    let runtime = ModelicaRuntime::new("SimpleThermalMVP");
    assert!(runtime.is_ok());
    
    let runtime = runtime.unwrap();
    assert_eq!(runtime.component_name(), "SimpleThermalMVP");
    assert_eq!(runtime.time(), 0.0);
}

#[test]
fn test_runtime_invalid_component() {
    let runtime = ModelicaRuntime::new("InvalidComponent");
    assert!(runtime.is_err());
}

#[test]
fn test_runtime_empty_name() {
    let runtime = ModelicaRuntime::new("");
    assert!(runtime.is_err());
}

#[test]
fn test_get_set_real_variable() {
    let mut runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    // Get initial value
    let temp = runtime.get_real_variable("temperature").unwrap();
    assert_eq!(temp, 250.0);
    
    // Set new value
    runtime.set_real_variable("roomTemp", 300.0).unwrap();
    let temp = runtime.get_real_variable("roomTemp").unwrap();
    assert_eq!(temp, 300.0);
}

#[test]
fn test_get_set_bool_variable() {
    let mut runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    // Get initial value
    let heater = runtime.get_bool_variable("heaterOn").unwrap();
    assert_eq!(heater, false);
    
    // Set new value
    runtime.set_bool_variable("heaterOn", true).unwrap();
    let heater = runtime.get_bool_variable("heaterOn").unwrap();
    assert_eq!(heater, true);
}

#[test]
fn test_variable_not_found() {
    let runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    let result = runtime.get_real_variable("nonexistent");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ComponentError::VariableNotFound(_)));
}

#[test]
fn test_bounds_checking() {
    let mut runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    // Try to set temperature outside valid range
    let result = runtime.set_real_variable("temperature", -100.0);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ComponentError::BoundsCheckFailed(_, _, _, _)));
    
    let result = runtime.set_real_variable("temperature", 2000.0);
    assert!(result.is_err());
}

#[test]
fn test_invalid_timestep() {
    let mut runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    // Negative timestep
    assert!(runtime.step(-0.1).is_err());
    
    // Zero timestep
    assert!(runtime.step(0.0).is_err());
    
    // Infinite timestep
    assert!(runtime.step(f64::INFINITY).is_err());
    
    // NaN timestep
    assert!(runtime.step(f64::NAN).is_err());
}

#[test]
fn test_simulation_step() {
    let mut runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    // Initial temperature
    let temp0 = runtime.get_real_variable("temperature").unwrap();
    assert_eq!(temp0, 250.0);
    
    // Turn on heater
    runtime.set_bool_variable("heaterOn", true).unwrap();
    
    // Step simulation
    runtime.step(0.1).unwrap();
    
    // Temperature should increase
    let temp1 = runtime.get_real_variable("temperature").unwrap();
    assert!(temp1 > temp0);
    
    // Time should advance
    assert_eq!(runtime.time(), 0.1);
}

#[test]
fn test_simulation_cooling() {
    let mut runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    // Set high initial temperature
    runtime.set_real_variable("roomTemp", 300.0).unwrap();
    runtime.set_real_variable("temperature", 300.0).unwrap();
    
    // Heater off
    runtime.set_bool_variable("heaterOn", false).unwrap();
    
    // Step simulation
    runtime.step(0.1).unwrap();
    
    // Temperature should decrease (cooling)
    let temp = runtime.get_real_variable("temperature").unwrap();
    assert!(temp < 300.0);
}

#[test]
fn test_reset() {
    let mut runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    // Change state
    runtime.set_bool_variable("heaterOn", true).unwrap();
    runtime.step(1.0).unwrap();
    
    let temp_before = runtime.get_real_variable("temperature").unwrap();
    assert_ne!(temp_before, 250.0);
    assert_eq!(runtime.time(), 1.0);
    
    // Reset
    runtime.reset().unwrap();
    
    // Should be back to initial state
    let temp_after = runtime.get_real_variable("temperature").unwrap();
    assert_eq!(temp_after, 250.0);
    assert_eq!(runtime.time(), 0.0);
    
    let heater = runtime.get_bool_variable("heaterOn").unwrap();
    assert_eq!(heater, false);
}

#[test]
fn test_component_with_runtime() {
    let mut component = SimpleThermalComponent::new().unwrap();
    component.initialize().unwrap();
    
    // Set input
    component.set_bool_input("heaterOn", true).unwrap();
    
    // Step
    for _ in 0..10 {
        component.step(0.1).unwrap();
    }
    
    // Check output
    let temp = component.get_output("temperature").unwrap();
    assert!(temp > 250.0);
}

#[test]
fn test_no_panic_on_error() {
    let mut runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    
    // These should return errors, not panic
    let _ = runtime.step(-1.0);
    let _ = runtime.get_real_variable("invalid");
    let _ = runtime.set_real_variable("temperature", f64::NAN);
}

#[test]
fn test_display_trait() {
    let runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    let display = format!("{}", runtime);
    assert!(display.contains("SimpleThermalMVP"));
    assert!(display.contains("t=0"));
}

#[test]
fn test_debug_trait() {
    let runtime = ModelicaRuntime::new("SimpleThermalMVP").unwrap();
    let debug = format!("{:?}", runtime);
    assert!(debug.contains("ModelicaRuntime"));
    assert!(debug.contains("component_name"));
}