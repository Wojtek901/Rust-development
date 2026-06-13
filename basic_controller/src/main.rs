struct Radians(f64);
struct Degrees(f64);
struct MotorThrottle(f64);

struct Hover;
struct Transition;
struct Level;

struct UAVState<State>{
    state: State,
    current_pitch_angle: Radians,
    desired_pitch_angle: Radians,
}

impl Radians {
    fn to_degrees(&self) -> Degrees{
        Degrees(self.0 * (180.0 / std::f64::consts::PI))
    }
}

impl Degrees {
    fn to_radians(&self) -> Radians{
        Radians(self.0 * (std::f64::consts::PI / 180.0))
    }
}

impl UAVState<Hover>{
    fn new(pitch: Radians) -> Self{
        UAVState {
            state: Hover,
            current_pitch_angle: pitch,
            desired_pitch_angle: Radians(std::f64::consts::PI / 2.0),
        }
    }

    fn hover_stabilize(&self) -> MotorThrottle{
        let proportional_regulation: f64 = 2.5 * (self.desired_pitch_angle.0 - self.current_pitch_angle.0);
        MotorThrottle(proportional_regulation)
    }

    fn start_transition(self) -> UAVState<Transition>{
        UAVState { state: Transition,
            current_pitch_angle: self.current_pitch_angle,
            desired_pitch_angle: Radians(0.0),
        }
    }
}

impl UAVState<Transition>{
    fn tune_transition_pid(&mut self, sweep_iteration: i32){
        println!("Oscilation tuning, iterations parameter: {}", sweep_iteration);
        self.current_pitch_angle = Radians(0.9 * self.current_pitch_angle.0 + 0.1 * self.desired_pitch_angle.0);
    }

    fn finish_transition(self) -> UAVState<Level>{
        UAVState { state: Level,
            current_pitch_angle: self.current_pitch_angle,
            desired_pitch_angle: Radians(0.0),
        }
    }
}

impl UAVState<Level>{
    fn cruise(&self){
        println!("Stable cruise flight.");
    }
}

fn main() {
    let controller_h = UAVState::new(Radians(1.5));

    let throttle = controller_h.hover_stabilize().0;

    let mut controller_t = controller_h.start_transition();

    for i in 10..13{
        controller_t.tune_transition_pid(i);
    }

    let controller_l = controller_t.finish_transition();
}
