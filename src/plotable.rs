use plotlib::repr::Plot;
use crate::data::Data;

/// Plottables can be made into plotlib Plots
pub trait Plotable: Data {
    /// Returns a plotlib Plot representing this object
    fn plot(&self) -> Plot;
}
