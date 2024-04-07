use zenith::*;

fn main() {
    let mut instance = Instance2D::new();

    instance.environment.add_entity(Entity::new().with_update_fn(up));




    instance.start()
}


fn up(inst: &mut Instance2D) {
    if inst.get_pressed().ESCAPE {
        inst.quit()
    }
}