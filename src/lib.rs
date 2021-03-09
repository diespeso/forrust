pub mod data;
pub mod plotable;

#[cfg(test)]
mod tests {
    use crate::data::TablaDemanda;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_data() {
        let demanda = [120.0, 80.0, 70.0, 60.0, 70.0, 90.0, 90.0,
        60.0, 70.0, 80.0, 100.0, 120.0, 70.0, 40.0];
        let tabla = TablaDemanda::new(demanda.into());
        println!("{}", tabla);
        println!("uwu"); 
    }
}
