#[derive(Debug)]
pub enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

#[derive(Debug)]
pub struct Vehicle {
    make: String,
    model: String,
    year: u16,
    price: f64,
    vehicle_type: VehicleType,
    horsepower: Option<u16>,
    fuel_type: Option<String>,
    weight: f64,
}