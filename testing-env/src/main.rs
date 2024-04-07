use zenith::*;

fn main() {
    let mut instance = Instance2D::new_skeleton();

    instance
        .environment
        .add_entity(Entity::new().with_name_tag("test"));

    let mut e = Entity::new().with_name_tag("hello");

    instance.environment.overwrite("test", &mut e);

    instance.environment.print_all_entities()

}
