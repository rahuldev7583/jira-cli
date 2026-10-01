mod db;
mod model;
fn main() {
    println!("Welcome to JIRA!");
}

mod tests {

    #[test]
    fn test_print() {
        println!("hi");
        let res = 1;
        assert_eq!(1, res);
    }

    #[test]
    fn test_second() {
        println!("hi");
        let res = 1;
        assert_eq!(1, res);
    }
}
