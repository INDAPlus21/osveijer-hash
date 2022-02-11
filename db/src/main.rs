#![allow(unused)]

use std::path::Path;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn hash(str: String) -> usize {
    let mut hash: usize = 0;

    let mut p: usize = 2;
    let m: usize = 1000;
    for _c in str.encode_utf16() {
        hash += _c as usize * p;
        p *= 2;
    }

    hash = hash % m;

    hash
}

#[derive(Clone)]
struct HNode {
    key: usize,
    value: Person
}

impl HNode {
    fn new(_key: &String, _value: &Vec<String>) -> HNode {
        HNode {
            key: hash(_key.to_string()),
            value: Person::new(&_value[0], _value[1].parse().unwrap())
        }
    }

    fn new_key_done(_key: usize, _value: &Vec<String>) -> HNode {
        HNode {
            key: _key,
            value: Person::new(&_value[0], _value[1].parse().unwrap())
        }
    }

    fn print(&self) {
        println!("key: {} value: {:?}", self.key, self.value);
    }
}

#[derive(Clone)]
struct HMap {
    cells: Vec<HNode>,
    count: usize
}

impl HMap {
    fn new() -> HMap {
        HMap {
            cells: Vec::with_capacity(1000),
            count: 0
        }
    }
}

fn add(map: &HMap, _key: &String, _value: &Vec<String>) -> HMap{
    let mut nmap = (*map).clone();
    nmap.cells.push(HNode::new(_key, _value));
    nmap
}

fn read_csv(map: &HMap, path: &str) -> HMap {
    let mut nmap = (*map).clone();
    let lines = read_lines(Path::new(path));
    for line in lines {
        let mut val: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
        if val.len() > 1 {
            let key: String = (&val[0]).to_string();
            val.remove(0);
            nmap.cells.push(HNode::new_key_done(key.parse().unwrap(), &val))
        }
    }
    nmap
}

fn sort(map: &HMap) -> HMap {
    let mut nmap = (*map).clone();

    // bubble sort because it's easy
    let mut changes = 1;
    while changes != 0 {
        changes = 0;
        for i in 0..(nmap.cells.len()-1) {
            if nmap.cells[i].key > nmap.cells[i+1].key {
                let temp = nmap.cells[i+1].clone();
                nmap.cells[i+1] = nmap.cells[i].clone();
                nmap.cells[i] = temp;
                changes += 1;
            }
        }
        
    }

    nmap
}

fn find(map: &HMap, _key: &String) -> usize {
    let key = hash(_key.to_string());

    if !map.cells.iter().any(|n| n.key == key) {
        println!("No data with key {}", _key);
        std::process::exit(1);
    }

    map.cells.iter().position(|n| n.key == key).unwrap()
}

#[derive(Parser)]
struct Args{
    command: String,
    input: Vec<String>
}

#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(_name: &String, _age: u8) -> Person {
        Person {
            name: _name.to_string(),
            age: _age
        }
    }

    fn to_csv(&self) -> String {
        self.name.to_string() + "," + &self.age.to_string()
    }
}

fn read_lines(_p: &Path) -> Vec<String> {
    let lines: Vec<String>;

    match File::open(_p) {
        Ok(f) => {
            lines = io::BufReader::new(f).lines().map(|l| l.ok().unwrap()).collect();
        
        },
        _ => {
            println!("Unable to read file");
            std::process::exit(1);
        }
    };

    lines
}

fn main() {
    let path = "data.csv";
    let mut table = HMap::new();

    table = read_csv(&table, path);

    let args = Args::parse();

    let mut changed = false;

    if args.command == "insert" {
        let mut val = args.input.clone();

        let key: String = (&val[0]).to_string();
        val.remove(0);

        table = add(&table, &key, &val);

        table = sort(&table);

        changed = true;
    }
    else if args.command == "get" {
        let index = find(&table, &args.input[0]);

        table.cells[index].print();
    }
    else if args.command == "all" {
        for i in &table.cells {
            i.print();
        }
    }
    else if args.command == "delete" {
        table.cells.remove(find(&table, &args.input[0]));

        changed = true;
    }
    else {
        println!("No command {}", args.command);
        std::process::exit(1);
    }

    if changed {
        let mut data = "".to_string();
        for i in &table.cells {
            data += &(i.key.to_string() + "," + &i.value.to_csv() + "\n");
        }
        let file = File::create(path).unwrap();
        let mut file = io::BufWriter::new(file);
        file.write_all(data.as_bytes());
    }
}
