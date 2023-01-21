use crate::parse_hero_name::parse_hero_name;

pub fn parse_subject(input: &str) -> Option<(String, String)> {
    if input.contains("illusion") {
        return None;
    }

    if !input.contains("is killed by") {
        return None;
    }

    let split: Vec<&str> = input.split(" ").collect();
    let subject = match split.get(1) {
        None => return None,
        Some(x) => *x,
    };

    if !subject.contains("npc_dota_hero") {
        return None;
    }

    let killer = match split.last() {
        None => return None,
        Some(x) => *x,
    };

    if !killer.contains("npc_dota_hero") {
        return None;
    }

    let subject_hero_name = parse_hero_name(subject).unwrap();
    let killer_hero_name = parse_hero_name(killer).unwrap();

    Some((subject_hero_name, killer_hero_name))
}
