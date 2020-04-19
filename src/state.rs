use std::f64;

// State transition behavior
#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub enum State<S,T> {
    ActionState(S, Option<T>),
    WaitState(f64, f64),
    AfterState(usize, Vec<State<S,T>>),
}