const RAINDROP_SOUNDS: &[(u32, &str)] = &[(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let result: String = RAINDROP_SOUNDS
        .iter()
        .filter_map(|(multiple, sound)| (n % multiple == 0).then_some(*sound))
        .collect();

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
