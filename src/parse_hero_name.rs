/// Expected input: "npc_dota_hero_{hero_name}"
pub fn parse_hero_name(input: &str) -> Option<String> {
    let split: Vec<&str> = input.splitn(4, "_").collect();
    match split.last() {
        Some(last) => Some(String::from(*last)),
        None => None,
    }
}
