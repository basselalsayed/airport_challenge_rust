use crate::plane::Plane;

pub struct Airport {
    pub hangar: Vec<Plane>,
    pub location: String,
    pub sunny: bool,
}

impl Airport {
    pub fn new(location: String, sunny: bool) -> Self {
        Self {
            location,
            hangar: Vec::new(),
            sunny,
        }
    }

    fn plane_in_hangar(&self, id: String) -> Option<usize> {
        self.hangar.iter().position(|plane| plane.id == id)
    }

    pub fn land_plane(&mut self, mut plane: Plane) {
        plane.land(self.location.clone());
        self.hangar.push(plane);
        self.check_hangar()
    }

    pub fn check_hangar(&self) {
        for plane in &self.hangar {
            println!("{:#?}", plane);
            println!("-----------------");
        }
        println!("hangar has {} plane", self.hangar.len());
    }

    pub fn confirm_takeoff(&mut self, id: String) {
        match self.plane_in_hangar(id.clone()) {
            Some(x) if self.sunny => {
                self.remove(x);
                let plane = &mut self.hangar[x];
                plane.take_off(self.location.clone());
                println!("plane departed to {}", plane.departed_from);
                self.check_hangar()
            }
            Some(x) => {
                println!(
                    "Plane {} could not depart to due to bad weather",
                    self.hangar[x].id
                );
            }
            _ => println!("Plane {} not found", id),
        }
    }

    pub fn remove(&mut self, index: usize) -> Plane {
        self.hangar.remove(index)
    }
}
