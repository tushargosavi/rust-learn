use std::vec::Vec;

fn to_iso_date(s: &str) -> String {
  let parts : Vec<&str> = s.split(" ").collect();
  format!("2022-01-{} {}", parts[1], parts[2])
}

fn main() {

}

mod test {
    use super::*;

    #[test]
    fn sample_test() {
        let date = "Jan 28 13:09:31";
        assert_eq!("2022-01-28 13:09:31", to_iso_date(date));
    }
}