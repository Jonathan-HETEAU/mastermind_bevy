use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Debug)]

pub struct Snapshots {
    snapshots: HashMap<String, Vec<Entity>>,
}

impl FromWorld for Snapshots {
    fn from_world(_world: &mut World) -> Self {
        Snapshots::new()
    }
}

impl Snapshots {
    pub fn new() -> Self {
        Snapshots {
            snapshots: HashMap::new(),
        }
    }

    pub fn snap(&mut self, name: &String, entities: Vec<Entity>) {
        self.snapshots.insert(name.clone(), entities);
    }

    pub fn get_mut_snap(&mut self, name: &String) -> Option<&mut Vec<Entity>> {
        self.snapshots.get_mut(name)
    }
}
