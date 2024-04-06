use crate::render::update_keystrokes;
use crate::Instance2D;
use std::time::{Duration, Instant};

pub fn eventloop(instance: Instance2D) {
    let mut instance = start_entities(instance); // Run the start functions on all of the entities

    let framerate_goal = instance.screen.framerate_cap;
    let frame_duration = Duration::from_secs(1) / framerate_goal as u32;
    let mut last_frame_time = Instant::now();

    loop {
        let frame_start_time = Instant::now();
        if !instance.engine_settings.is_running {
            break;
        }

        update_keystrokes(&mut instance);

        instance = update_entities(instance);
        
        maintain_framerate(frame_duration, &mut last_frame_time, frame_start_time);
    }
}

fn update_entities(mut instance: Instance2D) -> Instance2D {
    let entities = instance.environment.entities.clone();
    for entity in entities {
        if let Some(update_function) = &entity.update_function {
            update_function(&mut instance);
        }
    }
    instance
}

fn start_entities(mut instance: Instance2D) -> Instance2D {
    let entities = instance.environment.entities.clone();
    for entity in entities {
        if let Some(start_function) = &entity.start_function {
            start_function(&mut instance);
        }
    }
    instance
}

fn maintain_framerate(
    frame_duration: Duration,
    last_frame_time: &mut Instant,
    frame_start_time: Instant,
) {
    let frame_time = Instant::now().duration_since(frame_start_time);

    if frame_time < frame_duration {
        std::thread::sleep(frame_duration - frame_time);
    }

    let total_frame_time = Instant::now().duration_since(*last_frame_time);
    let elapsed = total_frame_time - frame_duration;

    if elapsed < frame_duration {
        std::thread::sleep(frame_duration - elapsed);
        // println!("{:?}", frame_duration - elapsed);
    }

    *last_frame_time = Instant::now();
}
