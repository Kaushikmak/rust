use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
enum Subject {
    English,
    Mathematics,
    Physics,
    Chemistry,
    French,
    Geography,
    History,
    Philosophy,
}

#[derive(Debug, Clone)]
enum Slot {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Clone)]
enum Class {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
    XI,
    XII,
}

#[derive(Debug, Clone)]
enum RoomNo {
    G01,
    G02,
    G03,
    G04,
    F01,
    F02,
    F03,
    F04,
    Library,
    MeetingRoom,
    SportsRoom,
}

#[derive(Debug, Clone)]
struct Person {
    id: u32,
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u32,
    address: String,
}

#[derive(Debug, Clone)]
struct Student {
    person: Person,
    class: Class,
}

#[derive(Debug, Clone)]
struct Teacher {
    person: Person,
    subjects: Vec<Subject>,
    salary: u32,
    join_date: DateTime<Local>,
    retirement_date: DateTime<Local>,
}

#[derive(Debug)]
struct TimetableEntry {
    class: Class,
    subject: Subject,
    teacher_id: u32,
    room: RoomNo,
    slot: Slot,
}

trait HasId {
    fn id(&self) -> u32;
}

trait FullName {
    fn full_name(&self) -> String;
}

trait Printable {
    fn print(&self);
}

impl FullName for Person {
    fn full_name(&self) -> String {
        format!(
            "{} {} {}",
            self.first_name,
            self.middle_name,
            self.last_name
        )
    }
}

impl HasId for Student {
    fn id(&self) -> u32 {
        self.person.id
    }
}

impl HasId for Teacher {
    fn id(&self) -> u32 {
        self.person.id
    }
}

impl Printable for Student {
    fn print(&self) {
        println!(
            "Student => ID: {}, Name: {}, Class: {:?}",
            self.person.id,
            self.person.full_name(),
            self.class
        );
    }
}

impl Printable for Teacher {
    fn print(&self) {
        println!(
            "Teacher => ID: {}, Name: {}, Salary: {}",
            self.person.id,
            self.person.full_name(),
            self.salary
        );
    }
}

struct Repository<T>
where
    T: HasId,
{
    records: Vec<T>,
}

impl<T> Repository<T>
where
    T: HasId,
{
    fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    fn add(&mut self, item: T) {
        self.records.push(item);
    }

    fn remove(&mut self, id: u32) {
        self.records.retain(|x| x.id() != id);
    }

    fn find(&self, id: u32) -> Option<&T> {
        self.records.iter().find(|x| x.id() == id)
    }

    fn count(&self) -> usize {
        self.records.len()
    }
}

impl<T> Repository<T>
where
    T: HasId + Printable,
{
    fn print_all(&self) {
        for item in &self.records {
            item.print();
        }
    }
}

fn main() {
    let mut student_repo = Repository::<Student>::new();
    let mut teacher_repo = Repository::<Teacher>::new();

    let student1 = Student {
        person: Person {
            id: 1,
            first_name: "John".to_string(),
            middle_name: "A".to_string(),
            last_name: "Smith".to_string(),
            age: 15,
            address: "London".to_string(),
        },
        class: Class::IX,
    };

    let student2 = Student {
        person: Person {
            id: 2,
            first_name: "Emma".to_string(),
            middle_name: "B".to_string(),
            last_name: "Johnson".to_string(),
            age: 16,
            address: "Paris".to_string(),
        },
        class: Class::X,
    };

    let teacher1 = Teacher {
        person: Person {
            id: 101,
            first_name: "Robert".to_string(),
            middle_name: "C".to_string(),
            last_name: "Brown".to_string(),
            age: 40,
            address: "Berlin".to_string(),
        },
        subjects: vec![Subject::Physics, Subject::Mathematics],
        salary: 80_000,
        join_date: Local::now(),
        retirement_date: Local::now(),
    };

    student_repo.add(student1);
    student_repo.add(student2);

    teacher_repo.add(teacher1);

    println!("Students:");
    student_repo.print_all();

    println!();

    println!("Teachers:");
    teacher_repo.print_all();

    println!();

    match student_repo.find(2) {
        Some(student) => {
            println!(
                "Found student: {}",
                student.person.full_name()
            );
        }
        None => {
            println!("Student not found");
        }
    }

    let timetable = TimetableEntry {
        class: Class::IX,
        subject: Subject::Physics,
        teacher_id: 101,
        room: RoomNo::G01,
        slot: Slot::A,
    };

    println!();
    println!("Timetable Entry:");
    println!("{:#?}", timetable);

    println!();
    println!(
        "Total Students: {}",
        student_repo.count()
    );

    println!(
        "Total Teachers: {}",
        teacher_repo.count()
    );

    student_repo.remove(1);

    println!();
    println!(
        "Students after removal: {}",
        student_repo.count()
    );
}