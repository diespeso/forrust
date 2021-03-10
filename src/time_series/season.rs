use crate::time_series::{TimeSeries, Style};
use crate::plotable::Plotable;

use plotlib::repr::Plot;
use plotlib::view::{View, ContinuousView};

/// represents a subset of the data
pub struct Season {
    series: TimeSeries,
    length: usize,
    season: Option<usize>,
}

impl Season {
    /// Takes a reference to a time series to create a Season from it
    /// Also takes the length fo that season.
    pub fn new(series: &TimeSeries, length: usize) -> Self {
        Self {
            series: series.clone(),
            length,
            season: None,
        }
    }

    /// Sets the season to be used
    pub fn set_season(mut self, u: usize) -> Self {
        self.season = Some(u);
        self
    }

    /// Returns a clone of the current style
    pub fn style(&self) -> Style {
        self.series.style.clone()
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.series.style
    }

    /// Sets the style for this Season plot
    pub fn set_style(mut self, style: Style) -> Self {
        self.series.style = style;
        self
    }
}

impl Plotable for Season {
    fn plot(&self) -> Box<dyn View> {
        let mut v = Vec::new();
        let data = self.series.get_data_repr();
        if let Some(season) = self.season {
            let low = self.length * (season - 1);
            let mut high = low + self.length;
            if high > data.len() { //out of bounds
                high = data.len();
            }
            let mut x = 1;
            for i in low..high {
                let (_, y) = data[i];
                v.push((x as f64, y));
                x += 1;
            }
        } else {
            v = data;
        }

        let mut plot = Plot::new(v);
        plot = plot.point_style(self.style().point).line_style(self.style().line);
        Box::new(ContinuousView::new()
        .add(plot))
    }
}