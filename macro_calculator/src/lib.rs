use json::JsonValue;
use json::object;
pub struct Food {
  pub  name: String,
   pub calories: (String, String),
    pub proteins: f64,
     pub fats: f64,
   pub  carbs: f64,
    pub nbr_of_portions: f64,
}
pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let kcal_str = food.calories.1.trim_end_matches("kcal");
        let kcal: f64 = kcal_str.parse().unwrap_or(0.0);


        let portions = food.nbr_of_portions;

        total_cals += kcal * portions;
        total_carbs += food.carbs * portions;
        total_proteins += food.proteins * portions;
        total_fats += food.fats * portions;
    }

    fn round_value(v: f64) -> f64 {
        let rounded = (v * 100.0).round() / 100.0;
        if (rounded * 10.0) % 1.0 == 0.0 {
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    }

    object! {
        "cals" => round_value(total_cals),
        "carbs" => round_value(total_carbs),
        "proteins" => round_value(total_proteins),
        "fats" => round_value(total_fats),
    }
}
