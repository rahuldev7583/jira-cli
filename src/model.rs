use std::collections::HashMap;

pub enum Status {
    Opened,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<u32>,
}

impl Epic {
    fn new(name: String, description: String) -> Epic {
        let epic = Epic {
            name,
            description,
            status: Status::Opened,
            stories: Vec::new(),
        };
        epic
    }
}
pub struct Story {
    name: String,
    description: String,
    status: Status,
}

impl Story {
    fn new(name: String, description: String) -> Story {
        let story = Story {
            name,
            description,
            status: Status::Opened,
        };
        story
    }
}

pub struct DBState {
    last_item_id: u32,
    epics: HashMap<u32, Epic>,
    stories: HashMap<u32, Story>,
}
