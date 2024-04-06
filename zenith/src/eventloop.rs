use crate::Instance2D;
use crate::RunEvent;
use crate::RunEventList;
use std::time::{Duration, Instant};

pub fn eventloop(instance: &mut Instance2D) {

    start_entities(instance); // Run the start functions on all of the entities

    let framerate_goal = instance.screen.framerate_cap;
    let frame_duration = Duration::from_secs(1) / framerate_goal as u32;
    let mut last_frame_time = Instant::now();

   

    loop {
        if !instance.engine_settings.is_running {break}
        let frame_start_time = Instant::now();

        let mut run_event_list = RunEventList::empty();

        update_entities(instance, &mut run_event_list);

        maintain_framerate(frame_duration, &mut last_frame_time, frame_start_time);
    }
}

fn update_entities(instance: &mut Instance2D, run_event_list: &mut RunEventList) {
    for entity in &instance.environment.entities {
        if let Some(update_function) = entity.update_function {
            run_event_list.merge(&mut update_function(instance));
        }
    }

    for event in &run_event_list.events {
        execute_run_event(*event, instance)
    }

}

fn execute_run_event(run_event: RunEvent, instance: &mut Instance2D) {
    match run_event {
        RunEvent::Quit => {
            instance.engine_settings.is_running = false;
        }
    }
}

fn start_entities(instance: &Instance2D) {
    for entity in &instance.environment.entities {
        if let Some(start_function) = entity.start_function {
            start_function(instance);
        }
    }
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
