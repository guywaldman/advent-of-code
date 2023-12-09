#[derive(Debug)]
pub struct RangeMapping {
    pub dest_start: usize,
    pub src_start: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct CategoryMapping {
    pub src_cat: String,
    pub dest_cat: String,
    pub mappings: Vec<RangeMapping>,
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<usize>,
    pub mappings: Vec<CategoryMapping>,
}
