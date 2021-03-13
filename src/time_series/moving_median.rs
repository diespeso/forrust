use crate::time_series::TimeSeries;
use crate::plotable::Plotable;
use crate::time_series::Style;

use std::convert::Into;

use plotlib::view::{View, ContinuousView};
use plotlib::repr::Plot;

//TODO: MAKE THIS CONVERTIBLE TO TIME SERIES SO I CAN APPLY A SECOND MOVING MEDIANS FILTER (A SECOND TIME I MEAN)


/// Represents de moving medians filter for a times series.
pub struct MovingMedian {
    series: TimeSeries,
    data: Option<Vec<(f64, f64)>>,
}

impl MovingMedian {
    /// Creates a new MovingMedian from a &TimeSeries
    pub fn new(series: &TimeSeries) -> Self {
        let mut r = Self {
            series: series.clone(),
            data: None,
        };
        r.update_data();
        r
    }

    /// Inner function that creates the moving medians data
    /// and puts it in the data field of this struct
    /// the first and last values are ignored
    fn update_data(&mut self) {
        let mut med = Vec::new();
        for i in 1..self.series.len() - 1 {
            let mut vals = [0.0, 0.0, 0.0];
            vals[0] = self.series.get_range()[i - 1];
            vals[1] = self.series.get_range()[i];
            vals[2] = self.series.get_range()[i + 1];
            med.push(
                ((i + 1) as f64, Self::get_middle_for_3(vals))
            );
        }
        self.data = Some(med);
    }

    /// Returns a vector of (x, y) pairs
    pub fn get_data(&self) -> Vec<(f64, f64)> {
        if let Some(data) = &self.data {
            return data.clone()
        } else {
            panic!("can't get data from empty moving median")
        }
    }

    /// Returns the style for this moving medians filter
    pub fn style(&self) -> Style {
        self.series.style()
    }

    /// Inner function, gets de median between 3 values with a step of 1
    fn get_middle_for_3(vals: [f64; 3]) -> f64 {
        //bad programming ahead
        let mut ints: Vec<u32> = vals.iter().map(|x| *x as u32).collect();
        let min = *ints.iter().min().unwrap();
        let max = *ints.iter().max().unwrap();

        ints.sort();
        ints[1] as f64
        /*
        for i in 0..ints.len() {
            println!("{}, {}, {}", min, ints[i], max);
            
            /*
            if ints[i] != min && ints[i] != max {
                return ints[i] as f64;
            }*/
        }
        panic!("couldn't find median");*/
    }

    pub fn as_time_series(&self) -> TimeSeries {
        TimeSeries::from_pairs_vec(self.get_data())
    }

}

impl Plotable for MovingMedian {
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