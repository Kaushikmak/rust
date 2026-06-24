use uuid::Uuid;

pub trait HasId {
    fn id(&self) -> Uuid;
}

pub trait Printable {
    fn print(&self);
}

pub trait Searchable {
    fn matches(&self, query: &str) -> bool;
}