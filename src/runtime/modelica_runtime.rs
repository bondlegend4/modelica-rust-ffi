use crate::component::{ComponentError, ComponentResult};
use std::collections::HashMap;
// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/simplethermalmvp_bindings.rs"));

/// Safe wrapper around OpenModelica runtime structures
/// 
/// This struct manages the lifecycle of OpenModelica DATA and threadData_t
/// structures, ensuring proper initialization and cleanup.
/// 
/// # Safety
/// 
/// While this struct uses unsafe code internally, it provides a 100% safe
/// public API. All unsafe operations are carefully encapsulated and validated.
/// 
/// # Examples
/// 
/// ```no_run
/// use modelica_rust_ffi::ModelicaRuntime;
/// 
/// let mut runtime = ModelicaRuntime::new("SimpleThermalMVP")?;
/// runtime.set_bool_variable("heaterOn", true)?;
/// runtime.step(0.1)?;
/// let temp = runtime.get_real_variable("temperature")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct ModelicaRuntime {
    component_name: String,
    // Currently using simplified simulation
    // TODO: Replace with actual OpenModelica pointers when ready
    // data: *mut DATA,
    // thread_data: *mut threadData_t,
    
    // Temporary: Rust-based state
    real_vars: std::collections::HashMap<String, f64>,
    bool_vars: std::collections::HashMap<String, bool>,
    time: f64,
}

impl ModelicaRuntime {
    /// Creates a new ModelicaRuntime instance
    /// 
    /// # Arguments
    /// 
    /// * `component_name` - Name of the Modelica component (e.g., "SimpleThermalMVP")
    /// 
    /// # Errors
    /// 
    /// Returns `ComponentError::InitializationFailed` if:
    /// - Memory allocation fails
    /// - OpenModelica initialization fails
    /// - Component name is invalid
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use modelica_rust_ffi::ModelicaRuntime;
    /// let runtime = ModelicaRuntime::new("SimpleThermalMVP")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(component_name: &str) -> ComponentResult<Self> {
        // Validate component name
        if component_name.is_empty() {
            return Err(ComponentError::InitializationFailed(
                "Component name cannot be empty".to_string()
            ));
        }
        
        // TODO: Initialize actual OpenModelica runtime
        // For now, create simplified runtime
        
        let mut real_vars = std::collections::HashMap::new();
        let mut bool_vars = std::collections::HashMap::new();
        
        // Initialize based on component type
        match component_name {
            "SimpleThermalMVP" => {
                // Initialize state variables
                real_vars.insert("roomTemp".to_string(), 250.0);
                real_vars.insert("temperature".to_string(), 250.0);
                real_vars.insert("heaterStatus".to_string(), 0.0);
                
                // Initialize parameters
                real_vars.insert("roomCapacity".to_string(), 1000.0);
                real_vars.insert("ambientTemp".to_string(), 250.0);
                real_vars.insert("heaterPower".to_string(), 500.0);
                real_vars.insert("lossCoefficient".to_string(), 2.0);
                
                // Initialize inputs
                bool_vars.insert("heaterOn".to_string(), false);
            }
            _ => {
                return Err(ComponentError::InitializationFailed(
                    format!("Unknown component: {}", component_name)
                ));
            }
        }
        
        Ok(Self {
            component_name: component_name.to_string(),
            real_vars,
            bool_vars,
            time: 0.0,
        })
    }
    
    /// Advances the simulation by the given time step
    /// 
    /// # Arguments
    /// 
    /// * `dt` - Time step in seconds (must be positive and finite)
    /// 
    /// # Errors
    /// 
    /// Returns `ComponentError::StepFailed` if:
    /// - Time step is invalid (negative, zero, infinite, or NaN)
    /// - Simulation equations fail to converge
    /// - Runtime error occurs
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use modelica_rust_ffi::ModelicaRuntime;
    /// # let mut runtime = ModelicaRuntime::new("SimpleThermalMVP")?;
    /// // Advance by 100ms
    /// runtime.step(0.1)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn step(&mut self, dt: f64) -> ComponentResult<()> {
        // Validate timestep
        if dt <= 0.0 || !dt.is_finite() {
            return Err(ComponentError::StepFailed(
                format!("Invalid timestep: {}. Must be positive and finite.", dt)
            ));
        }
        
        // TODO: Call actual OpenModelica step function
        // For now, implement simple Euler integration
        
        match self.component_name.as_str() {
            "SimpleThermalMVP" => {
                self.step_simple_thermal(dt)?;
            }
            _ => {
                return Err(ComponentError::StepFailed(
                    format!("Component {} has no step implementation", self.component_name)
                ));
            }
        }
        
        self.time += dt;
        Ok(())
    }
    
    /// Internal: Step SimpleThermalMVP simulation
    fn step_simple_thermal(&mut self, dt: f64) -> ComponentResult<()> {
        // Get state and parameters
        let room_temp = self.get_real_variable("roomTemp")?;
        let room_capacity = self.get_real_variable("roomCapacity")?;
        let ambient_temp = self.get_real_variable("ambientTemp")?;
        let heater_power = self.get_real_variable("heaterPower")?;
        let loss_coefficient = self.get_real_variable("lossCoefficient")?;
        let heater_on = self.get_bool_variable("heaterOn")?;
        
        // Calculate heating and losses
        let heating = if heater_on { heater_power } else { 0.0 };
        let losses = loss_coefficient * (room_temp - ambient_temp);
        
        // Euler integration: dT/dt = (heating - losses) / capacity
        let d_temp = (heating - losses) / room_capacity * dt;
        let new_temp = room_temp + d_temp;
        
        // Validate result
        if !new_temp.is_finite() {
            return Err(ComponentError::StepFailed(
                "Temperature calculation resulted in non-finite value".to_string()
            ));
        }
        
        // Update state
        self.set_real_variable("roomTemp", new_temp)?;
        self.set_real_variable("temperature", new_temp)?;
        self.set_real_variable("heaterStatus", if heater_on { 1.0 } else { 0.0 })?;
        
        Ok(())
    }
    
    /// Gets the value of a real variable
    /// 
    /// # Arguments
    /// 
    /// * `name` - Variable name
    /// 
    /// # Errors
    /// 
    /// Returns `ComponentError::VariableNotFound` if variable doesn't exist
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use modelica_rust_ffi::ModelicaRuntime;
    /// # let runtime = ModelicaRuntime::new("SimpleThermalMVP")?;
    /// let temp = runtime.get_real_variable("temperature")?;
    /// println!("Temperature: {} K", temp);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_real_variable(&self, name: &str) -> ComponentResult<f64> {
        self.real_vars.get(name)
            .copied()
            .ok_or_else(|| ComponentError::VariableNotFound(name.to_string()))
    }
    
    /// Sets the value of a real variable with bounds checking
    /// 
    /// # Arguments
    /// 
    /// * `name` - Variable name
    /// * `value` - New value (must be finite)
    /// 
    /// # Errors
    /// 
    /// Returns error if:
    /// - Variable doesn't exist
    /// - Value is not finite (NaN or infinite)
    /// - Value is outside valid bounds (if bounds exist)
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use modelica_rust_ffi::ModelicaRuntime;
    /// # let mut runtime = ModelicaRuntime::new("SimpleThermalMVP")?;
    /// runtime.set_real_variable("roomTemp", 273.15)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn set_real_variable(&mut self, name: &str, value: f64) -> ComponentResult<()> {
        // Validate value
        if !value.is_finite() {
            return Err(ComponentError::InvalidInput(
                format!("Value for '{}' must be finite, got: {}", name, value)
            ));
        }
        
        // Check if variable exists
        if !self.real_vars.contains_key(name) {
            return Err(ComponentError::VariableNotFound(name.to_string()));
        }
        
        // TODO: Add bounds checking based on Modelica variable attributes
        // For now, just basic sanity checks
        match name {
            "temperature" | "roomTemp" => {
                if value < 0.0 || value > 1000.0 {
                    return Err(ComponentError::BoundsCheckFailed(
                        name.to_string(), value, 0.0, 1000.0
                    ));
                }
            }
            _ => {}
        }
        
        self.real_vars.insert(name.to_string(), value);
        Ok(())
    }
    
    /// Gets the value of a boolean variable
    /// 
    /// # Arguments
    /// 
    /// * `name` - Variable name
    /// 
    /// # Errors
    /// 
    /// Returns `ComponentError::VariableNotFound` if variable doesn't exist
    pub fn get_bool_variable(&self, name: &str) -> ComponentResult<bool> {
        self.bool_vars.get(name)
            .copied()
            .ok_or_else(|| ComponentError::VariableNotFound(name.to_string()))
    }
    
    /// Sets the value of a boolean variable
    /// 
    /// # Arguments
    /// 
    /// * `name` - Variable name
    /// * `value` - New value
    /// 
    /// # Errors
    /// 
    /// Returns `ComponentError::VariableNotFound` if variable doesn't exist
    pub fn set_bool_variable(&mut self, name: &str, value: bool) -> ComponentResult<()> {
        if !self.bool_vars.contains_key(name) {
            return Err(ComponentError::VariableNotFound(name.to_string()));
        }
        
        self.bool_vars.insert(name.to_string(), value);
        Ok(())
    }
    
    /// Gets the current simulation time
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use modelica_rust_ffi::ModelicaRuntime;
    /// # let runtime = ModelicaRuntime::new("SimpleThermalMVP")?;
    /// let t = runtime.time();
    /// println!("Simulation time: {} s", t);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn time(&self) -> f64 {
        self.time
    }
    
    /// Resets the simulation to initial conditions
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use modelica_rust_ffi::ModelicaRuntime;
    /// # let mut runtime = ModelicaRuntime::new("SimpleThermalMVP")?;
    /// runtime.step(1.0)?;
    /// runtime.reset()?;
    /// assert_eq!(runtime.time(), 0.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn reset(&mut self) -> ComponentResult<()> {
        // Reset to initial state
        match self.component_name.as_str() {
            "SimpleThermalMVP" => {
                let ambient = self.real_vars.get("ambientTemp").copied().unwrap_or(250.0);
                self.set_real_variable("roomTemp", ambient)?;
                self.set_real_variable("temperature", ambient)?;
                self.set_real_variable("heaterStatus", 0.0)?;
                self.set_bool_variable("heaterOn", false)?;
            }
            _ => {}
        }
        
        self.time = 0.0;
        Ok(())
    }
    
    /// Gets the component name
    pub fn component_name(&self) -> &str {
        &self.component_name
    }
}

impl Drop for ModelicaRuntime {
    /// Automatically cleans up OpenModelica resources
    /// 
    /// This ensures proper cleanup even if the runtime is dropped due to panic
    /// or early return.
    fn drop(&mut self) {
        // TODO: Call OpenModelica cleanup functions
        // For now, Rust HashMap cleanup is automatic
    }
}

impl std::fmt::Debug for ModelicaRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelicaRuntime")
            .field("component_name", &self.component_name)
            .field("time", &self.time)
            .field("real_vars_count", &self.real_vars.len())
            .field("bool_vars_count", &self.bool_vars.len())
            .finish()
    }
}

impl std::fmt::Display for ModelicaRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModelicaRuntime({}, t={}s)", self.component_name, self.time)
    }
}

// Safe to send between threads (will add proper synchronization later)
unsafe impl Send for ModelicaRuntime {}

// TODO: Implement Sync with proper mutex protection
// For now, only Send is safe