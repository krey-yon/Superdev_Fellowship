enum TrafficLight {
    Red,
    Green,
    Yellow,
}

fn next_state(light: &TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Green => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
    }
}

fn name(light: &TrafficLight) -> &str {
    match light {
        TrafficLight::Red => "Red",
        TrafficLight::Green => "Green",
        TrafficLight::Yellow => "Yellow",
    }
}

fn simulate(steps: usize) -> Vec<String> {
    let mut states = Vec::with_capacity(steps + 1);
    let mut current = TrafficLight::Red;
    states.push(name(&current).to_string());
    for _ in 0..steps {
        current = next_state(&current);
        states.push(name(&current).to_string());
    }
    states
}

