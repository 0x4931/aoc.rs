use std::collections::BTreeMap;
use std::ops::Range;

#[derive(Debug, Default, Clone)]
struct Mapping {
    dst_start: i64,
    src_start: i64,
    range_len: i64,
}

fn compute_transform_mappings(transform: &Vec<Mapping>) -> BTreeMap<i64, i64> {
    // Numbers x in the range of K [a, b) is mapped to x + V
    let mut mappings = BTreeMap::new();
    mappings.insert(0, 0);

    for mapping in transform.iter() {
        mappings.insert(mapping.src_start, mapping.dst_start - mapping.src_start);
        // Reset the mapping if another mapping doesn't already exist
        if !mappings.contains_key(&(mapping.src_start + mapping.range_len)) {
            mappings.insert(mapping.src_start + mapping.range_len, 0);
        }
    }

    mappings
}

fn part_one(numbers: &Vec<i64>, transforms: &Vec<Vec<Mapping>>) {
    let mut numbers = numbers.clone();

    for transform in transforms.iter() {
        let mappings = compute_transform_mappings(transform);
        for number in numbers.iter_mut() {
            let mapping = *mappings.range(..=*number).rev().next().unwrap().1;
            let mapped = *number + mapping;
            *number = mapped;
        }
    }

    let lowest = numbers.into_iter().min().unwrap();
    println!("{}", lowest);
}

fn part_two(numbers: &Vec<i64>, transforms: &Vec<Vec<Mapping>>) {
    let mut ranges: Vec<Range<i64>> = numbers
        .chunks_exact(2)
        .map(|pair| pair.try_into().unwrap())
        .map(|[start, length]: [i64; 2]| start..start + length)
        .collect();

    for transform in transforms.iter() {
        let mappings = compute_transform_mappings(transform);

        ranges = ranges
            .into_iter()
            .flat_map(|range| {
                let mut subranges: Vec<_> = mappings
                    .range(range.start + 1..range.end)
                    .map(|(start, _)| *start)
                    .collect();
                subranges.insert(0, range.start);
                subranges.push(range.end);

                let mut mapped = Vec::new();

                for subrange in subranges.windows(2) {
                    let subrange: [_; 2] = subrange.try_into().unwrap();
                    let [start, end] = subrange;

                    let mapping = *mappings.range(..=start).rev().next().unwrap().1;
                    let start_mapped = start + mapping;
                    let end_mapped = end + mapping;

                    mapped.push(start_mapped..end_mapped);
                }

                mapped
            })
            .collect();
    }

    let lowest = ranges.into_iter().map(|range| range.start).min().unwrap();
    println!("{}", lowest);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 5)?;

    let mut numbers = Vec::new();
    let mut transforms = Vec::new();

    let (numbers_info, transforms_info) = input.split_once("\n\n").unwrap();
    let (_, numbers_info) = numbers_info.split_once(": ").unwrap();

    for number in numbers_info.split_whitespace() {
        let number: i64 = number.parse().unwrap();
        numbers.push(number);
    }

    for transform_info in transforms_info.split("\n\n") {
        let mut transform = Vec::new();
        // Assume none of the mappings overlap with each other
        for mapping_info in transform_info.lines().skip(1) {
            let mut mapping_info = mapping_info.split_whitespace();
            let dst_start: i64 = mapping_info.next().unwrap().parse().unwrap();
            let src_start: i64 = mapping_info.next().unwrap().parse().unwrap();
            let range_len: i64 = mapping_info.next().unwrap().parse().unwrap();
            transform.push(Mapping {
                dst_start,
                src_start,
                range_len,
            });
        }
        transforms.push(transform);
    }

    part_one(&numbers, &transforms);
    part_two(&numbers, &transforms);

    Ok(())
}
