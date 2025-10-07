use crate::component::{SimulationComponent, ComponentResult, ComponentError};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ComponentRegistry {
    components: HashMap<Uuid, Box<dyn SimulationComponent>>,
    name_to_id: HashMap<String, Uuid>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }
    
    /// Add a component with a specific ID
    pub fn add_component(&mut self, id: Uuid, name: String, component: Box<dyn SimulationComponent>) -> ComponentResult<()> {
        if self.name_to_id.contains_key(&name) {
            return Err(ComponentError::InitializationFailed(
                format!("Component with name '{}' already exists", name)
            ));
        }
        
        self.components.insert(id, component);
        self.name_to_id.insert(name, id);
        Ok(())
    }
    
    /// Add a component with auto-generated ID
    pub fn add(&mut self, name: String, component: Box<dyn SimulationComponent>) -> ComponentResult<Uuid> {
        let id = Uuid::new_v4();
        self.add_component(id, name, component)?;
        Ok(id)
    }
    
    /// Remove a component by ID
    pub fn remove(&mut self, id: Uuid) -> ComponentResult<()> {
        self.components.remove(&id)
            .ok_or(ComponentError::InvalidInput(format!("Component {} not found", id)))?;
        
        // Remove from name map
        self.name_to_id.retain(|_, v| *v != id);
        Ok(())
    }
    
    /// Get component by ID
    pub fn get(&self, id: Uuid) -> Option<&dyn SimulationComponent> {
        self.components.get(&id).map(|b| b.as_ref())
    }
    
    /// Get mutable component by ID
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Box<dyn SimulationComponent>> {
        self.components.get_mut(&id)
    }

    /// Get mutable component by name
    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut Box<dyn SimulationComponent>> {
        self.name_to_id.get(name).copied()
            .and_then(|id| self.components.get_mut(&id))
    }
    /// Get component by name
    pub fn get_by_name(&self, name: &str) -> Option<&dyn SimulationComponent> {
        self.name_to_id.get(name)
            .and_then(|id| self.get(*id))
    }
    
    /// Step all components
    pub fn step_all(&mut self, dt: f64) -> ComponentResult<()> {
        for component in self.components.values_mut() {
            component.step(dt)?;
        }
        Ok(())
    }
    
    /// List all component IDs
    pub fn list_ids(&self) -> Vec<Uuid> {
        self.components.keys().copied().collect()
    }
    
    /// List all component names
    pub fn list_names(&self) -> Vec<String> {
        self.name_to_id.keys().cloned().collect()
    }
}