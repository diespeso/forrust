pub mod season;
pub mod moving_median;
pub mod merger;
pub mod grouper;

pub use season::Season;
pub use moving_median::MovingMedian;
pub use merger::Merger;
pub use grouper::Grouper;

use crate::plotable::Plotable;

use std::fmt;

use plotlib::view::{View, CategoricalView, ContinuousView};
use plotlib::repr::{Plot, CategoricalRepresentation};
use plotlib::style::{PointStyle, LineStyle, PointMarker};

/// Holds the data of the series
/// its implied that every unit of data represents a unit of time
#[derive(Clone)]
pub struct TimeSeries {
    data: Vec<(f64, f64)>,
    dom_ran: Option<(String, String)>,
    style: Style,
}

impl TimeSeries {

    /// Creates a new Timeseries with the data given for the y axis.
    pub fn new(data: Vec<f64>) -> Self {
        let mut v = Vec::new();
        for i in 0..data.len() {
            v.push(((i + 1) as f64, data[i]));
        }
        Self {
            data: v,
            dom_ran: None,
            style: Default::default(),
        }
    }

    pub fn from_pairs_vec(pairs: Vec<(f64, f64)>) -> Self {
        /*let mut v = pairs.iter().map(|pair| pair.1).collect();
        let mut d = pairs.iter().map(|pair| pair.0).collect();
        let mut series = TimeSeries::new(v);*/
        Self {
            data: pairs,
            dom_ran: None,
            style: Default::default(),
        }
    }

    /// Returns all (x,y) values
    pub fn get_data(&self) ->Vec<(f64, f64)> {
        /*let mut vec = Vec::new();
        for i in 0..self.data.len() {
            vec.push(
                ((i + 1 as usize) as f64, self.data[i] as f64)
            );
        }*/

        self.data.clone()
    }

    /// returns the y value of this time series
    /// at the given x value
    fn get_range_at(&self, u: usize) -> f64 {
        for (x, y) in self.data.iter() {
            if *x as usize == u {
                return *y;
            }
        }
        panic!(format!("domain value {} doesn't exist in the time series", u));
    }

    /// returns the y value at the nth position
    fn get_range_at_ord(&self, n: usize) -> f64 {
        self.get_data()[n].1
    }

    /// adds the given data at the end of this timeseries
    pub fn push(&mut self, data: f64) {
        let dom_last = self.data[self.data.len() - 1].0; //gets last domain number
        self.data.push(((dom_last + 1 as f64) , data)); //last domain + 1
    }

    /// Returns all y values
    pub fn get_range(&self) -> Vec<f64> {
        self.data.iter().map(|(x, y)| *y).collect()
    }

    pub fn style(&self) -> Style {
        self.style.clone()
    }

    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Clone)]
pub struct Style{
    pub point: PointStyle,
    pub line: LineStyle,
}

impl Style {
    pub fn from_color(color: &str) -> Self {
        let mut style: Style = Default::default();
        style.point = style.point.colour(color);
        style.line = style.line.colour(color);
        style
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            point: PointStyle::new().colour("#000000").marker(PointMarker::Circle),
            line: LineStyle::new().colour("#000000")
        }
    }
}


impl Plotable for TimeSeries {
    fn plot(&self) -> Box<dyn View> {
        let plot = self.as_plot();
        let mut view = ContinuousView::new()
        .add(plot);
        Box::new(view)
    }

    fn as_plot(&self) -> Plot {
        let mut plot = Plot::new(self.get_data());
        plot.point_style(self.style().point).line_style(self.style().line)
    }
}


/// Un Mes del año.
#[derive(Copy, Clone, Debug)]
pub enum Mes {
    Enero,
    Febrero,
    Marzo,
    Abril,
    Mayo,
    Junio,
    Julio,
    Agosto,
    Septiembre,
    Octubre,
    Noviembre,
    Diciembre

}

impl fmt::Display for Mes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl From<u32> for Mes {
    fn from(v: u32) -> Self {
        
        match v {
            0 => Self::Enero,
            1 => Self::Febrero,
            2 => Self::Marzo,
            3 => Self::Abril,
            4 => Self::Mayo,
            5 => Self::Junio,
            6 => Self::Julio,
            7 => Self::Agosto,
            8 => Self::Septiembre,
            9 => Self::Octubre,
            10 => Self::Noviembre,
            11 => Self::Diciembre,
            _ => panic!(format!("Número de mes inválido: {}", v))
        }
    }
}

impl From<&str> for Mes {
    fn from(v: &str) -> Self {
        let v = v.to_lowercase();
        match v.as_str() {
           "enero" => Self::Enero,
           "febrero" => Self::Febrero,
           "marzo" => Self::Marzo,
           "abril" => Self::Abril,
           "mayo" => Self::Mayo,
           "junio" => Self::Junio,
           "julio" => Self::Julio,
           "agosto" => Self::Agosto,
           "septiembre" => Self::Septiembre,
           "octubre" => Self::Octubre,
           "noviembre" => Self::Noviembre,
           "diciembre" => Self::Diciembre,
           _ => panic!(format!("Mes inválido: {}", v))
        }
    }
}

