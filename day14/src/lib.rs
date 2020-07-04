use std::collections::HashMap;

pub struct CookBook {
    data: Vec<Recipe>
}

#[derive(Clone)]
pub struct Recipe {
    inputs: Vec<RecipeIngredient>,
    output: RecipeIngredient,
}

#[derive(Clone)]
struct RecipeIngredient {
    name: String,
    quantity: usize
}

impl CookBook {

    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.data.push(recipe);
    }

    pub fn produce_from(&self, input_ingredient: &str, output_ingredient: &str, output_quantity: usize) -> usize {
        let mut queue = self.get_recipe(output_ingredient).inputs;
        for item in &mut queue {
            item.quantity *= output_quantity;
        }

        let mut left_overs = HashMap::new();
        let mut total_input_ingredient = 0;

        while !queue.is_empty() {
            let input = queue.pop().unwrap();

            if input.name == input_ingredient {
                total_input_ingredient += input.quantity;
            } else {
                let recipe = self.get_recipe(&input.name);

                let input_left_over = left_overs.entry(input.name).or_insert(0);

                let input_quantity_needed = if &input.quantity >= input_left_over {
                    let result = input.quantity - *input_left_over;
                    *input_left_over = 0;
                    result
                } else {
                    *input_left_over -= input.quantity;
                    0
                };

                if input_quantity_needed > 0 {
                    let multiplier = (input_quantity_needed as f64 / recipe.output.quantity as f64).ceil() as usize;
                    
                    let left_over = recipe.output.quantity * multiplier - input_quantity_needed;
                    if left_over > 0 {
                        *input_left_over += left_over;
                    }

                    for mut item in recipe.inputs {
                        item.quantity *= multiplier;
                        queue.push(item);
                    }
                }
            }
        };

        total_input_ingredient
    }

    pub fn produce_maximum_from(&self, input_ingredient: &str, input_ingredient_quantity: usize, output_ingredient: &str) -> usize {
        let mut upper_output = 1;

        // Quick exponential search to approximate result
        loop {
            let input_used_quantity = self.produce_from(input_ingredient, output_ingredient, upper_output);
            
            if input_used_quantity < input_ingredient_quantity {
                upper_output *= 2;
            } else {
                break;
            }
        }

        // Binary search to match exact result
        let mut lower_output = upper_output / 2;
        let mut last_middle_value = 0;
        loop {
            let middle_value = (upper_output + lower_output) / 2;
            let input_used_quantity = self.produce_from(input_ingredient, output_ingredient, middle_value);
            
            if input_used_quantity < input_ingredient_quantity {
                lower_output = middle_value;
            } else {
                upper_output = middle_value;
            }

            if last_middle_value == middle_value {
                break;
            } else {
                last_middle_value = middle_value;
            }
        }

        last_middle_value
    }

    fn get_recipe(&self, recipe_name: &str) -> Recipe {
        self.data.iter().find(|recipe| recipe.output.name == recipe_name).expect("Invalid recipe").clone()
    }
}

impl Default for CookBook {
    fn default() -> Self {
        Self::new()
    }
}

impl Recipe {
    pub fn parse(raw_recipe: &str) -> Self {
        let mut recipe_components = raw_recipe.split("=>");
        
        let mut inputs_buffer = Vec::new();
        let inputs_data = recipe_components.next().expect("Invalid recipe. No inputs").split(',');
        for input in inputs_data {
            let mut input_data_split = input.trim().split(' ');
            let input_quantity = input_data_split.next().expect("Invalid recipe output quantity").parse::<usize>().expect("Invalid number");
            let input_name = input_data_split.next().expect("Invalid recipe output name");

            inputs_buffer.push(RecipeIngredient::from(input_name, input_quantity));
        }
        
        let mut output_data = recipe_components.next().expect("Invalid recipe. No output").trim().split(' ');
        let output_quantity = output_data.next().expect("Invalid recipe output quantity").parse::<usize>().expect("Invalid number");
        let output_name = output_data.next().expect("Invalid recipe output name");

        Self { inputs: inputs_buffer, output: RecipeIngredient::from(output_name, output_quantity) }
    }
}

impl RecipeIngredient {
    fn from(name: &str, quantity: usize) -> Self {
        Self { name: String::from(name), quantity }
    }
}