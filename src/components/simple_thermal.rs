use crate::component::*;
use crate::runtime::ModelicaRuntime;
use std::collections::HashMap;

pub struct SimpleThermalComponent {
    runtime: ModelicaRuntime,
}

impl SimpleThermalComponent {
    pub fn new() -> ComponentResult<Self> {
        Ok(Self {
            runtime: ModelicaRuntime::new("SimpleThermalMVP")?,
        })
    }
}

impl SimulationComponent for SimpleThermalComponent {
    fn component_type(&self) -> &str {
        "SimpleThermalMVP"
    }
    
    fn initialize(&mut self) -> ComponentResult<()> {
        self.runtime.reset()
    }
    
    fn set_input(&mut self, name: &str, value: f64) -> ComponentResult<()> {
        self.runtime.set_real_variable(name, value)
    }
    
    fn set_bool_input(&mut self, name: &str, value: bool) -> ComponentResult<()> {
        self.runtime.set_bool_variable(name, value)
    }
    
    fn get_output(&self, name: &str) -> ComponentResult<f64> {
        self.runtime.get_real_variable(name)
    }
    
    fn step(&mut self, dt: f64) -> ComponentResult<()> {
        self.runtime.step(dt)
    }
    
    fn reset(&mut self) -> ComponentResult<()> {
        self.runtime.reset()
    }
    
    fn get_all_outputs(&self) -> HashMap<String, f64> {
        let mut outputs = HashMap::new();
        if let Ok(temp) = self.runtime.get_real_variable("temperature") {
            outputs.insert("temperature".to_string(), temp);
        }
        if let Ok(status) = self.runtime.get_real_variable("heaterStatus") {
            outputs.insert("heaterStatus".to_string(), status);
        }
        outputs
    }
    
    fn metadata(&self) -> ComponentMetadata {
        ComponentMetadata {
            name: "SimpleThermalMVP".to_string(),
            component_type: "Thermal".to_string(),
            inputs: vec![
                IOSpec {
                    name: "heaterOn".to_string(),
                    io_type: IOType::Boolean,
                    unit: None,
                    description: Some("Heater control signal".to_string()),
                }
            ],
            outputs: vec![
                IOSpec {
                    name: "temperature".to_string(),
                    io_type: IOType::Real,
                    unit: Some("K".to_string()),
                    description: Some("Current room temperature".to_string()),
                },
                IOSpec {
                    name: "heaterStatus".to_string(),
                    io_type: IOType::Real,
                    unit: None,
                    description: Some("Heater status (0=off, 1=on)".to_string()),
                }
            ],
        }
    }
}

unsafe impl Send for SimpleThermalComponent {}
unsafe impl Sync for SimpleThermalComponent {}