use serde::{Serialize, Deserialize};
use chrono::{Datelike, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    make: String,
    model: String,
    year: u16,
    price: f64,
    vehicle_type: VehicleType,
    horsepower: Option<u16>,
    torque: Option<u16>,
    fuel_type: Option<String>,
    weight: f64,
    validated: bool,
}

impl Vehicle {
    pub fn validate(&mut self) -> Result<(), String> {
        self.validated = false;

        // Make validation
        if self.make.trim().is_empty() {
            return Err("Make must be a non-empty string.".to_string());
        } else if !self.make.chars().all(
            |c| c.is_alphanumeric()
            || c == ' '
            || c == '-'
            || c == '–'
            || c == '_') {
            return Err(format!(
                "Make contains invalid characters. Given: {}.",
                self.make
            ))
        }

        // Model validation
        if self.model.trim().is_empty() {
            return Err("Model must be a non-empty string.".to_string());
        } else if !self.model.chars().all(
            |c| c.is_alphanumeric()
                || c == ' '
                || c == '-'
                || c == '–'
                || c == '_') {
            return Err(format!(
                "Model contains invalid characters. Given: {}.",
                self.model
            ))
        }

        // Year validation
        let max_year = Utc::now().year() as u16 + 1;
        if self.year < 1886 || self.year > max_year {
            return Err(format!(
                "Year must be between 1886 and {}. Given: {}.",
                max_year, self.year
            ))
        }

        // Price validation
        if self.price < 0.0 {
            return Err(format!(
                "Price cannot be negative. Given: {}.",
                self.price
            ))
        }

        // Horsepower validation
        if let Some(hp) = self.horsepower {
            if hp <= 0 {
                return Err(format!(
                    "Horsepower must be positive. Given: {}.",
                    hp
                ))
            }
        }

        // Torque validation
        if let Some(tq) = self.torque {
            if tq <= 0 {
                return Err(format!(
                    "Torque must be positive. Given: {}.",
                    tq
                ))
            }
        }

        // Fuel type validation
        if let Some(fuel_type) = &self.fuel_type {
            if fuel_type.trim().is_empty() {
                return Err("Fuel type must be a non-empty string.".to_string());
            } else if !fuel_type.chars().all(
                |c| c.is_alphanumeric()
                    || c == ' '
                    || c == '-'
                    || c == '–'
                    || c == '_') {
                return Err(format!(
                    "Fuel type contains invalid characters. Given: {}.",
                    fuel_type
                ))
            }
        }

        // Weight validation
        if self.weight <= 0.0 {
            return Err(format!(
                "Weight must be positive. Given: {}.",
                self.weight
            ));
        }

        self.validated = true;
        Ok(())
    }
}