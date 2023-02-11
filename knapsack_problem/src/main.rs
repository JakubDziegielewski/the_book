fn main() {
    let foods: Vec<[f64; 2]> = Vec::from([[10.0, 5.0], [4.0, 3.0], [14.0, 6.0], [33.0, 10.0]]);
    let capacity: f64 = 5.0;
    let mut efficiencies: Vec<(usize, f64)> = Vec::new();
    for (i, food) in foods.iter().enumerate()  {
        let efficiency = food[0] / food[1];
        efficiencies.push((i, efficiency));
    }
    efficiencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let mut free_space: f64 = capacity;
    let mut food_taken: usize = 0;
    let mut total_value: f64 = 0.0;

        for efficency in &efficiencies{
            if free_space >= foods[efficency.0][1]{
                total_value = total_value + foods[efficency.0][0];
                free_space = free_space - foods[efficency.0][1];
                food_taken = food_taken + 1;
            }
            else {
                let percentage_fitting: f64 = free_space/foods[efficency.0][1];
                total_value = total_value + foods[efficency.0][0] * percentage_fitting;
                free_space = free_space - foods[efficency.0][1] * percentage_fitting;
                food_taken = food_taken + 1;
            }
            if !(free_space > 0.0){
                break;
            } 
            
        }

    println!("{}", format!("{:.2}", total_value));
}
