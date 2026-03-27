use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use crate::string_utils::{hash_code, starts_with};

#[derive(Debug, Clone)]
pub struct ResultField {
    pub percentage: f32,
    pub global_percentage: f32,
    pub name: String,
}

impl ResultField {
    pub fn get_color(&self) -> i32 {
        (hash_code(&self.name) & 0x00aa_aaaa) + 0x0044_4444
    }
}

#[derive(Default)]
struct PerfTimerState {
    enabled: bool,
    paths: Vec<String>,
    start_times: Vec<f32>,
    path: String,
    times: HashMap<String, f32>,
}

static START_TIME: OnceLock<Instant> = OnceLock::new();
static STATE: OnceLock<Mutex<PerfTimerState>> = OnceLock::new();

fn now_s() -> f32 {
    let base = START_TIME.get_or_init(Instant::now);
    base.elapsed().as_secs_f32()
}

fn state() -> &'static Mutex<PerfTimerState> {
    STATE.get_or_init(|| Mutex::new(PerfTimerState::default()))
}

pub fn set_enabled(enabled: bool) {
    let mut s = state().lock().expect("perf timer lock poisoned");
    s.enabled = enabled;
}

pub fn enabled() -> bool {
    state().lock().expect("perf timer lock poisoned").enabled
}

pub fn reset() {
    let mut s = state().lock().expect("perf timer lock poisoned");
    s.times.clear();
}

pub fn push(name: &str) {
    let mut s = state().lock().expect("perf timer lock poisoned");
    if !s.enabled {
        return;
    }
    if !s.path.is_empty() {
        s.path.push('.');
    }
    s.path.push_str(name);
    let path_snapshot = s.path.clone();
    s.paths.push(path_snapshot);
    s.start_times.push(now_s());
}

pub fn pop() {
    let mut s = state().lock().expect("perf timer lock poisoned");
    if !s.enabled || s.start_times.is_empty() {
        return;
    }
    let end_time = now_s();
    let start_time = s.start_times.pop().unwrap_or(end_time);
    let current_path = s.path.clone();

    s.paths.pop();
    let time = end_time - start_time;
    *s.times.entry(current_path).or_insert(0.0) += time;

    s.path = s.paths.last().cloned().unwrap_or_default();
}

pub fn pop_push(name: &str) {
    pop();
    push(name);
}

pub fn get_log(raw_path: &str) -> Vec<ResultField> {
    let mut s = state().lock().expect("perf timer lock poisoned");
    if !s.enabled {
        return Vec::new();
    }

    let mut path = raw_path.to_string();
    let global_time = *s.times.get("root").unwrap_or(&0.0);
    let total_time2 = *s.times.get(&path).unwrap_or(&-1.0);
    if !path.is_empty() {
        path.push('.');
    }

    let mut total_time = 0.0f32;
    for (key, time) in &s.times {
        if key.len() > path.len()
            && starts_with(key, &path)
            && !key[(path.len() + 1).min(key.len())..].contains('.')
        {
            total_time += *time;
        }
    }

    let old_time = total_time;
    if total_time < total_time2 {
        total_time = total_time2;
    }
    let mut global_time_adj = global_time;
    if global_time_adj < total_time {
        global_time_adj = total_time;
    }
    if global_time_adj <= 0.0 {
        global_time_adj = 1.0;
    }

    let mut result = Vec::new();
    for (key, time) in &s.times {
        if key.len() > path.len()
            && starts_with(key, &path)
            && !key[(path.len() + 1).min(key.len())..].contains('.')
        {
            let time_percentage = *time * 100.0 / total_time.max(1e-9);
            let global_percentage = *time * 100.0 / global_time_adj;
            let name = key[path.len()..].to_string();
            result.push(ResultField {
                name,
                percentage: time_percentage,
                global_percentage,
            });
        }
    }

    for value in s.times.values_mut() {
        *value *= 0.999;
    }

    if total_time > old_time {
        result.push(ResultField {
            name: "unspecified".to_string(),
            percentage: (total_time - old_time) * 100.0 / total_time.max(1e-9),
            global_percentage: (total_time - old_time) * 100.0 / global_time_adj,
        });
    }

    result.sort_by(|a, b| {
        b.percentage
            .partial_cmp(&a.percentage)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.name.cmp(&a.name))
    });
    result.insert(
        0,
        ResultField {
            name: raw_path.to_string(),
            percentage: 100.0,
            global_percentage: total_time * 100.0 / global_time_adj,
        },
    );
    result
}

