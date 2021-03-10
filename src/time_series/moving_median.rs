use crate::time_series::TimeSeries;
use crate::plotable::Plotable;
use crate::time_series::Style;

use std::convert::Into;

use plotlib::view::{View, ContinuousView};
use plotlib::repr::Plot;

pub struct MovingMedian {
    series: TimeSeries,
    data: Option<Vec<(f64, f64)>>,
}

impl MovingMedian {
    pub fn new(series: &TimeSeries) -> Self {
        let mut r = Self {
            series: series.clone(),
            data: None,
        };
        r.update_data();
        r
    }

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
            println!("{}, {}, {}", i - 1, i, i +1);
        }
        self.data = Some(med);
    }

    
    pub fn get_data(&self) -> Vec<(f64, f64)> {
        if let Some(data) = &self.data {
            return data.clone()
        } else {
            panic!("can't get data from empty moving median")
        }
    }

    pub fn style(&self) -> Style {
        self.series.style()
    }

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

}

impl Plotable for MovingMedian {
    fn plot(&self) -> Box<dyn View> {
        let v = self.get_data();
        let mut plot = Plot::new(v);
        plot = plot
        .point_style(self.style().point)
        .line_style(self.style().line);

        Box::new(ContinuousView::new().add(plot))
    }
}