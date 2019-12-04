use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

pub fn part_1() -> io::Result<usize> {
    let mut counter = 0;
    for i in 134564..585160 {
        if never_decreases(i) && has_two_same(i) {
            counter += 1;
        }
    }
    Ok(counter)
}

pub fn never_decreases(mut i: usize) -> bool {
    let ls_1 = i % 10;
    i = i / 10;
    let ls_2 = i % 10;
    i = i / 10;
    let ls_3 = i % 10;
    i = i / 10;
    let ls_4 = i % 10;
    i = i / 10;
    let ls_5 = i % 10;
    i = i / 10;
    let ls_6 = i % 10;

    ls_1 >= ls_2 && ls_2 >= ls_3 && ls_3 >= ls_4 && ls_4 >= ls_5 && ls_5 >= ls_6
}

pub fn has_two_same(mut i: usize) -> bool {
    let ls_1 = i % 10;
    i = i / 10;
    let ls_2 = i % 10;
    i = i / 10;
    let ls_3 = i % 10;
    i = i / 10;
    let ls_4 = i % 10;
    i = i / 10;
    let ls_5 = i % 10;
    i = i / 10;
    let ls_6 = i % 10;

    (ls_1 == ls_2) || (ls_2 == ls_3) || (ls_3 == ls_4) || (ls_4 == ls_5) || (ls_5 == ls_6)
}

pub fn non_multiple(mut i: usize) -> bool {
    let mut a: Vec<usize> = vec![];

    for _ in 0..=5 {
        let ls = i % 10;
        a.push(ls);
        println!("{:?}, {:?}", i, ls);
        i = i / 10;
    }
    a.push(100);

    let mut counter = 1000;
    let mut prev_ele = 10;
    //let mut r_val = false;
    for anele in a {
        if prev_ele == anele {
            counter += 1;
        } else {
            if counter == 2 {
                println!("true");
                return true;
            } else {
                counter = 1;
            }
        }
        prev_ele = anele;
    }
    println!("{:?}", false);
    false

    //r_val

    //let mut hm = std::collections::HashMap::<usize, usize>::new();

    //hm.insert(0, 0);
    //hm.insert(1, 0);
    //hm.insert(2, 0);
    //hm.insert(3, 0);
    //hm.insert(4, 0);
    //hm.insert(5, 0);
    //hm.insert(6, 0);
    //hm.insert(7, 0);
    //hm.insert(8, 0);
    //hm.insert(9, 0);

    //for i in a {
    //hm.insert(i, hm[&i] + 1);
    //}

    //for i in hm.keys() {
    //if hm[i] == 2 {
    //return true;
    //}
    //}
    //false
}

pub fn part_2() -> io::Result<usize> {
    let mut counter = 0;
    for i in 134564..585160 {
        if never_decreases(i) && non_multiple(i) {
            counter += 1;
        }
    }
    Ok(counter)
}
