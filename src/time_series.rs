use crate::plotable::Plotable;

use std::fmt;

use plotlib::view::{View, CategoricalView, ContinuousView};
use plotlib::repr::{Plot, CategoricalRepresentation};
use plotlib::style::{PointStyle, LineStyle, PointMarker};
/// Holds the data of the series
/// its implied that every unit of data represents a unit of time
pub struct TimeSeries {
    data: Vec<f64>,
    dom_ran: Option<(String, String)>,
    style: Option<(Option<PointStyle>, Option<LineStyle>)>
}

impl TimeSeries {

    /// Creates a new Timeseries with the data given for the y axis.
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data: data,
            dom_ran: None,
            style: Some(
                (Some(PointStyle::new().colour("#000000").marker(PointMarker::Circle)),
                Some(LineStyle::new().colour("#000000"))
                )   
            )
        }
    }

    fn get_data_repr(&self) ->Vec<(f64, f64)> {
        let mut vec = Vec::new();
        for i in 0..self.data.len() {
            vec.push(
                (i as f64, self.data[i] as f64)
            );
        }
        vec
    }
}


impl Plotable for TimeSeries {
    fn plot(&self) -> Box<dyn View> {
        let mut plot = Plot::new(self.get_data_repr());
        if let Some(styles) = &self.style {
            if let Some(pointstyle) = &styles.0 {
                plot = plot.point_style(pointstyle.clone());
            }
            if let Some(linestyle) = &styles.1 {
                plot = plot.line_style(linestyle.clone());
            }
        }
        let mut view = ContinuousView::new()
        .add(plot);
        Box::new(view)
    }
}

/// Reprents a repeteable domain
pub struct CustomDomain {

}

/// Un Mes del año.
#[derive(Copy, Clone, Debug)]
enum Mes {
    Enero,
    Febrero,
    Marzo,
    Abril,
    Mayo,
    Junio,
    Julio,
    Agosto,
    Septiembre,
    Octubre,
    Noviembre,
    Diciembre

}

impl fmt::Display for Mes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl From<u32> for Mes {
    fn from(v: u32) -> Self {
        
        match v {
            0 => Self::Enero,
            1 => Self::Febrero,
            2 => Self::Marzo,
            3 => Self::Abril,
            4 => Self::Mayo,
            5 => Self::Junio,
            6 => Self::Julio,
            7 => Self::Agosto,
            8 => Self::Septiembre,
            9 => Self::Octubre,
            10 => Self::Noviembre,
            11 => Self::Diciembre,
            _ => panic!(format!("Número de mes inválido: {}", v))
        }
    }
}

impl From<&str> for Mes {
    fn from(v: &str) -> Self {
        let v = v.to_lowercase();
        match v.as_str() {
           "enero" => Self::Enero,
           "febrero" => Self::Febrero,
           "marzo" => Self::Marzo,
           "abril" => Self::Abril,
           "mayo" => Self::Mayo,
           "junio" => Self::Junio,
           "julio" => Self::Julio,
           "agosto" => Self::Agosto,
           "septiembre" => Self::Septiembre,
           "octubre" => Self::Octubre,
           "noviembre" => Self::Noviembre,
           "diciembre" => Self::Diciembre,
           _ => panic!(format!("Mes inválido: {}", v))
        }
    }
}

