// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_core::parking_lot::Mutex;
use deno_core::serde::Deserialize;
use deno_core::serde::Serialize;
use deno_core::serde_json::json;
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::time::Duration;
use std::time::Instant;

use super::logging::lsp_debug;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceAverage {
  pub name: String,
  pub count: u32,
  pub average_duration: u32,
}

impl PartialOrd for PerformanceAverage {
  fn partial_cmp(
    &self,
    other: &Self,
  ) -> Option<cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for PerformanceAverage {
  fn cmp(
    &self,
    other: &Self,
  ) -> cmp::Ordering {
    self.name.cmp(&other.name)
  }
}

/// A structure which serves as a start of a measurement span.
#[derive(Debug)]
pub struct PerformanceMark {
  name: String,
  count: u32,
  start: Instant,
}

/// A structure which holds the information about the measured span.
#[derive(Debug, Clone)]
pub struct PerformanceMeasure {
  pub name: String,
  pub count: u32,
  pub duration: Duration,
}

impl fmt::Display for PerformanceMeasure {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    write!(
      f,
      "{} ({}ms)",
      self.name,
      self.duration.as_micros() as f64 / 1000.0
    )
  }
}

impl From<PerformanceMark> for PerformanceMeasure {
  fn from(value: PerformanceMark) -> Self {
    Self {
      name: value.name,
      count: value.count,
      duration: value.start.elapsed(),
    }
  }
}

/// A simple structure for marking a start of something to measure the duration
/// of and measuring that duration.  Each measurement is identified by a string
/// name and a counter is incremented each time a new measurement is marked.
///
/// The structure will limit the size of measurements to the most recent 1000,
/// and will roll off when that limit is reached.
#[derive(Debug)]
pub struct Performance {
  counts: Mutex<HashMap<String, u32>>,
  measurements_by_type: Mutex<HashMap<String, (/* count */ u32, /* duration */ f64)>>,
  max_size: usize,
  measures: Mutex<VecDeque<PerformanceMeasure>>,
}

impl Default for Performance {
  fn default() -> Self {
    Self {
      counts: Default::default(),
      measurements_by_type: Default::default(),
      max_size: 3_000,
      measures: Default::default(),
    }
  }
}

impl Performance {
  /// Return an iterator which provides the names, count, and average duration
  /// of each measurement.
  pub fn averages(&self) -> Vec<PerformanceAverage> {
    let mut averages: HashMap<String, Vec<Duration>> = HashMap::new();
    for measure in self.measures.lock().iter() {
      averages
        .entry(measure.name.clone())
        .or_default()
        .push(measure.duration);
    }
    averages
      .into_iter()
      .map(|(k, d)| {
        let count = d.len() as u32;
        let a = d.into_iter().sum::<Duration>() / count;
        PerformanceAverage {
          name: k,
          count,
          average_duration: a.as_millis() as u32,
        }
      })
      .collect()
  }

  pub fn measurements_by_type(&self) -> Vec<(String, u32, f64)> {
    let measurements_by_type = self.measurements_by_type.lock();
    measurements_by_type
      .iter()
      .map(|(name, (count, duration))| (name.to_string(), *count, *duration))
      .collect::<Vec<_>>()
  }

  pub fn averages_as_f64(&self) -> Vec<(String, u32, f64)> {
    let mut averages: HashMap<String, Vec<Duration>> = HashMap::new();
    for measure in self.measures.lock().iter() {
      averages
        .entry(measure.name.clone())
        .or_default()
        .push(measure.duration);
    }
    averages
      .into_iter()
      .map(|(k, d)| {
        let count = d.len() as u32;
        let a = d.into_iter().sum::<Duration>() / count;
        (k, count, a.as_micros() as f64 / 1000.0)
      })
      .collect()
  }

  fn mark_inner<S: AsRef<str>, V: Serialize>(
    &self,
    name: S,
    maybe_args: Option<V>,
  ) -> PerformanceMark {
    let name = name.as_ref();
    let mut counts = self.counts.lock();
    let count = counts.entry(name.to_string()).or_insert(0);
    *count += 1;
    {
      let mut measurements_by_type = self.measurements_by_type.lock();
      let measurement = measurements_by_type
        .entry(name.to_string())
        .or_insert((0, 0.0));
      measurement.0 += 1;
    }
    let msg = if let Some(args) = maybe_args {
      json!({
        "type": "mark",
        "name": name,
        "count": count,
        "args": args,
      })
    } else {
      json!({
        "type": "mark",
        "name": name,
      })
    };
    lsp_debug!("{},", msg);
    PerformanceMark {
      name: name.to_string(),
      count: *count,
      start: Instant::now(),
    }
  }

  /// Marks the start of a measurement which returns a performance mark
  /// structure, which is then passed to `.measure()` to finalize the duration
  /// and add it to the internal buffer.
  pub fn mark<S: AsRef<str>>(
    &self,
    name: S,
  ) -> PerformanceMark {
    self.mark_inner(name, None::<()>)
  }

  /// Marks the start of a measurement which returns a performance mark
  /// structure, which is then passed to `.measure()` to finalize the duration
  /// and add it to the internal buffer.
  pub fn mark_with_args<S: AsRef<str>, V: Serialize>(
    &self,
    name: S,
    args: V,
  ) -> PerformanceMark {
    self.mark_inner(name, Some(args))
  }

  /// A function which accepts a previously created performance mark which will
  /// be used to finalize the duration of the span being measured, and add the
  /// measurement to the internal buffer.
  pub fn measure(
    &self,
    mark: PerformanceMark,
  ) -> Duration {
    let measure = PerformanceMeasure::from(mark);
    lsp_debug!(
      "{},",
      json!({
        "type": "measure",
        "name": measure.name,
        "count": measure.count,
        "duration": measure.duration.as_micros() as f64 / 1000.0,
      })
    );
    let duration = measure.duration;
    {
      let mut measurements_by_type = self.measurements_by_type.lock();
      let measurement = measurements_by_type
        .entry(measure.name.to_string())
        .or_insert((0, 0.0));
      measurement.1 += duration.as_micros() as f64 / 1000.0;
    }
    let mut measures = self.measures.lock();
    measures.push_front(measure);
    while measures.len() > self.max_size {
      measures.pop_back();
    }
    duration
  }

  pub fn to_vec(&self) -> Vec<PerformanceMeasure> {
    let measures = self.measures.lock();
    measures.iter().cloned().collect()
  }
}
