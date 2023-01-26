use std::{collections::HashMap, fmt::Display};

use crate::{parse_game_time::parse_game_time, parse_hero_name::parse_hero_name};

const MISSED: &str = "Missed";

pub fn get_hook_percentage(file: &str) -> f32 {
    let pudge_team = get_team(&file);
    let split: Vec<&str> = file.split("\n").collect();
    let mut result: HashMap<String, u16> = HashMap::new();
    for (i, line) in split.iter().enumerate() {
        let has_cast = line.contains("npc_dota_hero_pudge casts ability pudge_meat_hook");
        if has_cast {
            let timestamp_of_hook_start = parse_game_time(line).unwrap();
            for interest_line in &split[i..] {
                let timestamp = match parse_game_time(&interest_line) {
                    Ok(res) => res,
                    Err(_) => continue,
                };
                // If timestamp is within 3 seconds, see if it hit
                // If timestamp is after 3 seconds, break the loop and report a miss
                if timestamp.as_millisecs() < timestamp_of_hook_start.as_millisecs() + 3_000 {
                    match did_hit_entity(&interest_line, &pudge_team) {
                        Some(entity) => {
                            match result.get(&entity.to_string()) {
                                Some(count) => {
                                    result.insert(entity.to_string(), count + 1);
                                }
                                None => {
                                    result.insert(entity.to_string(), 1);
                                }
                            }
                            break;
                        }
                        None => continue,
                    }
                } else {
                    match result.get(MISSED) {
                        Some(count) => {
                            result.insert(MISSED.to_string(), count + 1);
                        }
                        None => {
                            result.insert(MISSED.to_string(), 1);
                        }
                    };
                    break;
                }
            }
        }
    }

    println!("Result: {:?}", &result);
    calculate_percentage(&result)
}

fn calculate_percentage(result: &HashMap<String, u16>) -> f32 {
    let mut missed = 0;
    let mut hit = 0;

    for (key, value) in result {
        if key == MISSED {
            missed += value;
        } else if key == &DotaEntity::CreepAlly.to_string() {
            missed += value;
        } else if key == &DotaEntity::Neutral.to_string() {
            // Do nothing
        } else if key == &DotaEntity::CreepEnemy.to_string() {
            // Do nothing
        } else {
            hit += value;
        }
    }

    let total = missed + hit;
    hit as f32 / total as f32
}

fn did_hit_entity(line: &str, team: &Team) -> Option<DotaEntity> {
    if line.contains("with pudge_meat_hook") {
        let splits: Vec<&str> = line.split(" ").collect();
        let target = splits[3];
        if target.contains("npc_dota_hero") {
            let hero_name = parse_hero_name(&target).unwrap();
            Some(DotaEntity::Hero(hero_name))
        } else if target.contains("npc_dota_neutral") {
            Some(DotaEntity::Neutral)
        } else if target.contains("npc_dota_goodguys") {
            match team {
                Team::Radiant => Some(DotaEntity::CreepAlly),
                Team::Dire => Some(DotaEntity::CreepEnemy),
            }
        } else if target.contains("npc_dota_badguys") {
            match team {
                Team::Radiant => Some(DotaEntity::CreepEnemy),
                Team::Dire => Some(DotaEntity::CreepAlly),
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn get_team(file: &str) -> Team {
    // TODO: Replace with file.find()
    for line in file.split("\n").into_iter() {
        if line.contains("npc_dota_hero_pudge receives modifier_tower_aura_bonus buff/debuff from npc_dota_goodguys_tower4") {
            return Team::Radiant;
        } else if line.contains("npc_dota_hero_pudge receives modifier_tower_aura_bonus buff/debuff from npc_dota_badguys_tower4") {
            return Team::Dire;
        }
    }
    Team::Radiant
}

enum Team {
    Radiant,
    Dire,
}

enum DotaEntity {
    Hero(String),
    CreepAlly,
    CreepEnemy,
    Neutral,
}

impl Display for DotaEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotaEntity::Hero(name) => write!(f, "{}", name),
            DotaEntity::CreepAlly => write!(f, "Ally Creep"),
            DotaEntity::CreepEnemy => write!(f, "Enemy Creep"),
            DotaEntity::Neutral => write!(f, "Neutral Creep"),
        }
    }
}
