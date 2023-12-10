use crate::parser::Seeds;

enum MapResult {
    Less,
    Greater,
    Some(i64)
}

#[derive(Debug)]
pub struct MapRange {
    start: i64,
    len: i64,
    offset: i64
}

impl MapRange {
    pub fn new(start: i64, len: i64, offset: i64) -> Self {
        Self { start, len, offset }
    }

    pub(self) fn map(&self, val: i64) -> MapResult {
        if self.start > val {
            MapResult::Greater
        } else if self.start + self.len <= val {
            MapResult::Less
        } else {
            MapResult::Some(self.offset + val)
        }
    }
}

#[derive(Debug)]
pub struct CatMap {
    from: String,
    to: String,
    ranges: Vec<MapRange>
}

impl PartialEq for CatMap {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
    }
}

impl CatMap {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to, ranges: vec![] }
    }

    pub fn add_range(&mut self, range: MapRange) {
        self.ranges.push(range)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SortedCatMap {
    from: String,
    to: String,
    ranges: Vec<MapRange>
}

impl SortedCatMap {
    pub fn from_unsorted(unsorted: CatMap) -> Self {
        let mut new: Self = Self { from: unsorted.from, to: unsorted.to, ranges: unsorted.ranges };
        new.ranges.sort_by(|a, b| a.start.cmp(&b.start));
        new
    }

    pub fn map_seeds(&self, seeds: &mut Seeds) {
        for seed in seeds.0.iter_mut() {
            for range in self.ranges.iter() {
                match range.map(*seed) {
                    MapResult::Greater => break,
                    MapResult::Less => {},
                    MapResult::Some(mapped) => {
                        *seed = mapped;
                        break
                    }
                }
            }
        }
    }
}

pub fn closest_seed(mut seeds: Seeds, maps: Vec<SortedCatMap>) -> i64 {
    for map in maps {
        map.map_seeds(&mut seeds);
    }
    *seeds.0.iter().min().unwrap()
}