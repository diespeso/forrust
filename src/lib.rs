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
        Page::single(
            Season::new(&series, 12).set_season(2).plot().as_ref()
        ).save("seriedemanda_anio_2.svg").unwrap();
    }
}
