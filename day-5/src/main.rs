use std::env;
use std::fs;
use std::ops::Range;

fn main() {
    let args: Vec<String> = env::args().collect();

    let puzzle_num : &i64 = &args[1].parse().unwrap();
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    if *puzzle_num == 1 as i64 {
        puzzle_1(contents);
    } else if *puzzle_num == 2 as i64 {
        puzzle_2(contents);
    } else {
        println!("bad puzzle num");
    }
}

fn parse_seeds(line: &str) -> Vec<u64> {
    let (_seeds, nums) = line.split_once(":").unwrap();

    nums.split_whitespace().map(|num| num.parse::<u64>().unwrap()).collect()
}

fn parse_seeds_2(line: &str) -> Vec<DebugRange> {
    let (_seeds, nums) = line.split_once(":").unwrap();

    let mut result : Vec<DebugRange> = Vec::new();
    
    let mut num_iter = nums.trim().split_whitespace();

    while let Some(start) = num_iter.next() {
        let start_num = start.parse::<u64>().unwrap();
        let to_add = num_iter.next().unwrap().parse::<u64>().unwrap();

        let lineage = vec![LineageType::StartRange(Range { start: start_num, end: start_num + to_add})];

        result.push(DebugRange {
            lineage,
            curr_range: Range {
                start: start_num,
                end: start_num + to_add
            }
        });
    }

    result
}

enum LineType {
    MapLine(NextTypeMap),
    TitleLine,
    EmptyLine,
}

#[derive(Debug)]
struct NextTypeMap {
    dest: u64,
    source_range: Range<u64>
}

fn parse_line(line: &str)-> LineType {
    if line.is_empty() {
        return LineType::EmptyLine;
    } 

    if line.contains("map") {
        return LineType::TitleLine;
    }

    let mut nums = line.split_whitespace();

    let dest = nums.next().unwrap().parse::<u64>().unwrap();
    let source = nums.next().unwrap().parse::<u64>().unwrap();
    let num = nums.next().unwrap().parse::<u64>().unwrap();

    LineType::MapLine(
        NextTypeMap {
            dest,
            source_range: Range {
                start: source,
                end: source + num - 1
            }
        }
    )
}


fn puzzle_1(contents: String) {
    let mut lines = contents.lines();
   
    // Set up this way cause we always start with a swap on the first map line
    let mut next = parse_seeds(lines.next().unwrap());
    let mut current = Vec::new();

    for line in lines {
        match parse_line(line) {
            LineType::MapLine(next_type_map) => {
                println!("Map Line: {}", line);
                println!("Before: Current: {:?}, Next: {:?}", current, next);
                let source_range = next_type_map.source_range;
                current.retain(|num| {
                    if source_range.contains(num) {
                        let num_to_add = num - source_range.start;
                        next.push(next_type_map.dest + num_to_add);
                        return false;
                    }
                    true 
                });
                println!("After: Current: {:?}, Next: {:?}", current, next);
                println!("");
            },
            LineType::TitleLine => {
                println!("Swapping over");
                println!("Current is: {:?}", current);
                println!("Next is: {:?}", next);
                println!("");
                for num in current {
                    next.push(num);
                }
                current = next;
                next = Vec::new();
            },
            LineType::EmptyLine => {
                // do nothing
            }
        }
    }

    let min = next.iter().min().expect("has to have some value");

    println!("Lowest Location Num is: {}", min);
}

#[derive(Debug)]
struct OverlapRanges {
    ranges_to_keep: Vec<Range<u64>>,
    source_range_to_translate: Range<u64>
}

fn check_for_overlap(seed_range: &Range<u64>, next_type_map: &NextTypeMap) -> Option<OverlapRanges> {

    let next_range = &next_type_map.source_range;
    

    // No overlap 
    if seed_range.end < next_range.start || next_range.end < seed_range.start {
            // Skip this one
            return None;
    }

    // seed_range completely overlaps next_type_map
    if seed_range.start <= next_range.start && next_range.end <= seed_range.end {
        let mut ranges_to_keep = Vec::new();

        if seed_range.start != next_range.start {
            ranges_to_keep.push(Range {
                start: seed_range.start,
                end: next_range.start - 1
            });
        }

        if seed_range.end != next_range.end {
            ranges_to_keep.push(Range {
                start: next_range.end+1,
                end: seed_range.end
            });
        }

        return Some(OverlapRanges {
            ranges_to_keep,
            source_range_to_translate: next_range.to_owned()
        });
    }
    
    // seed_range completely inside next_type_map
    if next_range.start <= seed_range.start && seed_range.end <= next_range.end {
        return Some(OverlapRanges {
            ranges_to_keep: Vec::new(),
            source_range_to_translate: seed_range.to_owned()
        });
    }
    
    // seed_range overhangs from the front of next_type_map 
    if seed_range.start < next_range.start && seed_range.end <= next_range.end {
        let mut ranges_to_keep = Vec::new();
        let range_to_keep = Range {
            start: seed_range.start,
            end: next_range.start-1
        };
        ranges_to_keep.push(range_to_keep);

        return Some(OverlapRanges {
            ranges_to_keep,
            source_range_to_translate: Range {
                start: next_range.start,
                end: seed_range.end
            }
        });
    }
    
    // seed_range overhangs from the back of next_type_map
    if next_range.start <= seed_range.start && next_range.end < seed_range.end {
        let mut ranges_to_keep = Vec::new();
        let range_to_keep = Range {
            start: next_range.end + 1,
            end: seed_range.end
        };
        ranges_to_keep.push(range_to_keep);

        return Some(OverlapRanges {
            ranges_to_keep,
            source_range_to_translate: Range {
                start: seed_range.start,
                end: next_range.end
            }
        });
    }

    // Should never reach this
    println!("This shouldn't have happend");
    None
}

fn update_ranges(mut current: Vec<DebugRange>, curr_next_type_maps: Vec<NextTypeMap>) -> Vec<DebugRange> {

    let mut next : Vec<DebugRange> = Vec::new();
    
    // Check all ranges for overlap
    let mut i = 0;
    while i < current.len() {
        let curr_debug_range = current[i].clone();
        let curr_seed_range = curr_debug_range.curr_range;
        let mut overlapped = false;

        for next_type_map in &curr_next_type_maps {
            //println!("Current checking: {:?}", next_type_map);
            match check_for_overlap(&curr_seed_range, &next_type_map) {
                Some(overlap_ranges) => {
                    // One overlap occurred at least
                    overlapped = true;

                    // Mark any ranges that will need to be reprocessed
                    for range in overlap_ranges.ranges_to_keep {
                        // Tack it on to the back so that we'll check thsi again later
                        let mut new_lineage = curr_debug_range.lineage.clone();
                        new_lineage.push(LineageType::TrimmedPortionOfMap(LineageInfo {
                            range_used_for_decision: next_type_map.source_range.clone(),
                            range_snapshot: range.clone()
                        }));
                        current.push(DebugRange {
                            lineage: new_lineage,
                            curr_range: range
                        }); 
                    }


                    // Translate to new destination range
                    let to_translate = &overlap_ranges.source_range_to_translate;
                    let source_range = &next_type_map.source_range;

                    let to_add_to_start = to_translate.start - source_range.start;
                    let to_add_to_end = to_translate.end - source_range.start;

                    let range_to_add = Range {
                        start: next_type_map.dest + to_add_to_start,
                        end: next_type_map.dest+to_add_to_end
                    };

                    // Add new range
                    let mut new_lineage = curr_debug_range.lineage.clone();
                    new_lineage.push(LineageType::MapToNewVal(LineageInfo {
                        range_used_for_decision: next_type_map.source_range.clone(),
                        range_snapshot: range_to_add.clone()
                    }));
                    next.push(DebugRange {
                        lineage: new_lineage,
                        curr_range: range_to_add
                    });
                },
                None => {
                    // Do nothing in this case cause no overlap
                }
            }
        }

        if !overlapped {
            let mut new_lineage = curr_debug_range.lineage.clone();
            new_lineage.push(LineageType::PassedDown);
            next.push(DebugRange {
                lineage: new_lineage,
                curr_range: curr_seed_range.clone() 
            });
        }
        // Iterate to get next seed_range
        i += 1;
    }

    next
}

#[derive(Debug, Clone)]
struct LineageInfo {
    range_used_for_decision: Range<u64>,
    range_snapshot: Range<u64>
}

#[derive(Debug, Clone)]
enum LineageType {
    StartRange(Range<u64>),
    MapToNewVal(LineageInfo),
    TrimmedPortionOfMap(LineageInfo),
    PassedDown,
}

#[derive(Debug, Clone)]
struct DebugRange {
    lineage: Vec<LineageType>,
    curr_range: Range<u64>
}

fn puzzle_2(contents: String) {
    let mut lines = contents.lines();
   
    // Set up this way cause we always start with a swap on the first map line
    let mut current : Vec<DebugRange> = parse_seeds_2(lines.next().unwrap());
    let mut next : Vec<DebugRange> = Vec::new();

    let mut current_next_type_maps = Vec::new();

    for line in lines {
        match parse_line(line) {
            LineType::MapLine(next_type_map) => {
                current_next_type_maps.push(next_type_map);
            },
            LineType::TitleLine => {
                println!("Before Title line");
                //println!("Current: {:?}, Next:  {:?}", current, next);
                //println!("");

                next = update_ranges(current, current_next_type_maps);

                // Reset 
                current = next;
                next = Vec::new();
                current_next_type_maps = Vec::new();

                //println!("After Title line");
                //println!("Current: {:?}, Next: {:?}", current, next);
                //println!("");
            },
            LineType::EmptyLine => {
                // do nothing
            }
        }
    }

    // Have to update ranges one last time
    next = update_ranges(current, current_next_type_maps);

    let mut min = u64::MAX;
    next.iter().for_each(|debug_range| { 
        min = std::cmp::min(min, debug_range.curr_range.start);
        if min == debug_range.curr_range.start {
            //println!("Updated min with: {:?}", debug_range);
            //println!("Updated min to {}", min);
        }
    });

    println!("Lowest Location Num is: {}", min);
}

