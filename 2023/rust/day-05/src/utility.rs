use std::str::{FromStr,Lines};

pub type Id = u64;

pub struct MappingRule {
    dst_start_id: Id,
    src_start_id: Id,
    range: Id,
}

impl MappingRule {
    pub fn map(&self, input: Id) -> Option<Id> {
        if self.src_start_id <= input && self.src_start_id + self.range > input {
            Some(input + self.dst_start_id - self.src_start_id)
        } else {
            None
        }
    }
}

pub struct Mapping {
    mapping_rules: Vec<MappingRule>,
}

impl Mapping {
    pub fn map(&self, input: Id) -> Id {
        self.mapping_rules
            .iter()
            .filter_map(|rule| rule.map(input))
            .next()
            .unwrap_or(input)
    }
}

pub fn get_min_location_id(seed_iter: impl Iterator<Item = Id>, mappings: &[Mapping]) -> Id {
    seed_iter
        .map(|seed| {
            mappings.iter().fold(seed, |id, mapping| mapping.map(id))
        })
        .min()
        .expect("No seed could be mapped to a location")
}

// ======================= //

pub fn get_seed_numbers(lines: &mut Lines) -> Vec<Id> {
    let line = lines.next().unwrap();
    line[7..]
    .split(' ')
    .map(|s|  Id::from_str(s).expect("Should be able to convert str to integer"))
    .collect::<Vec<Id>>()
}

pub fn get_mappings(lines: &mut Lines) -> Vec<Mapping> {
    let mut mappings = Vec::new();
    for ln in lines {
        if ln.is_empty() {
            continue;
        } else if ln.contains("map") {
            // push a new Mapping struct into mappings
            mappings.push(Mapping {
                mapping_rules: Vec::new()
            });
        } else {
            // construct MappingRule struct
            let rule = ln.split(' ')
                .map(|str| Id::from_str(str).expect("String should be able to parse"))
                .collect::<Vec<Id>>();

            // and push it into the last Mapping struct
            mappings
                .last_mut()
                .expect("Unable to get mapping")
                .mapping_rules
                .push(MappingRule {
                    dst_start_id: rule[0], 
                    src_start_id: rule[1], 
                    range: rule[2] 
                });
        }
    }
    mappings
}