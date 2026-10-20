use crate::{Distribution, Model};
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

/// Statistics for a source or service station over the simulation horizon.
#[derive(Serialize, Debug, PartialEq)]
pub struct StationReport {
    pub name: String,
    pub source: bool,
    pub capacity: usize,
    pub completed: u64,
    pub remaining: usize,
    pub mean_number: f64,
    pub mean_queue: f64,
    pub utilization: f64,
    pub throughput: f64,
    pub mean_wait: Option<f64>,
    pub mean_response: Option<f64>,
}
/// Simulation totals, per-station statistics and sampled population trace.
#[derive(Serialize, Debug, PartialEq)]
pub struct Report {
    pub version: &'static str,
    pub seed: u32,
    pub horizon: f64,
    pub events: u64,
    pub initial: usize,
    pub generated: u64,
    pub exited: u64,
    pub remaining: usize,
    pub stations: Vec<StationReport>,
    pub trace: Vec<Trace>,
}
/// A sample of the network population at a simulation time.
#[derive(Serialize, Debug, PartialEq)]
pub struct Trace {
    pub time: f64,
    pub population: usize,
}
#[derive(Clone)]
struct Event {
    time: f64,
    serial: u64,
    station: usize,
    arrived: f64,
    wait: f64,
}
impl PartialEq for Event {
    fn eq(&self, b: &Self) -> bool {
        self.time == b.time && self.serial == b.serial
    }
}
impl Eq for Event {}
impl PartialOrd for Event {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}
impl Ord for Event {
    fn cmp(&self, b: &Self) -> Ordering {
        b.time
            .total_cmp(&self.time)
            .then_with(|| b.serial.cmp(&self.serial))
    }
}
#[derive(Default)]
struct State {
    queue: VecDeque<f64>,
    busy: usize,
    last: f64,
    area: f64,
    qarea: f64,
    barea: f64,
    completed: u64,
    wait: f64,
    response: f64,
}
impl State {
    fn update(&mut self, t: f64) {
        let dt = t - self.last;
        self.area += dt * (self.busy + self.queue.len()) as f64;
        self.qarea += dt * self.queue.len() as f64;
        self.barea += dt * self.busy as f64;
        self.last = t;
    }
}
struct Runtime<'a> {
    model: &'a Model,
    states: Vec<State>,
    calendar: BinaryHeap<Event>,
    serial: u64,
    rng: u32,
}
impl Runtime<'_> {
    fn delay(&mut self, index: usize) -> f64 {
        match self.model.stations[index].service {
            Distribution::Constant(d) => d,
            Distribution::Exponential(mean) => {
                self.rng ^= self.rng << 13;
                self.rng ^= self.rng >> 17;
                self.rng ^= self.rng << 5;
                let u = (self.rng as f64 + 0.5) / 4294967296.0;
                -mean * u.ln()
            }
        }
    }
    fn schedule(&mut self, index: usize, t: f64, arrived: f64, wait: f64) -> Result<(), String> {
        let next = t + self.delay(index);
        if !next.is_finite() || next <= t {
            return Err(
                "Time step is too small or overflows; rescale the model's time unit".into(),
            );
        }
        self.serial += 1;
        self.calendar.push(Event {
            time: next,
            serial: self.serial,
            station: index,
            arrived,
            wait,
        });
        Ok(())
    }
    fn arrive(&mut self, index: usize, t: f64) -> Result<(), String> {
        let s = &mut self.states[index];
        s.update(t);
        if s.busy < self.model.stations[index].capacity {
            s.busy += 1;
            self.schedule(index, t, t, 0.0)?;
        } else {
            s.queue.push_back(t);
        }
        Ok(())
    }
}
/// Simulate a model with a reproducible seed, validating it before execution.
///
/// Returns an error for invalid models or when runtime resource limits are reached.
pub fn simulate(model: &Model, seed: u32) -> Result<Report, String> {
    // The public Model API is intended for models returned by parse.
    if seed == 0 {
        return Err("Seed must be between 1 and 4294967295".into());
    }
    let n = model.stations.len();
    let mut rt = Runtime {
        model,
        states: (0..n).map(|_| State::default()).collect(),
        calendar: BinaryHeap::new(),
        serial: 0,
        rng: seed,
    };
    let initial = model.stations.iter().map(|s| s.initial).sum::<usize>();
    for (i, s) in model.stations.iter().enumerate() {
        if s.source {
            rt.schedule(i, 0.0, 0.0, 0.0)?;
        }
        for _ in 0..s.initial {
            rt.arrive(i, 0.0)?;
        }
    }
    let (mut events, mut generated, mut exited, mut population) = (0u64, 0u64, 0u64, initial);
    let mut trace = vec![Trace {
        time: 0.0,
        population,
    }];
    let mut sample = 1;
    while let Some(e) = rt.calendar.pop() {
        if e.time > model.horizon {
            break;
        }
        while sample <= 200 && model.horizon * (sample as f64) / 200.0 < e.time {
            trace.push(Trace {
                time: model.horizon * (sample as f64) / 200.0,
                population,
            });
            sample += 1;
        }
        events += 1;
        if events > 1000000 {
            return Err("Event limit (1,000,000) reached. Reduce TMAX or increase service times; no partial result is presented.".into());
        }
        let i = e.station;
        let station = &model.stations[i];
        if station.source {
            generated += 1;
            population += 1;
            rt.states[i].completed += 1;
            rt.schedule(i, e.time, 0.0, 0.0)?;
        } else {
            let s = &mut rt.states[i];
            s.update(e.time);
            s.busy -= 1;
            s.completed += 1;
            s.wait += e.wait;
            s.response += e.time - e.arrived;
            if let Some(arrived) = s.queue.pop_front() {
                s.busy += 1;
                rt.schedule(i, e.time, arrived, e.time - arrived)?;
            }
        }
        if let Some(dest) = station.destination {
            rt.arrive(dest, e.time)?;
        } else {
            exited += 1;
            population -= 1;
        }
        if population > 100000 {
            return Err(
                "Population limit (100,000) reached. Reduce TMAX or reduce the load.".into(),
            );
        }
    }
    while sample <= 200 {
        trace.push(Trace {
            time: model.horizon * (sample as f64) / 200.0,
            population,
        });
        sample += 1;
    }
    let mut stations = Vec::new();
    for (i, s) in rt.states.iter_mut().enumerate() {
        s.update(model.horizon);
        let spec = &model.stations[i];
        let count = s.completed as f64;
        stations.push(StationReport {
            name: spec.name.clone(),
            source: spec.source,
            capacity: spec.capacity,
            completed: s.completed,
            remaining: s.busy + s.queue.len(),
            mean_number: s.area / model.horizon,
            mean_queue: s.qarea / model.horizon,
            utilization: s.barea / (model.horizon * spec.capacity as f64),
            throughput: count / model.horizon,
            mean_wait: if count > 0.0 && !spec.source {
                Some(s.wait / count)
            } else {
                None
            },
            mean_response: if count > 0.0 && !spec.source {
                Some(s.response / count)
            } else {
                None
            },
        });
    }
    Ok(Report {
        version: env!("CARGO_PKG_VERSION"),
        seed,
        horizon: model.horizon,
        events,
        initial,
        generated,
        exited,
        remaining: population,
        stations,
        trace,
    })
}
