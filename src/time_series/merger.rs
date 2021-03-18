use crate::time_series::TimeSeries;

/// Merges multiple time series into one,
/// but can only merge time series of the same length or the same multiple.
/// the point is: it will always yield complete seasons if you use a season
/// on the as_time_series method with the same length as the base.
/// Once time series have been merged they can only
/// be separated using season, but this doesnt affect
/// the merger.
/// but don't worry, merger only uses clones ;)
/// 
/// If you just need to append data to an existing time series
/// then used the push func in the time series, you dumb thing.
pub struct Merger {
    pub series: TimeSeries,
    pub data: Vec<(f64, f64)>,
    length: usize,
}

impl Merger {
    /// Creates a new Merger using a timeseries as a base, or first season
    pub fn new(base: &TimeSeries) -> Self {
        Self {
            series: base.clone(),
            data: base.get_data().clone(),
            length: base.len(),
        }
    }

    pub fn merge_with(mut self, series: &TimeSeries) -> Self {
        if series.len() != self.merge_len() {
            panic!("can't merge a time series with a length different than the base's")
        }
        for i in 0..series.len() {
            self.series.push(series.get_range_at_ord(i))
        }
        self.update_data();
        self
    }

    /// Returns the lenght that any time series to merge must have (the base length)
    pub fn merge_len(&self) -> usize {
        self.length
    }

    /// Returns the total length of the timeseries merged
    pub fn len(&self) -> usize {
        self.series.len()
    }

    pub fn as_time_series(&self) -> TimeSeries {
       TimeSeries::from_pairs_vec(self.data.clone())
    }

    /// Remakes the merger data every time
    /// uses its time series to fill its data
    fn update_data(&mut self) {
        self.data = self.series.get_data();
    }
    
}