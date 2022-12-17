use std::{collections::HashSet, ops::{Range}, cmp::{min, max}};
use range_ext::intersect::Intersect;
use sscanf::sscanf;
#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct Sensor {
    position: Point,
    closest_beacon: Point,
}

fn distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        let parts = sscanf!(
            s,
            "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}"
        )
        .unwrap();
        Self {
            position: Point {
                x: parts.0,
                y: parts.1,
            },
            closest_beacon: Point {
                x: parts.2,
                y: parts.3,
            },
        }
    }
}

impl Sensor {
    fn distance_to_beacon(&self) -> i32 {
        distance(&self.position, &self.closest_beacon)
    }

    fn points_within_distance_to_beacon_on_y(&self, relevant_y: i32) -> Vec<i32> {
        let distance = self.distance_to_beacon();

        let distance_y = relevant_y - self.position.y;

        (0..=(distance - distance_y.abs())).flat_map(|d| vec![self.position.x + d, self.position.x - d]).collect()
    }

    fn range_within_distance_to_beacon_on_y(&self, relevant_y: i32) -> Range<i32> {
        let distance = self.distance_to_beacon();

        let distance_y = relevant_y - self.position.y;

        let distance_x = distance - distance_y.abs();
        (max(0,self.position.x - distance_x))..(self.position.x + distance_x + 1)
    }
}

fn solve_part_1_for_y(input: &[Sensor], y: i32) -> usize {
    let points_within_distance_to_beacons = input
        .into_iter()
        .flat_map(|s| s.points_within_distance_to_beacon_on_y(y)).collect::<HashSet<_>>();
    let beacons_at_y = input.iter().filter(|s| s.closest_beacon.y == y).map(|s| s.closest_beacon.clone()).map(|b| b.x).collect::<HashSet<_>>();
    points_within_distance_to_beacons.iter().filter(|p| !beacons_at_y.contains(p)).count()
}

fn merge_ranges_as_long_as_shrinking(input: &Vec<Range<i32>>) -> Vec<Range<i32>> {
    let mut start = input.clone();
    loop {
        let merged = merge_ranges(&start);
        if merged == start { return merged; }
        else {
            start = merged.clone();
        }
    }
}

fn merge_ranges(input: &Vec<Range<i32>>) -> Vec<Range<i32>> {
    let mut ranges = input.clone();
    let mut merged_ranges: Vec<Range<i32>> = Vec::new();
    ranges.sort_by(|a, b| (a.end - a.start).cmp(&(b.end - b.start)));

    for range in ranges  {
        if range.is_empty() { continue; }
        if merged_ranges.is_empty() { merged_ranges = vec![range]; }
            else if !merged_ranges.iter().any(|r| r.intersect(&range).is_any()) {
                let touch_1 = merged_ranges.clone().iter().enumerate().find(|r| r.1.end == range.start).map(|r| {
                    let new_range = r.1.start..range.end;
                    merged_ranges.remove(r.0);
                    merged_ranges.push(new_range);
                });
                if !touch_1.is_some() {
                    let touch_2 = merged_ranges.clone().iter().enumerate().find(|r| r.1.start == range.end).map(|r| {
                        let new_range = range.start..r.1.end;
                        merged_ranges.remove(r.0);
                        merged_ranges.push(new_range);
                    });
                    if !touch_2.is_some() {
                        merged_ranges.push(range);
                    }
                }

            }
            else if merged_ranges.iter().any(|r| r.intersect(&range).is_over()) { continue; }
            else {
                if let Some(w) = merged_ranges.iter().enumerate().find(|r| r.1.intersect(&range).is_within()) {
                    merged_ranges.remove(w.0);
                    merged_ranges.push(range);
                } else if let Some(w) = merged_ranges.clone().iter().enumerate().find(|r| r.1.intersect(&range).is_any()) {
                    merged_ranges.remove(w.0);
                    let new_range = min(range.start, w.1.start)..(max(range.end, w.1.end));
                    merged_ranges.push(new_range);
                } else {
                    merged_ranges.clone().iter().enumerate().find(|r| r.1.end == range.start).map(|r| {
                        let new_range = r.1.start..range.end;
                        merged_ranges.remove(r.0);
                        merged_ranges.push(new_range);
                    });
                    merged_ranges.clone().iter().enumerate().find(|r| r.1.start == range.end).map(|r| {
                        let new_range = range.start..r.1.end;
                        merged_ranges.remove(r.0);
                        merged_ranges.push(new_range);
                    });
                }
            }
    }
    merged_ranges
}

fn solve_part_2_for_y(input: &[Sensor], min_y: i32, max_y: i32) -> usize {
    let complete_range = min_y..(max_y + 1);
    let result_list = (min_y..=max_y).collect::<HashSet<_>>().iter().map(|y| {

        let ranges = input
            .into_iter()
            .map(|s| s.range_within_distance_to_beacon_on_y(*y)).collect::<Vec<_>>();


        let merged_ranges = &merge_ranges_as_long_as_shrinking(&ranges);

        if !merged_ranges.iter().any(|r| complete_range.intersect(r).is_within()) {
            //dbg!(y);
            dbg!(&merged_ranges);
            let all_points_set = merged_ranges.iter().flat_map(|r| r.clone().into_iter().collect::<Vec<_>>()).collect::<HashSet<_>>();
            let mut sorted = all_points_set.iter().collect::<Vec<_>>();
            sorted.sort();
            if let Some(missing) = sorted.windows(2).find(|s| *s[0] + 1 != *s[1]) {
                dbg!(missing);
                return Some(((missing[0] + 1) as usize) * 4000000 + *y as usize);
            }
        };
        return None;
    }).filter(|s| s.is_some()).map(|s| s.unwrap()).collect::<Vec<_>>();
    *result_list.first().unwrap()
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Sensor> {
    input.lines().map(|l| Sensor::from(l)).collect::<Vec<_>>()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[Sensor]) -> usize {
    solve_part_1_for_y(input, 2000000)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Sensor]) -> usize {
    solve_part_2_for_y(input, 0, 4000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_generate_input_1() {
        let input = input_generator(EXAMPLE_INPUT);

        assert_eq!(input.len(), 14);
        assert_eq!(
            input.contains(&Sensor {
                position: Point { x: 2, y: 18 },
                closest_beacon: Point { x: -2, y: 15 }
            }),
            true
        );
    }


    #[test]
    fn it_should_compute_points_on_y() {
        let sensor = Sensor {
            position: Point { x: 8, y: 7 },
            closest_beacon: Point { x: 8, y: 8 },
        };

        let result = sensor.points_within_distance_to_beacon_on_y(7);

        assert_eq!(result.len(), 2);
        assert_eq!(result.contains(&7), true);
        assert_eq!(result.contains(&9), true);
    }

    #[test]
    fn it_should_compute_points_with_same_distance_1() {
        let sensor = Sensor {
            position: Point { x: 8, y: 7 },
            closest_beacon: Point { x: 2, y: 10 },
        };

        let result = sensor.points_within_distance_to_beacon_on_y(10);
        dbg!(&result);
        assert_eq!(sensor.distance_to_beacon(), 9);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn it_should_solve_part1_example_input() {
        let input = input_generator(EXAMPLE_INPUT);

        let result = solve_part_1_for_y(&input, 10);

        assert_eq!(result, 26)
    }

    #[test]
    fn it_should_solve_part2_example_input() {
        let input = input_generator(EXAMPLE_INPUT);

        let result = solve_part_2_for_y(&input, 0, 20);

        assert_eq!(result, 56000011)
    }

    const EXAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}
