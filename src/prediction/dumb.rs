use crate::forecasting::ExpSmoothing;
use crate::time_series::{TimeSeries, Style};
use crate::regression::LinearRegression;
use crate::plotable::Plotable;
use crate::time_series::Grouper;

use plotlib::view::{View, ContinuousView};
use plotlib::repr::Plot;

const DEFAULT_ALPHA: f64 = 0.4;

/// Takes an exponential smoothing and makes a dumb prediction
/// of the next season
/// This uses an algorithm i made up myself
/// that takes to account the distances the exponential smoothing
/// for each month relative to the linear regresion of a time series
/// and calculates a growth factor thats 'a prediction' of the signal
/// for the future, and thats when i use a random value to add
/// some noise
pub struct Dumb {
    original: TimeSeries,
    expsmooth: Option<ExpSmoothing>,
    linear_reg: Option<LinearRegression>,
    season: usize,
    prediction: Vec<(f64, f64)>,
}

impl Dumb {
    pub fn new(time_series: &TimeSeries) -> Self {
        let mut this = Self {
            original: time_series.clone(),
            expsmooth: None,
            linear_reg: None,
            season: 0,
            prediction: Vec::new(),
        };
        this.expsmooth = Some(
            ExpSmoothing::new(time_series).with_alpha(DEFAULT_ALPHA)
        );
        this.linear_reg = Some(LinearRegression::new(time_series));
        this
    }

    /// Sets the length of the season
    /// its a must for the simulation
    /// Dumb is usable after this point
    pub fn with_season(mut self, season: usize) -> Self {
        self.season = season; //0 not set, panics
        self.update_data();
        self
    }

    fn update_data(&mut self) {
        if self.season == 0 {
            panic!("No season length set for Dumb")
        }

        let mut sm = self.exp_smooth().as_time_series().get_data();
        sm.insert(0, (1.0, -1.0));
        let mut sm = TimeSeries::from_pairs_vec(sm);
        //first, the boostrap
        //enero bootstrap
        //cada mes luego de enero usar mi algoritmo
        if let Some(expsmooth) = &self.expsmooth {
            self.prediction = Vec::new(); //reset
            let alpha = expsmooth.alpha();
            let len = sm.len();
            let r = alpha * sm.get_range_at(len - 1) + (1.0 - alpha) * expsmooth.as_time_series().get_range_at(len - 1); //last
            
            self.prediction.push((1.0, r)); //first value just a boostrap
           
           //separating all months in sesons acording to season length (self.season)
            let mut past = Vec::new();
            let mut c = 1;
            for seas in 0..(sm.len() / self.season) as usize {
                //for every past season of each month
                past.push(vec![0.0; self.season]);
                for i in 0..self.season {
                    //for each point of this season
                    past[seas][i] = sm.get_range_at(c);
                    c += 1;
                }
                
            }

            //for each month, get its seasonal components
            //its like inverting the array from 2x12 to 12x2
            let mut months = Vec::new(); //holds a vector of past values for every season of a month
            for month in 0..self.season {
                months.push(vec![0.0; past.len()]);
                for season in 0..past.len() {
                    months[month][season] = past[season][month];
                }
            }
            //println!("{:?}", months);
            //get distances
            let mut x_points = Vec::new();
            //x_points.push(vec![-1.0;past.len()]);//ignore first element: january
            for i in 0..months.len() + 1 {//every month, its seasons
                x_points.push(vec![0.0; past.len()]);
                for season in 0..months[0].len() { //every season, every year
                    x_points[i][season] = (i + season*self.season) as f64;
                }
                
            }
            //println!("{:?}", past);
            //println!("{:?}", x_points);
            //regression points used to get the distance between
            //the regression line and the exponential smoothing signal
            let regression_points: Vec<Vec<f64>> = x_points[1..].iter().map(
                |x| {x.iter().map(
                    |y| {
                        self.get_linear_regression().calculate(y.clone())
                    }
                ).collect()}
            ).collect();
            //println!("{:?}", &x_points[2..]); //ignore null and january
            //println!("{:?}", self.exp_smooth().as_time_series().get_data());
            //println!("{:?}", &regression_points[1..]);

            let smooth = self.exp_smooth().as_time_series();

            //x_points an regression_points have the same sice,
            //x_points represents the numbers in x of each month
            //regression_points represents the value of regresion in each season for amonths
            let x_points = (&x_points[2..]).to_vec();
            let regression_points = (&regression_points).to_vec();

            /// get the distances between the exp smooth and linear reg of a month every year
            let mut distances = Vec::new();
            for i in 0..x_points.len() { //every month pair
                distances.push(Vec::new());
                for j in 0..x_points[i].len() { //every element of the pair
                    distances[i].push(smooth.get_range_at(x_points[i][j] as usize) - regression_points[i][j]);
                    //print!("distance: {}, ", smooth.get_range_at(x_points[i][j] as usize) - regression_points[i][j] )
                }
            }

            let mut factors = Vec::new();
            for i in 0..distances.len() {
                factors.push(Vec::new());
                for j in 0..distances[i].len() - 1 {
                    let mut growth = distances[i][j + 1] / distances[i][j];
                    //FIXME: con que uno sea nega ya todo es nega
                    //muffling para negativos
                    if distances[i][j] < 0.0  && distances[i][j + 1] > 0.0 { //last below linear reg
                        //change sign and use only half
                        growth = -1.0 * growth / 2.0;
                    }

                    if growth > 10.0 || growth < -10.0 {
                        growth = DEFAULT_ALPHA * growth;
                    }
                    factors[i].push(growth);//the lastes divided by the one before: growth
                }
            }

            //factors to multiply last distances for
            let mut pro_factors = Vec::new();
            for i in 0..factors.len() { //hardcoded for 2 factors, in 3 years
                if factors[i].len() != 2 {
                    panic!("HARDCODED FOR 2 FACTORS IN 3 YEARS");
                }
                pro_factors.push(factors[i][0] * 0.8 + factors[i][1]);
            }

            let mut last_d = Vec::new(); //hardcoded for 2 factors, in 3 years
            for i in 0..distances.len() {
                last_d.push(distances[i][2] * pro_factors[i]);
            }
            //println!("ex = {:?}", expsmooth.as_time_series().get_data());
            println!("d = {:?}", distances);
            //println!("f = {:?}", factors);
            //println!("pro= {:?}", pro_factors);
            println!("lastd = {:?}", last_d);
            let mut finales = Vec::new();
            for i in 0..last_d.len() {

                finales.push(last_d[i] + self.get_linear_regression().calculate(i as f64 + 36.0));
            }  
            let mut counter = 38.0; //december + 1
            for element in finales.iter() {
                self.prediction.push((counter, element.clone()));
                counter += 1.0;
            }

            //HARDCODE: FIRST ELEMENT IN PREDICTION IS 37: JAN YEAR 4
            self.prediction[0].0 = 37.0;
            println!("finales: {:?}", self.prediction());
            //TODO: Calculate distance between regression points
            //and exponential smooting signal

            //TODO: get the growth factors
            //TODO: muffle growths: for 3 seasons use (0.4, 0.6) -> muffling factors
            //TODO: new point: (muffled 1 + muffled 2) * last distance
            //TODO: add noise to prediction signal, no noise algorithm yet
            //TODO: calculate regression in new point
            //TODO: add the combo (signal + noise) to the calculated regression point
            //TODO(CHECK): If the values get too spiky, muffle with mean * alpha factor at +- random level
        } else {
            panic!("Can't update Dumbs data, no expsmooth set.");
        }        
    }

    pub fn prediction(&self) -> Vec<(f64, f64)> {
        self.prediction.clone()
    }

    pub fn get_linear_regression(&self) -> LinearRegression {
        if let Some(reg) = &self.linear_reg {
            reg.clone()
        } else {
            panic!("No linear regression calculated for Dumb");
        }
    }

    fn exp_smooth(&self) -> ExpSmoothing {
        if let Some(ex) = &self.expsmooth {
            ex.clone()
        } else {
            panic!("Dumb has no exponential smoothing set")
        }
    }
}

impl Plotable for Dumb {
    fn plot(&self) -> Box<dyn View> {
        //plot every thing
        //timseries + regresion + smooth + prediction
        let mut tm = self.original.clone();
        let smooth = self.exp_smooth();
        let linear = self.get_linear_regression();
        let pred = TimeSeries::from_pairs_vec(self.prediction());

        let mut group = Grouper::new(&tm)
        .last_with_style(Style::from_color("#000000"))
        .add(&smooth.as_time_series())
        .last_with_style(Style::from_color("#af0af6"))
        .add(&linear.as_time_series())
        .last_with_style(Style::from_color("#87faa4"))
        .add(&pred)
        .last_with_style(Style::from_color("#ff00a2"));

        group.plot()
    }
    fn as_plot(&self) -> Plot {
        unimplemented!()
    }
}