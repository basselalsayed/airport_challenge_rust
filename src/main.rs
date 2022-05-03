use airport::{airport::Airport, plane::Plane};

fn main() {
    let plane = Plane::new();
    let id = plane.id.clone();
    let plane_2 = Plane::new();
    let id_2 = plane_2.id.clone();

    let mut heathrow = Airport::new("Heathrow".to_string(), true);
    heathrow.land_plane(plane);
    heathrow.land_plane(plane_2);
    heathrow.confirm_takeoff(id);
    heathrow.sunny = false;
    heathrow.confirm_takeoff(id_2);
}
