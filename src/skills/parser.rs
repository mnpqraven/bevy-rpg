//! this will hold alll the information about skills and also relationships
//! between skills and units
use crate::ecs::component::{SkillGroup, Target, Learned, UnitArchetype};
use serde::{Deserialize, Serialize};
use std::fs;

// NOTE: current tasks needing to be answered
// which entity has access to which skills?
// which entity can learn which skills?

/// Resource
/// Skill data table, struct for importing/exporting to json table
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillEntry {
    pub label_name: String,
    pub skill_group: Vec<SkillGroup>,
    pub target: Target,
    // used as filter for units
    // (units also have UnitArchetype)
    pub learnable_archetypes: Vec<UnitArchetype>,
    pub learned: Learned, // tristate, basic: starter skills, Learned, unlearned
    pub mana: Option<i32>,
    pub damage: Option<i32>,
    pub block: Option<i32>,
    pub heal: Option<i32>,
    pub channel: Option<u32>,
}

/// Scan skillbook.yaml in assets/db for list of default skills in the database
pub fn scan_skillbook_yaml() -> Vec<SkillEntry> {
    let file = fs::read_to_string("./assets/db/skillbook.yaml")
        .expect("file not found or read perm error ");
    let res: Vec<SkillEntry> = serde_yaml::from_str(&file).expect("unable to parse");
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

