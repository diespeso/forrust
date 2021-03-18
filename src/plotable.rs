use plotlib::view::View;
use plotlib::repr::Plot;


/// Plottables can be made into plotlib Plots
pub trait Plotable {
    /// Returns a plotlib Plot representing this object
    fn plot(&self) -> Box<dyn View>;

    fn as_plot(&self) -> Plot;
}
