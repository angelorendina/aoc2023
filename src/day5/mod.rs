use std::collections::BTreeMap;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day5/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day5/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut lines = input.lines().filter(|line| !line.is_empty());

    // Read the seeds into a Vec<u64>
    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Skip an empty line
    lines.next();

    // Helper to read each "map" into a map `source -> (length, target)`
    let mut read_chunk = || -> _ {
        let mut map = BTreeMap::<u64, (u64, u64)>::new();
        for line in lines.by_ref() {
            if line.ends_with("map:") {
                break;
            }
            let mut line = line.split_whitespace();
            let target = line.next().unwrap().parse().unwrap();
            let source = line.next().unwrap().parse().unwrap();
            let length = line.next().unwrap().parse().unwrap();
            map.insert(source, (length, target));
        }
        map
    };

    // Read all the maps
    let layers = vec![
        read_chunk(),
        read_chunk(),
        read_chunk(),
        read_chunk(),
        read_chunk(),
        read_chunk(),
        read_chunk(),
    ];

    let mut smallest = u64::MAX;
    // for each seed
    for mut x in seeds {
        // try mapping it through each layer
        for layer in &layers {
            let mut mapped = None;
            // for every `(source, source+length)` window
            for (&source, &(length, target)) in layer.iter() {
                let range = source..(source + length);
                // if the seed is in the window
                if range.contains(&x) {
                    // map it through the windows' target with the correct offset
                    let offset = x - source;
                    mapped = Some(target + offset);
                }
            }

            // Update x if it was mapped through any of the windows
            x = mapped.unwrap_or(x);
        }

        // Is it the smaller so far?
        smallest = smallest.min(x);
    }

    smallest
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day5/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day5/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut lines = input.lines().filter(|line| !line.is_empty());

    // Read the seeds into a Vec<u64>
    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Skip an empty line
    lines.next();

    // This time the maps are represented as segmentations of the following form:
    // what was "source -> (length, target)"
    // becomes two entries `([source, target)` and `(]source+length, _)`
    // where the last value is either the start of another offset window, or identical if not a window
    let mut read_chunk = || -> _ {
        let mut map = BTreeMap::<u64, u64>::from([(0, 0)]);
        for line in lines.by_ref() {
            if line.ends_with("map:") {
                break;
            }
            let mut line = line.split_whitespace();
            let target = line.next().unwrap().parse().unwrap();
            let source = line.next().unwrap().parse().unwrap();
            let length = line.next().unwrap().parse::<u64>().unwrap();
            map.insert(source, target);
            map.entry(source + length).or_insert(source + length);
        }
        map
    };

    // Read all maps
    let layers = vec![
        read_chunk(),
        read_chunk(),
        read_chunk(),
        read_chunk(),
        read_chunk(),
        read_chunk(),
        read_chunk(),
    ];

    // Seeds are also represented as a segmentation, where each point is either the start
    // or the end of a segment, e.g. [0, 5] u [9, 10] is seens as
    // [0, ]6, [9, ]11
    // or more precisely
    // (0, true) (6, false) (9, true) (11, false)
    // Each segment can be decoded by a consecutive stretches of `true`s up to the first `false`.
    let mut segmentation = BTreeMap::<u64, bool>::new();
    for chunk in seeds.chunks_exact(2) {
        let &[start, length] = chunk else {
            unreachable!()
        };
        segmentation.insert(start, true);
        segmentation.entry(start + length).or_insert(false);
    }

    // Now that both seeds and maps are segmented, they can be combined
    for layer in layers {
        // Add partition points in the segmentations, keeping track if this happened within an existing segment.
        // So that there is a boundary for each segment AND map boundary.
        for &map_boundary in layer.keys() {
            let last_segment_boundary = segmentation.range(..=map_boundary).last();
            if let Some((_, &spanning)) = last_segment_boundary {
                segmentation.insert(map_boundary, spanning);
            } else {
                segmentation.insert(map_boundary, false);
            }
        }

        // Each boundary can now be mapped through the layer.
        // Either it stays put, or it gets shifted somewhere else.
        // The segment-spanning property is preserved.
        let mut new_segmentation = BTreeMap::<u64, bool>::new();
        for (segment_boundary, spanning) in segmentation {
            let last_map_boundary = layer.range(..=segment_boundary).last();
            match last_map_boundary {
                None => {
                    new_segmentation.insert(segment_boundary, spanning);
                }
                Some((&source, &target)) => {
                    let delta = segment_boundary - source;
                    new_segmentation.insert(target + delta, spanning);
                }
            }
        }

        // Simplify, keeping only the minimal set of boundaries for the new segmentation.
        let mut current_spanning = false;
        segmentation = BTreeMap::new();
        for (boundary, spanning) in new_segmentation {
            if spanning != current_spanning {
                current_spanning = spanning;
                segmentation.insert(boundary, spanning);
            }
        }
    }

    // Pick the start of the first segment :)
    segmentation
        .into_iter()
        .find_map(|(boundary, spanning)| spanning.then_some(boundary))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 35);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 46);
    }
}
