const INPUT: &str = include_str!("input.txt");

fn calculate_calories(inventory: &str) -> usize {
    inventory
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .sum()
}

fn main() {
    let mut inventories = INPUT
        .split("\n\n")
        .map(calculate_calories)
        .collect::<Vec<_>>();
    inventories.sort_by(|a, b| b.cmp(a));
    println!("{}", inventories[0]);
    println!("{}", inventories.iter().take(3).sum::<usize>());
}
