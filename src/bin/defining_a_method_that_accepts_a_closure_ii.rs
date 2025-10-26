#[derive(Debug)]
struct Location {
    name: String,
    treasure: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        let final_index = self.locations.len() - 1;
        let mut current_index = 0;
        while current_index <= final_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}



fn main() {
    let locations = [
        Location {
            name: String::from("Enchanted Forest"),
            treasure: 5,
        },
        Location {
            name: String::from("Mystic Mountain"),
            treasure: 10,
        },
    ];

    let map = Map {
        locations: &locations,
    };

    let mut total_treasure = 0;

    map.explore(|location| {
        total_treasure += location.treasure;
    });

    println!("Total treasure collected: {total_treasure}");
    let mut location_names: Vec<String> = Vec::new();
    
    map.explore(|location| {
        location_names.push(location.name.clone());
    });

    println!("{location_names:?}");
}