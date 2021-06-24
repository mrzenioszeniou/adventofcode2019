use std::{collections::HashMap, fs::File, io::Read};

use regex::Regex;

pub fn part1() -> usize {
  let root = parse();
  root.orbits(1)
}

pub fn part2() -> usize {
  let root = parse();

  let mut path_you = root.path_to("YOU").expect("No path to 'YOU' found");
  let mut path_san = root.path_to("SAN").expect("No path to 'SAN' found");

  while path_you[0] == path_san[0] {
    path_you.remove(0);
    path_san.remove(0);
  }

  path_you.len() + path_san.len() - 2
}

struct Node {
  name: String,
  moons: Vec<Node>,
}

impl Node {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      moons: vec![],
    }
  }

  pub fn attach_all(&mut self, orbits: &mut HashMap<&str, Vec<&str>>) {
    if orbits.contains_key(self.name.as_str()) {
      let orbiters = orbits.remove(self.name.as_str()).expect("IMPOSSIBRU");

      for orbiter in orbiters {
        let mut orbiter = Self::new(orbiter);

        orbiter.attach_all(orbits);

        self.moons.push(orbiter);
      }
    }
  }

  pub fn orbits(&self, depth: usize) -> usize {
    self.moons.iter().map(|m| depth + m.orbits(depth + 1)).sum()
  }

  pub fn path_to(&self, to: &str) -> Option<Vec<&str>> {
    if self.name == to {
      return Some(vec![self.name.as_str()]);
    }

    for moon in self.moons.iter() {
      match moon.path_to(to) {
        Some(mut path) => {
          path.insert(0, self.name.as_str());
          return Some(path);
        }
        _ => {}
      }
    }

    None
  }
}

fn parse() -> Node {
  let mut content = String::new();

  File::open("res/day6.txt")
    .unwrap()
    .read_to_string(&mut content)
    .unwrap();

  let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();

  let re = Regex::new("([A-Z0-9]+)\\)([A-Z0-9]+)").expect("Could not parse regex");

  for line in content.trim().split_whitespace() {
    let caps = re.captures(line).unwrap();

    let orbitee_name = caps.get(1).unwrap().as_str();
    let orbiter_name = caps.get(2).unwrap().as_str();

    match orbits.get_mut(orbitee_name) {
      Some(orbitee) => {
        orbitee.push(orbiter_name);
      }
      None => {
        orbits.insert(orbitee_name, vec![orbiter_name]);
      }
    }
  }
  let mut root = Node::new("COM");

  root.attach_all(&mut orbits);

  root
}
