use std::collections::{BTreeMap, HashMap};

pub fn part1() -> usize {
    let mut factory = Factory::new(parse());
    factory.set_material("ORE", usize::MAX);
    factory.produce("FUEL");
    usize::MAX - factory.get_material("ORE")
}

pub fn part2() -> usize {
    42
}

struct Factory {
    recipes: HashMap<String, Recipe>,
    materials: BTreeMap<String, usize>,
}

impl Factory {
    fn new(recipes: HashMap<String, Recipe>) -> Self {
        Self {
            recipes,
            materials: BTreeMap::new(),
        }
    }

    fn get_material(&self, material: &str) -> usize {
        *self.materials.get(material).unwrap_or(&0)
    }

    fn set_material(&mut self, material: &str, amount: usize) {
        self.materials
            .entry(material.to_string())
            .and_modify(|a| *a = amount)
            .or_insert(amount);
    }

    fn print_materials(&self) {
        println!("Materials:");
        for (material, amount) in self.materials.iter() {
            if *amount != 0 {
                println!("  {:>6}{:>10}", material, amount);
            }
        }
    }

    fn produce(&mut self, material: &str) -> Option<()> {
        let Recipe { input, output } = self.recipes.get(material)?.clone();

        for (ingredient, amount) in input.into_iter() {
            while self.get_material(&ingredient) < amount {
                self.produce(&ingredient)?;
            }

            self.set_material(&ingredient, self.get_material(&ingredient) - amount);
        }

        self.set_material(material, self.get_material(material) + output);

        Some(())
    }
}

fn parse() -> HashMap<String, Recipe> {
    let content = std::fs::read_to_string("res/day14.txt").unwrap();

    let mut recipes = HashMap::new();

    for line in content.split('\n') {
        let mut recipe = line.split(" => ");

        let mut input = HashMap::new();
        for ingredient in recipe.next().unwrap().split(", ") {
            let (amount, ingredient) = sscanf::scanf!(ingredient, "{} {}", usize, String).unwrap();
            input.insert(ingredient, amount);
        }

        let (output, produce) =
            sscanf::scanf!(recipe.next().unwrap(), "{} {}", usize, String).unwrap();

        recipes.insert(produce, Recipe { input, output });
    }

    recipes
}

#[derive(Clone)]
struct Recipe {
    input: HashMap<String, usize>,
    output: usize,
}
