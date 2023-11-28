use crate::config::from_config::Config;
use crate::coordinate_parser::cparser::Coordinates;
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::slice::Iter;

pub fn compute_conf(files: Iter<String>) -> Result<Config> {
    let mut config = Config::default();
    for file in files {
        let mut content = "".to_string();
        get_content(&mut content, file)?;
        config.merge(&content)?;
    }
    Ok(config)
}

pub fn compute(points: &HashSet<Coordinates>, start: Coordinates) -> Vec<(Coordinates,f64)> {
    let mut unvisited = points.clone();
    let mut path = Vec::with_capacity(points.len());
    let mut current_point = start;
    while !unvisited.is_empty() {
        unvisited.remove(&current_point);

        let next_point = unvisited
            .iter()
            .min_by(|&a, &b| {
                current_point
                    .dist(a)
                    .partial_cmp(&current_point.dist(b))
                    .unwrap()
            })
            .unwrap_or(&start);
        path.push((current_point, start.dist(&current_point)));
        current_point = *next_point;
    }
    path.push((start, 0.0));
    path
}

fn get_dist(p0: &Coordinates, p1: &&Coordinates) -> f64 {
    p0.dist(p1)
}

fn get_content(p0: &mut String, p1: &String) -> Result<()> {
    let f = File::open(p1);
    if f.is_err() {
        return Err(anyhow!("Unable to open file at path: {p1}"));
    }
    let mut f = f.unwrap();
    match f.read_to_string(p0) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("{}", e.to_string())),
    }
}
