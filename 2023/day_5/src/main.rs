#![feature(iter_array_chunks)]
#![feature(exact_size_is_empty)]
use std::{ops::Range, str::FromStr};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Map {
    source: Range<usize>,
    destination: Range<usize>,
}
impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_ascii_whitespace();
        let destination_start = s.next().unwrap().parse()?;
        let source_start = s.next().unwrap().parse()?;
        let length = s.next().unwrap().parse::<usize>()?;
        Ok(Self {
            source: source_start..source_start + length,
            destination: destination_start..destination_start + length,
        })
    }
}

fn get_location(seed: usize, maps_list: &[Vec<Map>]) -> usize {
    let mut location = seed;
    for maps in maps_list {
        for map in maps {
            if map.source.start <= location && location < map.source.end {
                location = map.destination.start + (location - map.source.start);
                break;
            }
        }
    }
    location
}

fn get_location_range(seed_ranges: Range<usize>, maps_list: &[Vec<Map>]) -> Vec<Range<usize>> {
    let mut locations = vec![seed_ranges];

    for maps in maps_list {
        let mut new_locations = vec![];

        for location in &mut locations {
            for map in maps {
                if location.is_empty() {
                    break;
                }
                if map.source.end <= location.start {
                    continue;
                }
                if map.source.start <= location.start {
                    new_locations.push(
                        location.start - map.source.start + map.destination.start
                            ..map
                                .destination
                                .end
                                .min(location.end - map.source.start + map.destination.start),
                    );
                    location.start = map.source.end;
                    continue;
                }
                if location.end <= map.source.start {
                    continue;
                }
                new_locations.push(
                    map.destination.start
                        ..map
                            .destination
                            .end
                            .min(location.end - map.source.start + map.destination.start),
                );
                location.end = map.source.start;
            }
            if !location.is_empty() {
                new_locations.push(location.start..location.end);
            }
        }
        locations = new_locations;
    }
    locations
}

fn get_lowest_location(almanac: &str) -> usize {
    let (seeds, maps_list) = almanac.split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let maps_list = maps_list
        .split("\n\n")
        .map(|maps| {
            let mut maps = maps.lines();
            maps.next();
            maps.map(|map| map.parse::<Map>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut min_location = usize::MAX;
    for seed in seeds
        .split_ascii_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
    {
        let location = get_location(seed, &maps_list);
        if location < min_location {
            min_location = location;
        }
    }
    min_location
}

fn get_lowest_location_with_range(almanac: &str) -> usize {
    let (seeds, maps_list) = almanac.split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let maps_list = maps_list
        .split("\n\n")
        .map(|maps| {
            let mut maps = maps.lines();
            maps.next();
            maps.map(|map| map.parse::<Map>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut min_location = usize::MAX;
    for [seed_range_start, seed_range_length] in seeds
        .split_ascii_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .array_chunks()
    {
        let locations = get_location_range(
            seed_range_start..seed_range_start + seed_range_length,
            &maps_list,
        );
        for location in &locations {
            if location.start < min_location {
                min_location = location.start;
            }
        }
    }
    min_location
}

fn main() {
    println!("{}", get_lowest_location(INPUT.trim()));
    println!("{}", get_lowest_location_with_range(INPUT.trim()));
}
