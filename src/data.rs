use std::fmt;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, PointStyle, LineJoin};

use crate::plotable::Plotable;

/// Una abstracción de un vector de pares (f64, f64)
/// Los datos tienen un parametro de temporada, usualmente 12.
pub trait Data {
    /// Returns the vector of (x, y) pairs of the given season
    /// season start from 1. There can be non-full seasons.
    fn season(&self, n: usize) -> Vec<(f64, f64)> {
        let mut v = Vec::new();
        let low = self.season_length() * (n - 1);
        let mut high = low + self.season_length();
        if high > self.as_vec().len() { //out of bounds
            high = self.as_vec().len()
        }
        for i in low..high {
            v.push(self.as_vec()[i]);
        }
        v
    }
    /// Returns the size of a season for this data.
    fn season_length(&self) -> usize;
    /// Returns the 'primitive' data pairs vector
    fn as_vec(&self) -> Vec<(f64, f64)>;
    /// Returns the number of seasons in this data.
    fn seasons(&self) -> usize {
        (self.as_vec().len() as f64 / self.season_length() as f64).ceil() as usize
    }
    /// Returns the (x, y) pair in the given unit of the data.
    /// Like, if every pair represents a month, it returns a month
    fn get(&self, i: usize) -> (f64, f64) {
        self.as_vec()[i]
    }
}

/// Holds the data of market demand month by month
pub struct TablaDemanda {
    elements: Vec<f64>,
}

impl TablaDemanda {
    /// Creates a new TablaDemanda given a month demand vector.
    pub fn new(elements: Vec<f64>) -> Self {
        Self {
            elements: elements,
        }
    }
}

impl Data for TablaDemanda {
    fn season_length(&self) -> usize {
        12
    }

    fn as_vec(&self) -> Vec<(f64, f64)> {
        let mut v = Vec::new();
        for i in 0..self.elements.len() {
            v.push((i as f64, self.elements[i] as f64));
        }
        v
    }
}

impl Plotable for TablaDemanda {
    fn plot(&self) -> Plot {
        Plot::new(self.as_vec())
        .point_style(PointStyle::new()
            .colour("#000000")
        ).line_style(LineStyle::new().linejoin(LineJoin::Round))
    }
}

impl fmt::Display for TablaDemanda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in 1..self.seasons() + 1 {
            s += format!("season: {}\n", i).as_str();
            for e in self.season(i).iter() {
                s += format!("{} -> {}\n", e.0, e.1).as_str();
            }
        }
        write!(f, "{}", s)
    }
}