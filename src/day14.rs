use std::collections::HashMap;

pub fn part1() -> usize {
    let mut factory = Factory::new(parse());
    factory.set_material("ORE", usize::MAX);
    factory.produce("FUEL", 1);
    usize::MAX - factory.get_material("ORE")
}

pub fn part2() -> usize {
    let mut init_factory = Factory::new(parse());
    init_factory.set_material("ORE", 1_000_000_000_000);

    let mut high = 1_000_000_000_000;
    let mut low = 0;

    while high != low + 1 {
        let curr = (high + low) / 2;

        let mut factory = init_factory.clone();
        if factory.produce("FUEL", curr).is_some() {
            low = curr;
        } else {
            high = curr;
        }
    }

    low
}

#[derive(Clone)]
struct Recipe {
    input: HashMap<String, usize>,
    output: usize,
}

#[derive(Clone)]
struct Factory {
    recipes: HashMap<String, Recipe>,
    materials: HashMap<String, usize>,
}

impl Factory {
    fn new(recipes: HashMap<String, Recipe>) -> Self {
        Self {
            recipes,
            materials: HashMap::new(),
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

    fn produce(&mut self, req_material: &str, req_amount: usize) -> Option<()> {
        let Recipe { input, output } = self.recipes.get(req_material)?.clone();

        // Amount of times the recipe must be executed
        let multiplier = req_amount.div_ceil(output);

        for (ingredient, amount) in input.into_iter() {
            let missing_amount =
                (amount * multiplier).saturating_sub(self.get_material(&ingredient));

            if missing_amount > 0 {
                self.produce(&ingredient, missing_amount)?;
            }

            self.set_material(
                &ingredient,
                self.get_material(&ingredient) - amount * multiplier,
            );
        }

        self.set_material(
            req_material,
            self.get_material(req_material) + multiplier * output,
        );

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
