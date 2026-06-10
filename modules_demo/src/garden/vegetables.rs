pub struct Plant {
    pub name: String,
}

pub fn tomato() -> Plant {
    Plant {
        name: String::from("tomato"),
    }
}

fn harvest_note() -> &'static str {
    "ready to pick"
}

pub fn label(plant: &Plant) -> String {
    format!("{} ({})", plant.name, harvest_note())
}
