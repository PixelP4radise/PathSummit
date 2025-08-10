use crate::configuration::get_configuration;

mod configuration;

fn main() {
    let graphSettings = get_configuration("test_files/test.txt");
    
}
