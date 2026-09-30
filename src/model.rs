use serde::{Deserialize, Serialize};
use serde_json::error;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    fn new(name: String, description: String) -> Epic {
        let epic = Epic {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        };
        epic
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    fn new(name: String, description: String) -> Story {
        let story = Story {
            name,
            description,
            status: Status::Open,
        };
        story
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
