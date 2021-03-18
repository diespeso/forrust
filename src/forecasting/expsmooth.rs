use crate::time_series::{TimeSeries, Style};
use crate::plotable::Plotable;

use plotlib::view::{View, ContinuousView};
use plotlib::repr::Plot;

/// Takes a TimeSeries and obtains its simple exponential smoothing
#[derive(Clone)]
pub struct ExpSmoothing {
    series: TimeSeries,
    alpha: Option<f64>,
    cur: usize, //last smoothed value
    smooth: Vec<(f64, f64)>
}

impl ExpSmoothing {
    /// Returns a new ExpSmothing with no alpha
    pub fn new(series: &TimeSeries) -> Self {
        let mut result = Self {
            series: series.clone(),
            alpha: None,
            cur: 0,
            smooth: Vec::new(),
        };
        result
    }

    /// Sets the alpha and automatically generates the smoothing for all x existing values
    pub fn with_alpha(mut self, alpha: f64) -> Self {
        //restart
        self.cur = 0;
        self.smooth = Vec::new();
        
        self.alpha = Some(alpha);
        self.smooth = self.exp_smooth(alpha); //generates the smoothing
        /* or
        Self::exp_smooth(self.series.get_data()[self.cur..], alpha);
        */
        self
    }

    /// Takes data and an alpha value to smooth it.
    fn exp_smooth(&mut self, alpha: f64) -> Vec<(f64, f64)> {
        let mut smooth = Vec::new();
        if self.cur == 0 { //only smoothing, no forecasting
            let new_cur = self.series.len();
            let data = self.series.get_data();
            let mut last_fore = data[0].1; // for only smoothing, the first real is the last forecast in iteration one
            for i in self.cur + 1..new_cur { //for each value
                smooth.push(//the current x ignoring the first, the y of the first and the last forecast for y
                    ((i + 1) as f64, Self::calculate_smooth(alpha, data[i - 1].1, last_fore))
                );
                last_fore = smooth.last().expect("No last forecasted value").1; //get the value calculated before
            }
            self.cur = new_cur; //store the last smoothed value
        }
        
        smooth
    }

    pub fn get_data(&self) -> Vec<(f64, f64)> {
        if self.alpha.is_none() {
            panic!("Can't get data of ExpSmoothing with no alpha");
        }
        self.smooth.clone()
    }

    pub fn style(&self) -> Style {
        self.series.style()
    }

    pub fn as_time_series(&self) -> TimeSeries {
        TimeSeries::from_pairs_vec(self.get_data())
    }

    fn calculate_smooth(alpha: f64, last: f64, last_fore: f64) -> f64 {
        if alpha > 1.0 || alpha < 0.0 {
            panic!("bad alpha argument for ExpSmooth");
        }
        alpha * last + (1.0 - alpha) * last_fore
    }

    pub fn alpha(&self) -> f64 {
        self.alpha.expect("Couldn't get alpha from expsmooth: None")
    }

}

impl Plotable for ExpSmoothing {
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
