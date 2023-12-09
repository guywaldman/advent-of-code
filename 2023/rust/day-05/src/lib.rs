pub mod models;
pub mod parser;

use models::Almanac;

use std::error::Error;

impl Almanac {
    fn map_category(&self, src_cat: &str, dest_cat: &str, value: usize) -> Option<usize> {
        let mapping = self.mappings.iter().find(|m| m.src_cat == src_cat && m.dest_cat == dest_cat)?;
        for range in &mapping.mappings {
            if value >= range.src_start && value < (range.src_start + range.length) {
                let dest_value = range.dest_start + (value - range.src_start);
                return Some(dest_value);
            }
        }
        // Values that aren't mapped, correspond to the same value in the destination category.
        Some(value)
    }

    fn map_seed_to_location(&self, seed: usize) -> Option<usize> {
        let mut curr_cat = "seed";
        let mut curr_value = seed;
        for _ in 0..=self.mappings.len() {
            let mapping = &self.mappings.iter().find(|m| m.src_cat == curr_cat)?;
            let dest_cat = &mapping.dest_cat;
            let dest_value = self.map_category(curr_cat, dest_cat, curr_value)?;
            if dest_cat == "location" {
                return Some(dest_value);
            }
            curr_cat = dest_cat;
            curr_value = dest_value;
        }
        None
    }
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let almanac = parser::parse_alamanac(input)?;
    let min_location = almanac.seeds.iter().map(|s| almanac.map_seed_to_location(*s).unwrap()).min().unwrap();
    Ok(min_location.to_string())
}

pub fn solve_part_2(_input: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_alamanac;

    #[test]
    fn cat_mapping_basic() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48";
        let almanac = parse_alamanac(input).unwrap();
        for i in 1..10 {
            assert_eq!(i, almanac.map_category("seed", "soil", i).unwrap());
        }
        assert_eq!(almanac.map_category("seed", "soil", 49).unwrap(), 49);
        assert_eq!(almanac.map_category("seed", "soil", 50).unwrap(), 52);
        assert_eq!(almanac.map_category("seed", "soil", 51).unwrap(), 53);
        assert_eq!(almanac.map_category("seed", "soil", 62).unwrap(), 64);
        assert_eq!(almanac.map_category("seed", "soil", 96).unwrap(), 98);
        assert_eq!(almanac.map_category("seed", "soil", 97).unwrap(), 99);
        assert_eq!(almanac.map_category("seed", "soil", 98).unwrap(), 50);
        assert_eq!(almanac.map_category("seed", "soil", 99).unwrap(), 51);
        assert_eq!(almanac.map_category("seed", "soil", 100).unwrap(), 100);
        assert_eq!(almanac.map_category("seed", "soil", 101).unwrap(), 101);
    }
}
