#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod component;
pub mod registry;
pub mod runtime;  // Add this
pub mod components;

pub use component::{SimulationComponent, ComponentError, ComponentResult, ComponentMetadata, IOSpec, IOType};
pub use registry::ComponentRegistry;
pub use runtime::ModelicaRuntime;  // Add this
pub use components::simple_thermal::SimpleThermalComponent;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_thermal() {
        let mut component = SimpleThermalComponent::new().unwrap();  // Add .unwrap()
        component.initialize().unwrap();
        
        // Test initial state
        assert_eq!(component.get_output("temperature").unwrap(), 250.0);
        
        // Turn heater on
        component.set_bool_input("heaterOn", true).unwrap();
        
        // Step simulation
        for _ in 0..100 {
            component.step(0.1).unwrap();
        }
        
        // Temperature should have increased
        assert!(component.get_output("temperature").unwrap() > 250.0);
    }
    
    #[test]
    fn test_registry() {
        let mut registry = ComponentRegistry::new();
        
        let component = Box::new(SimpleThermalComponent::new().unwrap());  // Add .unwrap()
        let id = registry.add("thermal_1".to_string(), component).unwrap();
        
        // Access by name
        let comp = registry.get_by_name("thermal_1").unwrap();
        assert_eq!(comp.component_type(), "SimpleThermalMVP");
        
        // Remove
        registry.remove(id).unwrap();
        assert!(registry.get_by_name("thermal_1").is_none());
    }
}