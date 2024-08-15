fn main() {
    let t = 23;
    let fuel = t;
    if fuel < 50{
        println!("fuel levels low");
    } else if fuel > 50{
        println!("tank full");
    } else {
        println!("check fuel tank");
    }
}