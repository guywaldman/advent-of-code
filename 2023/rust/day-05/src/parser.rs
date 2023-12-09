use std::error::Error;

use crate::models::{Almanac, CategoryMapping, RangeMapping};

pub fn parse_alamanac(input: &str) -> Result<Almanac, Box<dyn Error>> {
    let first_line = input.lines().next().unwrap();
    let seeds = first_line.split(':').nth(1).unwrap().trim();
    let seeds = seeds.split(' ').map(|seed| seed.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let alamanc_input = input.lines().skip(2).collect::<Vec<_>>().join("\n");
    let mappings = alamanc_input
        .split("\n\n")
        .map(|category_mapping| parse_category_mapping(category_mapping.trim()).unwrap())
        .collect::<Vec<_>>();
    let alamanac = Almanac { seeds, mappings };
    Ok(alamanac)
}

fn parse_category_mapping(input: &str) -> Result<CategoryMapping, Box<dyn Error>> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let map_title = first_line.split(' ').next().unwrap();
    let (src_cat, dest_cat) = match &map_title.split('-').collect::<Vec<_>>()[..] {
        [src, _, dest] => (*src, *dest),
        _ => panic!("Invalid map title"),
    };
    let line_mappings = lines.take_while(|line| !line.trim().is_empty()).collect::<Vec<_>>();
    let mappings = line_mappings
        .iter()
        .filter_map(|line| {
            let mut line_parts = line.split(' ');
            let dest_start = line_parts.next().unwrap().parse::<usize>().ok()?;
            let src_start = line_parts.next()?.parse::<usize>().ok()?;
            let length = line_parts.next()?.parse::<usize>().ok()?;
            Some(RangeMapping { dest_start, src_start, length })
        })
        .collect::<Vec<_>>();
    Ok(CategoryMapping {
        src_cat: src_cat.to_owned(),
        dest_cat: dest_cat.to_owned(),
        mappings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_almanac_basic() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48";
        let almanac = parse_alamanac(input).unwrap();
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.mappings.len(), 1);
    }

    #[test]
    fn parse_map_basic() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let CategoryMapping { src_cat, dest_cat, mappings } = parse_category_mapping(input).unwrap();
        assert_eq!(src_cat, "seed");
        assert_eq!(dest_cat, "soil");
        assert_eq!(mappings.len(), 2);
        let RangeMapping { dest_start, src_start, length } = mappings[0];
        assert_eq!(dest_start, 50);
        assert_eq!(src_start, 98);
        assert_eq!(length, 2);
        let RangeMapping { dest_start, src_start, length } = mappings[1];
        assert_eq!(dest_start, 52);
        assert_eq!(src_start, 50);
        assert_eq!(length, 48);
    }
}
