use crate::time_series::{TimeSeries, Style};
use crate::plotable::Plotable;

use plotlib::repr::Plot;
use plotlib::view::{View, ContinuousView};


#[derive(Clone)]
/// represents a subset of the data
pub struct Season {
    series: TimeSeries,
    length: usize,
    season: Option<usize>,
    data: Option<Vec<(f64, f64)>>,
}

impl Season {
    /// Takes a reference to a time series to create a Season from it
    /// Also takes the length for that season.
    pub fn new(series: &TimeSeries, length: usize) -> Self {
        Self {
            series: series.clone(),
            length,
            season: None,
            data: None,
        }
    }

    /// Sets the season to be used
    pub fn set_season(mut self, u: usize) -> Self {
        self.season = Some(u);
        self.update_data();
        self
    }
    /*
    /// Makes a new season, doesnt affect current one
    pub fn get_season(&self, u: usize) -> Self {
        let mut this = self.clone();
        this.season = Some(u);
        this.update_data();
        this
    }

    pub fn get_all_seasons(&self) -> Vec<Season> {

    }

    /// How many seasons are there in the time series used to make this
    pub fn calculate_season_number(&self) -> usize {
        (self.series.len() as f64 / self.length as f64).ceil() as usize
    }*/

    /// Makes the data that represents this season
    /// this inner function is called every time a new
    /// season is set
    fn update_data(&mut self) {
        let mut v = Vec::new();
        let data = self.series.get_data();
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
            self.data = Some(v); 
        } else {
            panic!("Can't generate season data without a set season");
        }       
    }

    pub fn as_time_series(&self) -> TimeSeries {
        TimeSeries::new(self.get_range())
    }

    pub fn get_range(&self) -> Vec<f64> {
        if self.data.is_none() {
            panic!("can't get domain from empty season");
        }
        let mut v = Vec::new();
        for i in 0..self.data.as_ref().unwrap().len() {
            v.push(self.data.as_ref().unwrap()[i].1); //ys
        }
        v
    }

    /// Returns the set of (x, y) points that represent this season
    pub fn get_data(&self) -> Vec<(f64, f64)> {
        if let Some(data) = &self.data {
            data.clone()
        } else {
            panic!("Season data empty");
        }
    }

    /// Returns a clone of the current style
    pub fn style(&self) -> Style {
        self.series.style()
    }

    /// Returns a mut ref to this Season's plot style
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
        let plot = self.as_plot();
        Box::new(ContinuousView::new()
        .add(plot))
    }

    fn as_plot(&self) -> Plot {
        Plot::new(self.get_data())
        .point_style(self.style().point)
        .line_style(self.style().line)
    }
}