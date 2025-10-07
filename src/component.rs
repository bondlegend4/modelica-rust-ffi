use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Component initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Simulation step failed: {0}")]
    StepFailed(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Invalid output: {0}")]
    InvalidOutput(String),
}

pub type ComponentResult<T> = Result<T, ComponentError>;

/// Trait that all Modelica components must implement
pub trait SimulationComponent: Send + Sync {
    /// Unique identifier for this component type
    fn component_type(&self) -> &str;
    
    /// Initialize the component
    fn initialize(&mut self) -> ComponentResult<()>;
    
    /// Set input values
    fn set_input(&mut self, name: &str, value: f64) -> ComponentResult<()>;
    
    /// Set boolean input
    fn set_bool_input(&mut self, name: &str, value: bool) -> ComponentResult<()>;
    
    /// Get output value
    fn get_output(&self, name: &str) -> ComponentResult<f64>;
    
    /// Step the simulation forward by dt seconds
    fn step(&mut self, dt: f64) -> ComponentResult<()>;
    
    /// Reset component to initial state
    fn reset(&mut self) -> ComponentResult<()>;
    
    /// Get all outputs as a map
    fn get_all_outputs(&self) -> HashMap<String, f64> {
        HashMap::new() // Default implementation
    }
    
    /// Get component metadata
    fn metadata(&self) -> ComponentMetadata;
}

#[derive(Debug, Clone)]
pub struct ComponentMetadata {
    pub name: String,
    pub component_type: String,
    pub inputs: Vec<IOSpec>,
    pub outputs: Vec<IOSpec>,
}

#[derive(Debug, Clone)]
pub struct IOSpec {
    pub name: String,
    pub io_type: IOType,
    pub unit: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub enum IOType {
    Real,
    Boolean,
    Integer,
}