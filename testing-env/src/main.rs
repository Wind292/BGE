use zenith::*;

fn main() {
    let mut instance = Instance2D::new();

    instance.environment.add_update_script("player movement", |x| {});

    let player = Entity::new().with_name_tag("player").with_tag("location", TagValue::Vec2(Vec2::new(0,0)));

    instance.start();
}
 

// fn player_up(instance: &mut Instance2D ) {
    
//     let mut entity = instance.environment.get_mut_entity("player").unwrap();

//     entity.set_tag("location", )


// }