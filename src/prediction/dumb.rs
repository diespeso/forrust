use crate::forecasting::ExpSmoothing;
use crate::time_series::TimeSeries;
use crate::regression::LinearRegression;

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
        let sm = TimeSeries::from_pairs_vec(sm);
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
            println!("{:?}", x_points);
            x_points = x_points[1..].iter().map(
                |x| {x.iter().map(
                    |y| {
                        println!("y: {:?}", y);
                        self.get_linear_regression().calculate(y.clone())
                    }
                ).collect()}
            ).collect();
            println!("{:?}", &x_points[1..]); //ignore january
            //TODO: Calculate 
        } else {
            panic!("Can't update Dumbs data, no expsmooth set.");
        }



        
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