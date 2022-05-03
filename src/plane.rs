use nanoid::nanoid;

#[derive(Debug)]
pub struct Plane {
    pub id: String,
    pub flying: bool,
    pub departed_from: String,
    pub landed_at: String,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            id: nanoid!(),
            flying: false,
            departed_from: "".to_string(),
            landed_at: "".to_string(),
        }
    }

    pub fn take_off(&mut self, location: String) {
        self.departed_from = location;
        self.flying = true;
    }

    pub fn land(&mut self, location: String) {
        self.landed_at = location;
        self.flying = false;
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Plane {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            flying: self.flying,
            departed_from: self.departed_from.clone(),
            landed_at: self.landed_at.clone(),
        }
    }
}
