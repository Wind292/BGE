use zenith::*;

fn main() {
    let instance = Instance2D::new();

    instance.environment.print_all_entities();

    instance.environment.print_all_scripts();
    
    instance.start()

}
