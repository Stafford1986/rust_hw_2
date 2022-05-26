enum Breaker {
    On,
    Off,
}

struct Rosette {
    breaker: Breaker,
    description: String,
    current_power: u32,
}

struct Thermometer {
    temperature: i16,
}

impl Rosette {
    fn new(desctiption: String) -> Self {
        Self {
            breaker: Breaker::On,
            description: desctiption,
            current_power: 0,
        }
    }

    fn switch_breaker(&mut self) {
        let new_state = match self.breaker {
            Breaker::On => Breaker::Off,
            Breaker::Off => Breaker::On,
        };

        self.breaker = new_state;
    }

    fn get_current_power(&self) -> u32 {
        self.current_power
    }

    fn description(&self) -> String {
        format!("This is rosette description: {}", self.description)
    }
}

impl Thermometer {
    fn new() -> Self {
        Self { temperature: 0 }
    }

    fn set_temperature(&mut self, temp: i16) {
        self.temperature = temp
    }

    fn get_temperature(&self) -> i16 {
        self.temperature
    }
}

fn main() {
    let mut rosette = Rosette::new("First rosette".into());
    rosette.switch_breaker();

    println!("Rosette power is: {}", rosette.get_current_power());
    println!("{}", rosette.description());

    let mut thermometer = Thermometer::new();
    thermometer.set_temperature(10);

    println!("Current temperature is: {}", thermometer.get_temperature());
}
