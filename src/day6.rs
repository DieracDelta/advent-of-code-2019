use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn parse_input(hm: &mut HashMap<String, HashSet<String>>) -> io::Result<bool> {
    for x in BufReader::new(File::open("./input/input-day-6.txt")?).lines() {
        let a: Vec<String> = x
            .unwrap()
            .split(')')
            .map(|x| -> String { x.to_owned() })
            .collect();
        println!("A: {:?}", a);
        if !hm.contains_key(&a[0]) {
            hm.insert(a[0].clone(), HashSet::new());
        }
        if !hm.contains_key(&a[1]) {
            hm.insert(a[1].clone(), HashSet::new());
        }
        hm.get_mut(&a[0]).unwrap().insert(a[1].clone());
        hm.get_mut(&a[1]).unwrap().insert(a[0].clone());
    }
    Ok(true)
}

pub fn part_1() -> io::Result<usize> {
    let mut hm = HashMap::<String, HashSet<String>>::new();
    if !parse_input(&mut hm).unwrap() {
        panic!("problem parsing")
    };
    println!("hm {:?}", hm);
    let cur_set = &mut HashSet::<String>::new();
    //let queue: VecDeque<String> = VecDeque::new();
    cur_set.insert("SAN".to_owned());
    let seen = &mut HashSet::<String>::new();
    let mut num_orbits = 0;
    let mut depth = 0;
    while !cur_set.is_empty() {
        let new_eles_filter = cur_set.drain().filter(|x| !seen.contains(x));
        let seen_temp = &mut HashSet::<String>::new();
        let new_eles: Vec<String> = new_eles_filter
            .flat_map(|x| -> Vec<String> {
                seen_temp.insert(x.clone());
                num_orbits += depth;
                hm.get(&x).unwrap().clone().drain().collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();
        seen.extend(seen_temp.clone().into_iter());
        cur_set.extend(new_eles);
        depth += 1;
    }
    Ok(num_orbits)
}
pub fn part_2() -> io::Result<usize> {
    let mut hm = HashMap::<String, HashSet<String>>::new();
    if !parse_input(&mut hm).unwrap() {
        panic!("problem parsing")
    };
    let cur_set = &mut HashSet::<String>::new();
    //let queue: VecDeque<String> = VecDeque::new();
    cur_set.insert("YOU".to_owned());
    let seen = &mut HashSet::<String>::new();
    let mut depth = -1;
    while !cur_set.is_empty() {
        let new_eles_filter = cur_set.drain().filter(|x| !seen.contains(x));
        let seen_temp = &mut HashSet::<String>::new();
        let new_eles: Vec<String> = new_eles_filter
            .flat_map(|x| -> Vec<String> {
                seen_temp.insert(x.clone());
                hm.get(&x).unwrap().clone().drain().collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();
        let a = hm.get("SAN").unwrap().clone();
        assert!(a.len() == 1);
        let node = a.iter().next().unwrap();
        if seen_temp.contains(&node.clone()) {
            return Ok(depth as usize);
        }
        seen.extend(seen_temp.clone().into_iter());
        cur_set.extend(new_eles);
        depth += 1;
    }
    return Ok(0);
}
