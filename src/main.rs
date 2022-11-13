use std::process::{Command, exit};
#[allow(unused_assignments)]
fn main() {
    let default_str = Command::new("bash").
        arg("-c").
        arg("pactl info | awk '/Default Sink: /{print $3}'").
        output().
        expect("Shell process failed").stdout;

    let sink_string = String::from(
        String::from_utf8_lossy(&default_str)
    );
    let mut name = String::new();
    if sink_string.contains("Schiit") {name = "Schiit Modi 3+".to_string();}
    else if sink_string.contains("hdmi") {name = "Monitor".to_string();}
    else if sink_string.contains("pci") {name = "Built-in Audio".to_string();}
    else if sink_string.contains("bluez") {name = "Bluetooth".to_string();}
    else {name = String::from(&sink_string[..4]);}
    println!("|{}|",name);
    exit(0)
}
