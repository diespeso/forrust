use crate::time_series::{TimeSeries, Style};
use crate::plotable::Plotable;

use plotlib::view::{View, ContinuousView};
use plotlib::repr::Plot;

/// Groups timeseries in a same plot
/// is kinda dumb cause once a timeseries is in
/// you can't modify it.
/// it's plotabble but you can't use as_plot with this one
pub struct Grouper {
    group: Vec<TimeSeries>,
}

impl Grouper {

    /// Creates a new grouper with a base, clones the grouper
    pub fn new(series: &TimeSeries) -> Self {
        Self {
            group: vec!(series.clone())
        }
    }

    pub fn add(mut self, series: &TimeSeries) -> Self{
        self.group.push(series.clone());
        self
    }

    /// Sets the style of the last added TimeSeries
    pub fn last_with_style(mut self, style: Style) -> Self {
        *self.last_mut().style_mut() = style;
        self
    }

    fn last_mut(&mut self) -> &mut TimeSeries {
        self.group.last_mut().expect("Can't get last element in grouper")
    }
}

impl Plotable for Grouper {
    /// Returns a view with all the plots of this grouper
    fn plot(&self) -> Box<dyn View> {
       // let plots: Vec<Plot> = self.group.iter().map(|x| x.as_plot()).collect();

        let mut view = ContinuousView::new();
        for i in 0..self.group.len() {
            view = view.add(self.group[i].as_plot());
        }
        Box::new(view)
    }

    fn as_plot(&self) -> Plot {
        panic!("Sorry, mate, you can't get a plot from a grouper")
    }
}

impl Default for Grouper {
    fn default() -> Self {
        Self {
            group: Vec::default()
        }
    }
}