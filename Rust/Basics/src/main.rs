fn main() {
    let mut car = Car {
        current_speed: 0.0,
        max_speed: 200.0,
        transmission: Transmission::Automatic,
        model: String::from("model T"),
        brand: String::from("Tesla"),
    };
    car.accelerate(50.0, 2.0);

    let car2 = Car {
        current_speed: 10.0,
        max_speed: 100.0,
        transmission: Transmission::Manual,
        model: String::from("Uno"),
        brand: String::from("Fiat"),
    };

    let car3 = Car {
        current_speed: 60.0,
        max_speed: 80.0,
        transmission: Transmission::Manual,
        model: String::from("Fusca"),
        brand: String::from("Volkswagen"),
    };

    let mut array = vec![car, car2, car3];

    compare_speeds(&array);
    crazy_accelerate(&mut array);
    compare_speeds(&array);

    let slower = array
        .into_iter()
        .min_by(|a, b| a.current_speed.partial_cmp(&b.current_speed).unwrap())
        .unwrap();
        
    println!("The slower one is: {}", slower.model)
}

fn crazy_accelerate(array: &mut Vec<Car>) {
    for car in array {
        car.accelerate(car.max_speed, car.current_speed);
        println!("{} acelerado para {}", car.model, car.current_speed)
    }
}

fn compare_speeds(array: &Vec<Car>) {
    let mut iter = array.iter();
    while let Some(car) = iter.next() {
        for other_car in iter.as_slice() {
            println!("Carro 1: {} Velocidade: {}", &car.model, &car.current_speed);
            println!(
                "Carro 2: {} Velocidade: {}",
                &other_car.model, &other_car.current_speed
            );

            let faster = if car.current_speed > other_car.current_speed {
                &car
            } else {
                other_car
            };
            println!("Vencedor: {}", faster.model);
        }
    }
}
#[derive(Clone, Copy)]
enum Transmission {
    Automatic,
    Manual,
}
struct Car {
    max_speed: f64,
    current_speed: f64,
    transmission: Transmission,
    model: String,
    brand: String,
}
trait VehicleThings {
    fn set_current_speed(&mut self, new_speed: f64);
    fn get_current_speed(&self) -> f64;
    fn get_transmission(&self) -> Transmission;
    fn get_model(&self) -> &String;
    fn get_brand(&self) -> &String;
}
impl VehicleThings for Car {
    fn set_current_speed(&mut self, new_speed: f64) {
        self.current_speed = new_speed
    }

    fn get_current_speed(&self) -> f64 {
        self.current_speed
    }

    fn get_model(&self) -> &String {
        &self.model
    }

    fn get_brand(&self) -> &String {
        &self.brand
    }

    fn get_transmission(&self) -> Transmission {
        self.transmission
    }
}
trait SpeedUp: VehicleThings {
    fn accelerate(&mut self, speed_kph_ps: f64, seconds: f64) {
        let new_speed = speed_kph_ps * seconds;
        self.set_current_speed(self.get_current_speed() + new_speed)
    }
}
impl SpeedUp for Car {}
