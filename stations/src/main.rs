use rand::Rng;
fn main() {
    let mut full_tank;
    let mut distance: u32;
    let mut stations_array = [0,0,0,0,0,0,0,0,0,0];
    
    loop {
        full_tank = rand::thread_rng().gen_range(500..=800);
        distance = rand::thread_rng().gen_range(500..=5000);
        let mut iterator: u32 = 1;
        for mut _i in stations_array.iter_mut(){
            *_i = iterator * rand::thread_rng().gen_range(100..=300);
            iterator = iterator + 1;
        }
        stations_array.sort();
        let stations = &stations_array[..];
        let res: i32 = find_how_many_stops(full_tank, distance, stations);
        if res != -1 {
            println!("Number of stops: {res}");
        } else {
            println!("Cannot reach final destination");
        }
        let res_worse: i32 = find_how_many_stops_worse(full_tank, distance, stations);
        if res != res_worse {
            println!("ALERT");
            println!("{}, {}, {:?}", full_tank, distance, stations);
            break;
        }
    }
}

fn find_how_many_stops(full_tank: u32, distance: u32, stations: &[u32]) -> i32 {
    let mut position: usize = 0;
    let mut stops: i32 = 0;
    let mut position_change: usize = 0;
    while distance > stations[position] + full_tank {
        if !(stations.len() > position + 1)
            || stations[position + 1] > stations[position] + full_tank
        {
            return -1;
        } else {
            position_change = position_change + 1;
            if !(stations.len() > position + position_change + 1)
                || stations[position] + full_tank < stations[position + position_change + 1]
            {
                position = position + position_change;
                position_change = 0;
                stops = stops + 1;
            }
        }
    }
    stops
}

fn find_how_many_stops_worse(full_tank: u32, distance: u32, stations: &[u32]) -> i32 {
    let mut position: usize = 0;
    let mut stops: i32 = 0;
    let mut position_change: usize;
    while distance > stations[position] + full_tank {
        position_change = find_farthest_stop(full_tank, &stations[position..]);
        if position_change == 0 {
            return -1;
        } else {
            position = position + position_change;
            stops = stops + 1;
            //println!("Stopped at: {}", stations[position])
        }
    }
    stops
}
fn find_farthest_stop(full_tank: u32, stations: &[u32]) -> usize {
    let mut position_change: usize = 0;
    while stations.len() > position_change + 1
        && stations[position_change + 1] <= stations[0] + full_tank
    {
        position_change = position_change + 1;
    }
    position_change
}
/*fn find_farthest_stop(full_tank: u32, stations: &[u32], mut position: u32) -> u32 {
    for station in stations.iter().rev() {
        if station - position <= full_tank {
            position = *station;
            break;
        }
    }
    position
}*/
