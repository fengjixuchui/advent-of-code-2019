mod utility;
use std::collections::HashMap;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

struct Planet {
    pub name: String,
    pub direct_planets_orbiting: Vec<String>,
    pub orbiting_planet: Option<String>,
}

fn part_2() {
    let input = load_file("resources/day_6_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_new_line(input);

    let mut planets: HashMap<String, Planet> = HashMap::new();

    for id in ids {
        let orbits = split_by(id, ")".parse().unwrap());
        let left_planet_name = &orbits[0];
        let right_planet_name = &orbits[1];
        let left_planet = planets
            .entry(left_planet_name.parse().unwrap())
            .or_insert(Planet {
                name: left_planet_name.parse().unwrap(),
                direct_planets_orbiting: vec![],
                orbiting_planet: None,
            });
        left_planet
            .direct_planets_orbiting
            .push(right_planet_name.parse().unwrap());
        let right_planet = planets
            .entry(right_planet_name.parse().unwrap())
            .or_insert(Planet {
                name: right_planet_name.parse().unwrap(),
                direct_planets_orbiting: vec![],
                orbiting_planet: None,
            });
        right_planet.orbiting_planet = Some(left_planet_name.parse().unwrap());
    }

    println!("Created planets");

    //Calculate orbits
    let mut total_orbits = 0;
    let keys = planets.keys().clone();
    let mut orbits_map: HashMap<String, i32> = HashMap::new();
    for planet_name in keys {
        total_orbits += get_planet_orbits(&mut orbits_map, &planets, planet_name);
    }

    //Make a list of all planets we visit
    let mut you_list =
        get_vec_of_orbitting_planets(&mut orbits_map, &planets, &String::from("YOU"));
    let mut san_list =
        get_vec_of_orbitting_planets(&mut orbits_map, &planets, &String::from("SAN"));

    loop {
        if you_list.last().unwrap() == san_list.last().unwrap() {
            you_list.pop();
            san_list.pop();
        } else {
            break;
        }
    }
    println!("You List: {:?}", you_list);
    println!("SAN List: {:?}", san_list);

    stopwatch.print_elapsed();
    println!("Result: {}", you_list.len() + san_list.len());
}

fn get_vec_of_orbitting_planets(
    orbits_map: &mut HashMap<String, i32>,
    planets: &HashMap<String, Planet>,
    planet_name: &String,
) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    let mut current_planet_name_optional = &planets.get(planet_name).unwrap().orbiting_planet;
    loop {
        if let Some(current_planet_name) = current_planet_name_optional {
            list.push(current_planet_name.parse().unwrap());
            current_planet_name_optional =
                &planets.get(current_planet_name).unwrap().orbiting_planet;
        } else {
            break;
        }
    }
    return list;
}

fn part_1() {
    let input = load_file("resources/day_6_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_new_line(input);

    let mut planets: HashMap<String, Planet> = HashMap::new();

    for id in ids {
        let orbits = split_by(id, ")".parse().unwrap());
        let left_planet_name = &orbits[0];
        let right_planet_name = &orbits[1];
        let left_planet = planets
            .entry(left_planet_name.parse().unwrap())
            .or_insert(Planet {
                name: left_planet_name.parse().unwrap(),
                direct_planets_orbiting: vec![],
                orbiting_planet: None,
            });
        left_planet
            .direct_planets_orbiting
            .push(right_planet_name.parse().unwrap());
        let right_planet = planets
            .entry(right_planet_name.parse().unwrap())
            .or_insert(Planet {
                name: right_planet_name.parse().unwrap(),
                direct_planets_orbiting: vec![],
                orbiting_planet: None,
            });
        right_planet.orbiting_planet = Some(left_planet_name.parse().unwrap());
    }

    println!("Created planets");

    //Calculate orbits
    let mut total_orbits = 0;
    let keys = planets.keys().clone();
    let mut orbits_map: HashMap<String, i32> = HashMap::new();
    for planet_name in keys {
        total_orbits += get_planet_orbits(&mut orbits_map, &planets, planet_name);
    }

    stopwatch.print_elapsed();
    println!("Result: {}", total_orbits);
}

fn get_planet_orbits(
    orbits_map: &mut HashMap<String, i32>,
    planets: &HashMap<String, Planet>,
    planet_name: &String,
) -> i32 {
    let planet: &Planet = planets.get(planet_name).unwrap();
    if let Some(target_planet) = &planet.orbiting_planet {
        let mut orbits = 1;
        if (orbits_map.contains_key(target_planet)) {
            orbits += *orbits_map.get(target_planet).unwrap();
        } else {
            orbits += get_planet_orbits(orbits_map, planets, target_planet);
        }
        orbits_map.insert(planet_name.parse().unwrap(), orbits);
        return orbits;
    }
    return 0;
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
