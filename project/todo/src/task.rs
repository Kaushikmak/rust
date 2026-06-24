use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub created_at: DateTime<Local>,
    pub last_modified_at: DateTime<Local>,
}

impl Task {
    pub fn new(title: String, body: String, priority: TaskPriority,) -> Self {
        let now: DateTime<Local> = Local::now();
        Self {
            id: Uuid::new_v4(),
            title,
            body,
            status: TaskStatus::NotStarted,
            priority,
            created_at: now,
            last_modified_at: now,
        }
    }

    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
        self.modified();
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.modified();
    }

    pub fn reopen(&mut self) {
        self.status = TaskStatus::NotStarted;
        self.modified();
    }

    pub fn rename(&mut self, new_title: String) {
        self.title = new_title;
        self.modified();
    }

    pub fn update_body(&mut self, new_body: String) {
        self.body = new_body;
        self.modified();
    }

    pub fn set_priority(&mut self,priority: TaskPriority,) {
        self.priority = priority;
        self.modified();
    }

    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }

    pub fn is_in_progress(&self) -> bool {
        self.status == TaskStatus::InProgress
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn get_status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn get_priority(&self) -> &TaskPriority {
        &self.priority
    }

    fn modified(&mut self) {
        self.last_modified_at = Local::now();
    }
}