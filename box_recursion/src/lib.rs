#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Role {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }

}

#[derive(Debug)]

pub struct WorkEnvironment {
    pub grade: Link,
}
pub type Link = Option<Box<Worker>>; 

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
     Self {grade: None}
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let worker = Worker {
            role: role.into(),
            name: name.to_string(),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|ch| {
            self.grade = ch.next;
            ch.name
        })
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
       self.grade.as_ref().map(|worker| {
        (worker.name.clone(), worker.role.clone())
       })
    }
}
