use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn pick(count: usize) -> Vec<String> {
    let mut emoji: Vec<&str> = include_str!("./emoji_list.txt").split("\n").collect();
    emoji.shuffle(&mut thread_rng());
    return emoji.iter().take(count).map(|s| s.to_string()).collect();
}
