use datetime::DateTime;

fn main() {
    let date = DateTime::now();

    println!("{}", date.as_time_stamp());
}
