use crate::computation::compute;
use crate::coordinate_parser::cparser::Coordinates;
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Config {
    paths: HashMap<Coordinates, String>,
    head: Coordinates,
}

impl Config {
    pub fn merge(&mut self, input: &str) -> Result<()> {
        let val = serde_json::from_str::<Value>(input)?;
        let arr = val
            .as_array()
            .ok_or(anyhow!("Content of the file is not JSON arr"))?;
        let cur = &mut self.paths;
        for v in arr {
            let (c, l) = Coordinates::from(v).ok_or(anyhow!("Invalid Child: {}", v))?;
            cur.insert(c, l);
            if cur.len() == 1 {
                self.head = c;
            }
        }
        Ok(())
    }
    pub fn get(&self, coordinates: &Coordinates) -> String {
        self.paths.get(coordinates).unwrap().clone()
    }
    pub fn compute(&self) -> Vec<Coordinates> {
        let mut v = vec![];
        for k in self.paths.keys() {
            v.push(*k);
        }
        compute::compute(&v.into_iter().collect(), self.head)
    }
}