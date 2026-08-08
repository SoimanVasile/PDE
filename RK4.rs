const PLANCK_CONSTANT: f64 = 1.0; //atomic unit of action
const ELECTRON_MASS: f64 = 1.0; //atomic unit of mass
const NUMBER_OF_ITERATIONS: u64= 10000; // how many iterations the runge kutta to do

#[derive(Debug)]
struct State{
    value: f64,
    slope: f64,
}

impl State{
    pub fn new(value: f64, slope: f64) -> Self{
        Self{ value, slope}
    }

    pub fn get_value(&self) -> f64{
        self.value
    }

    pub fn get_slope(&self) -> f64{
        self.slope
    }

    pub fn set_value(&mut self, value: f64){
        self.value = value;
    }

    pub fn set_slope(&mut self, slope: f64){
        self.slope = slope;
    }
} fn calculate_schrodinger_equation<F: Fn(f64) -> f64>(state: State, potential: &F, x: f64, energy: f64) -> State
{
    let phi = state.get_value();
    let der_phi = state.get_slope();

    let sec_der_phi = (( 2.0 * ELECTRON_MASS ) / ( -1.0 * PLANCK_CONSTANT )) * phi * (energy - potential(x));

    State::new(der_phi, sec_der_phi)
}

fn runge_kutta4<F: Fn(f64) -> f64>(state: &mut State, potential: &F, x:f64, energy: f64, step: f64){
    let phi = state.get_value();
    let der_phi = state.get_slope();

    let k1: State = calculate_schrodinger_equation(State::new(phi, der_phi), potential, x, energy);

    let k2: State = calculate_schrodinger_equation(State::new(phi + k1.get_value() * step / 2.0, der_phi + k1.get_slope() * step / 2.0), potential, x + step / 2.0, energy);

    let k3: State = calculate_schrodinger_equation(State::new(phi + k2.get_value() * step / 2.0, der_phi + k2.get_slope() * step / 2.0), potential, x + step / 2.0, energy);

    let k4: State = calculate_schrodinger_equation(State::new(phi + k3.get_value() * step, der_phi + k3.get_slope() * step), potential, x + step, energy);


    let next_iteration_phi = phi + (step / 6.0) * (k1.get_value() + 2.0 * k2.get_value() + 2.0 * k3.get_value() + k4.get_value());

    let next_iteration_der_phi = der_phi + (step / 6.0) * (k1.get_slope() + 2.0 * k2.get_slope() + 2.0 * k3.get_slope() + k4.get_slope());

    state.set_value(next_iteration_phi);
    state.set_slope(next_iteration_der_phi);
}

fn shoot<F: Fn(f64) -> f64>(x_start: f64, x_end: f64, energy: f64, potential: &F) -> f64{
    let mut state = State::new(0.0, -0.0005);
    let step = (x_end - x_start) / NUMBER_OF_ITERATIONS as f64;
    let mut x = x_start;

    for _ in 0..NUMBER_OF_ITERATIONS{
        runge_kutta4(&mut state, potential, x, energy, step);
        x+=step;
    }

    state.get_value()
}

fn find_energy<F: Fn(f64) -> f64>(x_start: f64, x_end: f64, min_e: f64, max_e: f64, potential: &F) -> Option<f64>{
    let tolerance = 1e-7;

    let mut phi_min = shoot(x_start, x_end, min_e, potential);
    let phi_max = shoot(x_start, x_end, max_e, potential);

    if phi_min * phi_max > 0.0{
        return None;
    }

    if phi_min.abs() < tolerance{
        return Some(min_e);
    }

    if phi_max.abs() < tolerance{
        return Some(max_e);
    }

    let mut low_e = min_e;
    let mut high_e = max_e;
    while (high_e - low_e).abs() > tolerance{
        
        let mid_e = low_e + (high_e - low_e) / 2.0;

        let phi_mid = shoot(x_start, x_end, mid_e, potential);
        if phi_mid.abs() < tolerance{
            return Some(mid_e);
        }

        if phi_mid * phi_min < 0.0{
            high_e = mid_e;
        }else{
            low_e = mid_e;
            phi_min = phi_mid;
        }
    }

    Some(low_e)
}

fn main() {

    let x_start = -5.0;
    let x_end = 5.0;

    let harmonic_oscilator = |x: f64| -> f64{ 0.5 * x * x};

    let e_min_guess = 0.0;
    let e_max_guess = 1.0;

    match find_energy(x_start, x_end, e_min_guess, e_max_guess, &harmonic_oscilator){
        Some(energy) =>{
            println!("Success! Found valid energy level:");
            println!("Calculated E = {:.6}", energy);
            println!("Theoretical E = 0.500000");
        },
        None => {
            println!("Failed to find an energy level. Try changing your initial guesses.");
            println!("(e_min and e_max must produce final waves with opposite signs)");
        }
    }
}
