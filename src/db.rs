use anyhow::{Error, Ok, Result, anyhow};
use serde::Serialize;
use std::{
    collections::HashMap,
    f32::consts::E,
    fs::{self, File},
    io::{Read, Write},
};
use tempfile::*;

use crate::model::*;

pub trait Database {
    fn read_db(&self) -> Result<DBState, Error>;
    fn write_db(&self, dbstate: &DBState) -> Result<(), Error>;
}

pub struct JiraDatabase {
    pub database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        // println!("file_path:{}", file_path);
        // let mut file = File::open(file_path)?;
        // let data = DBState {
        //     last_item_id: 0,
        //     epics: HashMap::new(),
        //     stories: HashMap::new(),
        // };
        //
        // let data_json = serde_json::to_string(&data)?;
        //
        // file.write_all(&data_json.as_bytes());
        //
        // Ok(data)

        Self {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }
    pub fn read_db(&self) -> Result<DBState, Error> {
        //  println!("hi");

        let result = self.database.read_db()?;

        Ok(result)
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32, Error> {
        //  println!("create epic called");
        let mut exiting_file = self.database.read_db().expect("error");

        let last_item = exiting_file.last_item_id;
        //  println!("last_item: {}", last_item);
        let new_item = last_item + 1;

        exiting_file.last_item_id = new_item;
        exiting_file.epics.insert(new_item, epic);

        //   println!("exiting_file: {:?}", exiting_file);

        self.database.write_db(&exiting_file);

        Ok(new_item)
    }
    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        //   println!("create story called");
        let mut exiting_file = self.database.read_db()?;

        let last_item = exiting_file.last_item_id;
        // println!("last_item: {}", last_item);
        let new_item = last_item + 1;

        exiting_file.last_item_id = new_item;

        let find_epic = exiting_file.epics.get_mut(&epic_id);

        let mut epic = find_epic.ok_or(anyhow::Error::msg("epic not found"))?;

        //  println!("epic: {:?}", epic);

        epic.stories.push(new_item);
        // println!("exiting_file: {:?}", exiting_file);

        exiting_file.stories.insert(new_item, story);
        // println!("exiting_file: {:?}", exiting_file);

        self.database.write_db(&exiting_file);

        Ok(new_item)
    }
    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        //  println!("delete epic called");
        let mut exiting_file = self.database.read_db().expect("error");

        //  println!("exiting_file: {:?}", exiting_file);
        let find_epic = exiting_file.epics.get_mut(&epic_id);

        let mut epic = find_epic.ok_or(anyhow::Error::msg("epic not found"))?;

        //  println!("epic: {:?}", epic);

        let epic_stories = &epic.stories;

        //iterate ont his epic stories, and remove this from strories hashmap

        for i in epic_stories {
            println!("delete this story from epic: {}", i);

            exiting_file.stories.remove(i);
        }

        exiting_file.epics.remove(&epic_id);
        //  println!("exiting_file: {:?}", exiting_file);
        self.database.write_db(&exiting_file);

        Ok(())
    }
    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<(), Error> {
        // println!("delete story called");
        let mut exiting_file = self.database.read_db().expect("error");

        //  println!("exiting_file: {:?}", exiting_file);
        let find_epic = exiting_file.epics.get_mut(&epic_id);

        let mut epic = find_epic.ok_or(anyhow::Error::msg("epic not found"))?;

        // println!("epic: {:?}", epic);

        let epic_stories = &epic.stories;

        let find_story = epic_stories
            .iter()
            .find(|&e| e == &story_id)
            .ok_or(anyhow::Error::msg("story  not found"))?;

        //   println!("find story: {:?}", find_story);

        //  println!("story index: {}", 1);

        let arr = vec![4, 2, 3, 6, 8, 1];

        let index = epic_stories.iter().position(|&a| a == story_id).unwrap();

        println!("arr: {:?}, and index:{}", arr, index);

        epic.stories.remove(index);

        exiting_file.stories.remove(&story_id);

        println!("exiting_file: {:?}", exiting_file);
        self.database.write_db(&exiting_file);

        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<u32> {
        //    println!("update epic called");
        let mut exiting_file = self.database.read_db().expect("error");

        //   println!("exiting_file: {:?}", exiting_file);
        let find_epic = exiting_file.epics.get_mut(&epic_id);

        let mut epic = find_epic.ok_or(anyhow::Error::msg("epic not found"))?;

        //    println!("epic: {:?}", epic);

        epic.status = status;
        //  println!("exiting_file: {:?}", exiting_file);
        self.database.write_db(&exiting_file);
        Ok(epic_id)
    }
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<u32> {
        // println!("update story called");
        let mut exiting_file = self.database.read_db().expect("error");

        //  println!("exiting_file: {:?}", exiting_file);
        let find_story = exiting_file.stories.get_mut(&story_id);

        let mut story = find_story.ok_or(anyhow::Error::msg("story not found"))?;

        // println!("story: {:?}", story);

        story.status = status;
        // println!("exiting_file: {:?}", exiting_file);
        self.database.write_db(&exiting_file);
        Ok(story_id)
    }
}

#[derive(Clone, Debug)]
pub struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState, Error> {
        // println!("file_path: {}", self.file_path);

        let mut file = File::open(&self.file_path)?;

        //  println!("file: {:?}", file);
        let mut buffer = String::new();

        //   println!("buffer: {}", buffer);
        file.read_to_string(&mut buffer)?;

        //  println!("got read");

        //    println!("content: {}", buffer);

        let json_file: DBState = serde_json::from_str(&buffer)?;

        //  println!("json_file, {:?}", json_file);
        Ok(json_file)
    }

    fn write_db(&self, dbstate: &DBState) -> Result<(), Error> {
        let content = serde_json::to_string(dbstate)?;

        fs::write(&self.file_path, content);
        Ok(())
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     mod database {
//         use std::collections::HashMap;
//         use std::io::Write;
//
//         use super::*;
//
//         #[test]
//         fn read_db_should_fail_with_invalid_path() {
//             let db = JSONFileDatabase {
//                 file_path: "INVALID_PATH".to_owned(),
//             };
//             assert_eq!(db.read_db().is_err(), true);
//         }
//
//         #[test]
//         fn read_db_should_fail_with_invalid_json() {
//             let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
//
//             let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
//             write!(tmpfile, "{}", file_contents).unwrap();
//
//             let db = JSONFileDatabase {
//                 file_path: tmpfile
//                     .path()
//                     .to_str()
//                     .expect("failed to convert tmpfile path to str")
//                     .to_string(),
//             };
//
//             let result = db.read_db();
//
//             assert_eq!(result.is_err(), true);
//         }
//
//         #[test]
//         fn read_db_should_parse_json_file() {
//             let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
//
//             let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
//             write!(tmpfile, "{}", file_contents).unwrap();
//
//             let db = JSONFileDatabase {
//                 file_path: tmpfile
//                     .path()
//                     .to_str()
//                     .expect("failed to convert tmpfile path to str")
//                     .to_string(),
//             };
//
//             let result = db.read_db();
//
//             assert_eq!(result.is_ok(), true);
//         }
//
//         #[test]
//         fn write_db_should_work() {
//             let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
//
//             let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
//             write!(tmpfile, "{}", file_contents).unwrap();
//
//             let db = JSONFileDatabase {
//                 file_path: tmpfile
//                     .path()
//                     .to_str()
//                     .expect("failed to convert tmpfile path to str")
//                     .to_string(),
//             };
//
//             let story = Story {
//                 name: "epic 1".to_owned(),
//                 description: "epic 1".to_owned(),
//                 status: Status::Open,
//             };
//             let epic = Epic {
//                 name: "epic 1".to_owned(),
//                 description: "epic 1".to_owned(),
//                 status: Status::Open,
//                 stories: vec![2],
//             };
//
//             let mut stories = HashMap::new();
//             stories.insert(2, story);
//
//             let mut epics = HashMap::new();
//             epics.insert(1, epic);
//
//             let state = DBState {
//                 last_item_id: 2,
//                 epics,
//                 stories,
//             };
//
//             let write_result = db.clone().write_db(&state);
//             let read_result = db.read_db().unwrap();
//
//             assert_eq!(write_result.is_ok(), true);
//             // TODO: fix this error by deriving the appropriate traits for DBState
//             assert_eq!(read_result, state);
//         }
//     }
// }

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }

    impl MockDB {
        pub fn new() -> Self {
            Self {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics: HashMap::new(),
                    stories: HashMap::new(),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            // TODO: fix this error by deriving the appropriate traits for Story
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            // TODO: fix this error by deriving the appropriate traits for DBState
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::MockDB;
    use super::*;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        // TODO: fix this error by deriving the appropriate traits for Epic
        let result = db.create_epic(epic.clone());

        assert_eq!(result.is_ok(), true);

        println!("result: {:?}", result);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        println!("db_state: {:?}", db_state);

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);

        let epic_test = db_state.epics.get(&id);

        println!("epic_test: {:?}, epic: {:?}", epic_test, &epic);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        // TODO: fix this error by deriving the appropriate traits for Story
        let result = db.create_story(story.clone(), epic_id);
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        println!("db_state: {:?}", db_state);
        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(
            db_state.epics.get(&epic_id).unwrap().stories.contains(&id),
            true
        );
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let non_existent_epic_id = 999;

        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let non_existent_story_id = 999;

        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        println!("db_state: {:?}", db_state);

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(
            db_state
                .epics
                .get(&epic_id)
                .unwrap()
                .stories
                .contains(&story_id),
            false
        );
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_story_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(
            db_state.stories.get(&story_id).unwrap().status,
            Status::Closed
        );
    }

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_owned(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let story = Story {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
            };
            let epic = Epic {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
                stories: vec![2],
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}
