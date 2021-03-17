use crate::time_series::{TimeSeries, Style};
use crate::plotable::Plotable;


use plotlib::view::{View, ContinuousView};
use plotlib::repr::Plot;

#[derive(Clone)]
pub struct LinearRegression {
    series: TimeSeries,
    data: Vec<(f64, f64)>,
    cons: f64,
    slope: f64,
}

impl LinearRegression {
    pub fn new(series: &TimeSeries) -> Self {
        let mut new = Self{
            series: series.clone(),
            data: Vec::new(),
            cons: 0.0,
            slope: 0.0,
        };
        new.update_data();
        new
    }

    /// Generates the linear regression data for the time series
    /// given in the construction
    fn update_data(&mut self) {
        self.data = Vec::new(); //restart
        self.slope = self.slope();
        self.cons = self.cons();

        let pairs = self.series.get_data();
        for i in 0..pairs.len() {
            self.data.push(
                (pairs[i].0, self.calculate(pairs[i].0) ) //mx+b
            )
        }
    }

    pub fn slope(&self) -> f64 {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;

        let data = self.series.get_data();
        for pair in data.iter() {
            sum_x += pair.0;
            sum_y += pair.1;
            sum_xy += pair.0 * pair.1;
            sum_x2 += pair.0.powi(2);
        }
        let count = data.len() as f64;

        ( (count * sum_xy) - (sum_x) * (sum_y) ) / (count * sum_x2 - sum_x.powi(2))
    }

    pub fn cons(&self) -> f64 {
        let data = self.series.get_data();

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        for pair in data.iter() {
            sum_x += pair.0;
            sum_y += pair.1;
        }
        (sum_y - self.slope() * sum_x) / (data.len() as f64)
    }

    /// Calculates a regression value using this linear regression's found function
    pub fn calculate(&self, x: f64) -> f64 {
        (self.slope() * x) + self.cons()
    }

    pub fn as_time_series(&self) -> TimeSeries {
        TimeSeries::from_pairs_vec(self.get_data())
    }

    pub fn get_data(&self) -> Vec<(f64, f64)> {
        self.data.clone()
    }

    pub fn style(&self) -> Style {
        self.series.style()
    }
}

impl Plotable for LinearRegression {
    fn plot(&self) -> Box<dyn View> {
        let plot = self.as_plot();
        Box::new(ContinuousView::new().add(plot))
    }

    fn as_plot(&self) -> Plot {
        let mut plot = Plot::new(self.get_data());

        plot
        .point_style(self.style().point)
        .line_style(self.style().line)
    }
}