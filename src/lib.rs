/// This library was made by Edgar Regalado
/// its under the MIT License
/// may not be useful.
/// don't complain.


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
       // println!("{:?}", season.get_data());

        let mut season2 = Season::new(&season.as_time_series(), 4).set_season(2);
        Page::single(
            season2.plot().as_ref()
        ).save("seriedemanda_time2.svg").unwrap();
        //println!("{:?}", season.get_data());
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

    use crate::time_series::Merger;
    #[test]
    fn test_merger() {
        let demanda = [120.0, 80.0, 70.0, 60.0, 70.0, 90.0, 90.0,
        60.0, 70.0, 80.0, 100.0, 120.0];
        let series = TimeSeries::new(demanda.to_vec());
        Page::single(
            series.plot().as_ref()
        ).save("median_origin.svg").unwrap();

        let demanda2 = [100.0, 60.0, 80.0, 80.0, 90.0, 100.0, 80.0,
        90.0, 50.0, 40.0, 70.0, 80.0];

        let demanda3 = [90.0, 50.0, 70.0, 80.0, 80.0, 120.0, 100.0,
        120.0, 70.0, 70.0, 90.0, 120.0];

        let t2 = TimeSeries::new(demanda2.to_vec());
        let t3 = TimeSeries::new(demanda3.to_vec());

        let mut merger = Merger::new(&series).merge_with(&t2);
        Page::single(
            merger.as_time_series().plot().as_ref()
        ).save("merged2.svg").unwrap();
        
        merger = merger.merge_with(&t3);
        println!("{:?}", merger.as_time_series().get_data());
        Page::single(
            merger.as_time_series().plot().as_ref() //merger dropped
        ).save("merged3.svg").unwrap();

        let filtered_merger = MovingMedian::new(&merger.as_time_series());
        println!("{:?}", filtered_merger.get_data());
        Page::single(
            filtered_merger.plot().as_ref()
        ).save("merged3_moving_medians.svg").unwrap();

        let filtered_second = MovingMedian::new(&filtered_merger.as_time_series());

        Page::single(
            filtered_second.plot().as_ref()
        ).save("merged3_second_moving.svg").unwrap();
    }

    use crate::time_series::{Grouper, Style};
    #[test]
    fn test_grouper() {
        let demanda1 = [120.0, 80.0, 70.0, 60.0, 70.0, 90.0, 90.0,
        60.0, 70.0, 80.0, 100.0, 120.0];

        let demanda2 = [100.0, 60.0, 80.0, 80.0, 90.0, 100.0, 80.0,
        90.0, 50.0, 40.0, 70.0, 80.0];

        let demanda3 = [90.0, 50.0, 70.0, 80.0, 80.0, 120.0, 100.0,
        120.0, 70.0, 70.0, 90.0, 120.0];

        let t1 = TimeSeries::new(demanda1.to_vec());
        let t2 = TimeSeries::new(demanda2.to_vec());
        let t3 = TimeSeries::new(demanda3.to_vec());

        let complete: TimeSeries = Merger::new(&t1)
        .merge_with(&t2)
        .merge_with(&t3)
        .as_time_series();

        let filtered = MovingMedian::new(&complete);
        let filtered2 = MovingMedian::new(&filtered.as_time_series());
        println!("orig:{:?}\nfil_uno:{:?}\nfil_dos:{:?}",
        complete.get_data(),
        filtered.as_time_series().get_data(),
        filtered2.as_time_series().get_data()
        );
        let group = Grouper::new(&complete)
        .add(&filtered.as_time_series())
        .last_with_style(Style::from_color("#ff0000"))
        .add(&filtered2.as_time_series())
        .last_with_style(Style::from_color("#00ff00"));

        Page::single(
            group.plot().as_ref()
        ).save("series_filter.svg").unwrap()
    }
}
