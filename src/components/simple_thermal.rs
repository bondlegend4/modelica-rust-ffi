use crate::component::*;
use std::collections::HashMap;

// Include generated bindings
include!(concat!(env!("OUT_DIR"), "/simplethermalmvp_bindings.rs"));

pub struct SimpleThermalComponent {
    // Cached values (using Rust simulation for now)
    temperature: f64,
    heater_status: f64,
    heater_on: bool,
}

impl SimpleThermalComponent {
    pub fn new() -> Self {
        Self {
            temperature: 250.0,
            heater_status: 0.0,
            heater_on: false,
        }
    }
}

impl SimulationComponent for SimpleThermalComponent {
    fn component_type(&self) -> &str {
        "SimpleThermalMVP"
    }
    
    fn initialize(&mut self) -> ComponentResult<()> {
        self.temperature = 250.0;
        self.heater_status = 0.0;
        self.heater_on = false;
        Ok(())
    }
    
    fn set_input(&mut self, name: &str, _value: f64) -> ComponentResult<()> {
        Err(ComponentError::InvalidInput(
            format!("SimpleThermal has no real inputs. Got: {}", name)
        ))
    }
    
    fn set_bool_input(&mut self, name: &str, value: bool) -> ComponentResult<()> {
        match name {
            "heaterOn" => {
                self.heater_on = value;
                Ok(())
            }
            _ => Err(ComponentError::InvalidInput(
                format!("Unknown boolean input: {}", name)
            ))
        }
    }
    
    fn get_output(&self, name: &str) -> ComponentResult<f64> {
        match name {
            "temperature" => Ok(self.temperature),
            "heaterStatus" => Ok(self.heater_status),
            _ => Err(ComponentError::InvalidOutput(
                format!("Unknown output: {}", name)
            ))
        }
    }
    
    fn step(&mut self, dt: f64) -> ComponentResult<()> {
        // Simple Euler integration (Rust implementation for now)
        let room_capacity = 1000.0;
        let ambient_temp = 250.0;
        let heater_power = 500.0;
        let loss_coefficient = 2.0;
        
        let heating = if self.heater_on { heater_power } else { 0.0 };
        let losses = loss_coefficient * (self.temperature - ambient_temp);
        
        let d_temp = (heating - losses) / room_capacity * dt;
        self.temperature += d_temp;
        
        self.heater_status = if self.heater_on { 1.0 } else { 0.0 };
        
        Ok(())
    }
    
    fn reset(&mut self) -> ComponentResult<()> {
        self.temperature = 250.0;
        self.heater_status = 0.0;
        self.heater_on = false;
        Ok(())
    }
    
    fn get_all_outputs(&self) -> HashMap<String, f64> {
        let mut outputs = HashMap::new();
        outputs.insert("temperature".to_string(), self.temperature);
        outputs.insert("heaterStatus".to_string(), self.heater_status);
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