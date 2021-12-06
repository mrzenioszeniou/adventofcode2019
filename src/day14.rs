use std::collections::HashMap;

pub fn part1() -> usize {
    let mut recipes = parse();
    let mut raw = HashMap::from([(String::from("FUEL"), 1)]);

    while !recipes.is_empty() {
        let production = raw
            .keys()
            .find(|ingredient| {
                recipes
                    .values()
                    .all(|recipe| !recipe.input.contains_key(*ingredient))
            })
            .unwrap()
            .clone();

        let mut production_amount = raw.remove(&production).unwrap();

        let production_recipe = recipes.remove(&production).unwrap();

        while production_amount > 0 {
            for (ingredient, ingredient_amount) in production_recipe.input.iter() {
                raw.entry(ingredient.clone())
                    .and_modify(|c| *c += ingredient_amount)
                    .or_insert(*ingredient_amount);
            }

            production_amount = production_amount.saturating_sub(production_recipe.output);
        }
    }

    assert_eq!(raw.len(), 1);
    raw.remove("ORE").unwrap()
}

pub fn part2() -> usize {
    let _ = parse();
    42
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

struct Recipe {
    input: HashMap<String, usize>,
    output: usize,
}
