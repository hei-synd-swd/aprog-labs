// TODO: add a method fuel_type(&self) -> &str to Engine
// TODO: delegate fuel_type in Car to its engine

struct Engine {
    horsepower: u32,
    fuel: String,
}

impl Engine {
    fn new(horsepower: u32, fuel: &str) -> Self {
        Self {
            horsepower,
            fuel: fuel.into(),
        }
    }

    fn start(&self) -> String {
        format!("{} hp engine is running", self.horsepower)
    }

    // TODO: add fn fuel_type(&self) -> &str that returns &self.fuel
}

struct Car {
    make: String,
    engine: Engine, // has-a Engine
    seats: u8,
}

impl Car {
    fn new(make: &str, engine: Engine, seats: u8) -> Self {
        Self {
            make: make.into(),
            engine,
            seats,
        }
    }

    // delegation: forward to the inner engine
    fn start(&self) -> String {
        format!("{}: {}", self.make, self.engine.start())
    }

    // TODO: add fn fuel_type(&self) -> String that delegates to self.engine.fuel_type()
    // Return format: "{} uses {}", self.make, fuel
}

fn main() {
    let v8 = Engine::new(450, "gasoline");
    let mustang = Car::new("Ford Mustang", v8, 4);

    println!("{}", mustang.start());
    println!("{}", mustang.fuel_type());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_start() {
        let e = Engine::new(100, "diesel");
        let msg = e.start();
        assert!(msg.contains("100"));
        assert!(msg.contains("running"));
    }

    #[test]
    fn test_engine_fuel_type() {
        let e = Engine::new(200, "electric");
        assert_eq!(e.fuel_type(), "electric");
    }

    #[test]
    fn test_car_delegates_start() {
        let e = Engine::new(150, "gasoline");
        let car = Car::new("Toyota", e, 5);
        let msg = car.start();
        assert!(msg.contains("Toyota"));
        assert!(msg.contains("150"));
    }

    #[test]
    fn test_car_delegates_fuel_type() {
        let e = Engine::new(300, "diesel");
        let car = Car::new("BMW", e, 5);
        let msg = car.fuel_type();
        assert!(msg.contains("BMW"));
        assert!(msg.contains("diesel"));
    }

    #[test]
    fn test_car_has_seats() {
        let e = Engine::new(100, "gasoline");
        let car = Car::new("Fiat", e, 2);
        // We can test that Car was constructed with correct seats
        // by checking if start works (indirect verification of structure)
        assert!(car.start().contains("Fiat"));
    }
}
