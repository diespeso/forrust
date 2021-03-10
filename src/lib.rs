//pub mod data;
pub mod plotable;
pub mod time_series;

#[cfg(test)]
mod tests {
    
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_data() {
      /*  let demanda = [120.0, 80.0, 70.0, 60.0, 70.0, 90.0, 90.0,
        60.0, 70.0, 80.0, 100.0, 120.0, 70.0, 40.0];
        let tabla = TablaDemanda::new(demanda.into());
        println!("{}", tabla);
        println!("uwu"); */
    }

    use crate::plotable::Plotable;
    use crate::time_series::TimeSeries;

    use plotlib::page::Page;
    #[test]
    fn test_time_series() {
        let demanda = [120.0, 80.0, 70.0, 60.0, 70.0, 90.0, 90.0,
        60.0, 70.0, 80.0, 100.0, 120.0, 70.0, 40.0];
        let series = TimeSeries::new(demanda.to_vec());
        Page::single(series.plot().as_ref()).save("seriedemanda.svg").unwrap();
    }

    use crate::time_series::Season;
    #[test]
    fn test_season() {
        let demanda = [120.0, 80.0, 70.0, 60.0, 70.0, 90.0, 90.0,
        60.0, 70.0, 80.0, 100.0, 120.0, 70.0, 40.0, 60.0, 50.0, 90.0];
        let series = TimeSeries::new(demanda.to_vec());
        Page::single(
            series.plot().as_ref()
        ).save("serie_total.svg").unwrap();
        let mut season = Season::new(&series, 12).set_season(1);
        Page::single(
            season.plot().as_ref()
        ).save("seriedemanda_anio_2.svg").unwrap();
        println!("{:?}", season.get_data());

        let mut season2 = Season::new(&season.as_time_series(), 4).set_season(2);
        Page::single(
            season2.plot().as_ref()
        ).save("seriedemanda_time2.svg").unwrap();
        println!("{:?}", season.get_data());
    }

    use crate::time_series::MovingMedian;
    #[test]
    fn test_moving_median() {
        let demanda = [120.0, 80.0, 70.0, 60.0, 70.0, 90.0, 90.0,
        60.0, 70.0, 80.0, 100.0, 120.0, 70.0, 40.0, 60.0, 50.0, 90.0];
        let series = TimeSeries::new(demanda.to_vec());
        Page::single(
            series.plot().as_ref()
        ).save("median_origin.svg").unwrap();
        let medians = MovingMedian::new(&series);
        Page::single(
            medians.plot().as_ref()
        ).save("median_result.svg").unwrap();
    }
}
