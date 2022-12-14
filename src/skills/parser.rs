use serde::{Deserialize, Serialize};
use std::fs;

use crate::ecs::component::{SkillGroup, Target};

/// Resource
/// Skill data table, struct for importing/exporting to json table
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillDataTable {
    pub label_name: String,
    pub skill_group: SkillGroup,
    pub target: Target,
    pub mana: Option<i32>,
    pub damage: Option<i32>,
    pub block: Option<i32>,
    pub heal: Option<i32>,
    pub channel: Option<u32>,
    pub learned: Option<bool>,
}

/// Scan skillbook.yaml in assets/db for list of default skills in the database
pub fn scan_skillbook_yaml() -> Vec<SkillDataTable> {
    let file = fs::read_to_string("./assets/db/skillbook.yaml")
        .expect("file not found or read perm error ");
    let res: Vec<SkillDataTable> = serde_yaml::from_str(&file).expect("unable to parse");
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn yamlscan_test() {
        println!("{:?}", scan_skillbook_yaml());
    }
}