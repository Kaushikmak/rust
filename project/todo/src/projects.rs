use std::collections::HashMap;

use chrono::{DateTime, Local};
use uuid::Uuid;

use crate::task::Task;

#[derive(Debug)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub tasks: HashMap<Uuid, Task>,
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: Local::now(),
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    pub fn remove_task(&mut self, task_id: Uuid) -> Option<Task> {
        self.tasks.remove(&task_id)
    }

    pub fn find_task(&self, task_id: Uuid) -> Option<&Task> {
        self.tasks.get(&task_id)
    }

    pub fn find_task_mut(&mut self, task_id: Uuid) -> Option<&mut Task> {
        self.tasks.get_mut(&task_id)
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
    }

    pub fn print_summary(&self) {
        println!("Project: {}", self.name);
        println!("Description: {}", self.description);
        println!("Tasks: {}", self.tasks.len());
    }
}