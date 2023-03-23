use screeps::{game, RoomXY, StructureType};
use wasm_bindgen::prelude::*;
use room_visual_ext::RoomVisualExt;

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    fn show_sample(room_visual_ext: &mut RoomVisualExt, x_offset: f32, opacity: f32) {
        // All RoomVisual functions may also be used.
        let label = format!("RoomVisualExt with opacity={}", opacity);
        room_visual_ext.text(13f32 + x_offset, 6.5f32, label, None);

        // Roads first so that nothing renders below them.
        room_visual_ext.structure(13f32 + x_offset, 10f32, StructureType::Road, opacity);
        room_visual_ext.structure(12f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(11f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(10f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(9f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(8f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(14f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(15f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(16f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(17f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(18f32 + x_offset, 9f32, StructureType::Road, opacity);
        room_visual_ext.structure(12f32 + x_offset, 11f32, StructureType::Road, opacity);
        room_visual_ext.structure(14f32 + x_offset, 11f32, StructureType::Road, opacity);
        room_visual_ext.structure(12f32 + x_offset, 12f32, StructureType::Road, opacity);
        room_visual_ext.structure(14f32 + x_offset, 12f32, StructureType::Road, opacity);
        room_visual_ext.structure(12f32 + x_offset, 13f32, StructureType::Road, opacity);
        room_visual_ext.structure(14f32 + x_offset, 13f32, StructureType::Road, opacity);
        room_visual_ext.structure(12f32 + x_offset, 14f32, StructureType::Road, opacity);
        room_visual_ext.structure(14f32 + x_offset, 14f32, StructureType::Road, opacity);
        room_visual_ext.structure(13f32 + x_offset, 15f32, StructureType::Road, opacity);
        room_visual_ext.structure(13f32 + x_offset, 16f32, StructureType::Road, opacity);
        room_visual_ext.structure(12f32 + x_offset, 16f32, StructureType::Road, opacity);
        room_visual_ext.structure(14f32 + x_offset, 16f32, StructureType::Road, opacity);

        // RoomXY is also supported.
        room_visual_ext.structure_roomxy(
            unsafe { RoomXY::unchecked_new(13 + x_offset as u8, 14) },
            StructureType::Spawn,
            opacity,
        );
        room_visual_ext.structure(13f32 + x_offset, 13f32, StructureType::Nuker, opacity);
        room_visual_ext.structure(13f32 + x_offset, 12f32, StructureType::Link, opacity);
        room_visual_ext.structure(13f32 + x_offset, 11f32, StructureType::PowerSpawn, opacity);

        // Ramparts over the buildings below them, though preferably after all other structures
        // due to buildings that stick out, such as terminal.
        room_visual_ext.structure(13f32 + x_offset, 14f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(13f32 + x_offset, 13f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(13f32 + x_offset, 12f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(13f32 + x_offset, 11f32, StructureType::Rampart, opacity);

        // In particular, containers after roads below them, so that they do not render below them.
        room_visual_ext.structure(13f32 + x_offset, 16f32, StructureType::Container, opacity);
        room_visual_ext.structure(13f32 + x_offset, 17f32, StructureType::Extractor, opacity);

        room_visual_ext.structure(13f32 + x_offset, 9f32, StructureType::Terminal, opacity);
        room_visual_ext.structure(13f32 + x_offset, 8f32, StructureType::Extension, opacity);
        room_visual_ext.structure(12f32 + x_offset, 8f32, StructureType::Extension, opacity);
        room_visual_ext.structure(11f32 + x_offset, 8f32, StructureType::Extension, opacity);
        room_visual_ext.structure(14f32 + x_offset, 8f32, StructureType::Extension, opacity);
        room_visual_ext.structure(15f32 + x_offset, 8f32, StructureType::Extension, opacity);

        room_visual_ext.structure(12f32 + x_offset, 10f32, StructureType::Observer, opacity);
        room_visual_ext.structure(11f32 + x_offset, 10f32, StructureType::Extension, opacity);
        room_visual_ext.structure(11f32 + x_offset, 11f32, StructureType::Factory, opacity);
        room_visual_ext.structure(11f32 + x_offset, 12f32, StructureType::Extension, opacity);
        room_visual_ext.structure(11f32 + x_offset, 13f32, StructureType::Extension, opacity);
        room_visual_ext.structure(11f32 + x_offset, 14f32, StructureType::Storage, opacity);
        room_visual_ext.structure(11f32 + x_offset, 15f32, StructureType::Extension, opacity);
        room_visual_ext.structure(12f32 + x_offset, 15f32, StructureType::Extension, opacity);

        room_visual_ext.structure(14f32 + x_offset, 10f32, StructureType::Extension, opacity);
        room_visual_ext.structure(15f32 + x_offset, 10f32, StructureType::Extension, opacity);
        room_visual_ext.structure(15f32 + x_offset, 11f32, StructureType::Tower, opacity);
        room_visual_ext.structure(15f32 + x_offset, 12f32, StructureType::Lab, opacity);
        room_visual_ext.structure(15f32 + x_offset, 13f32, StructureType::Lab, opacity);
        room_visual_ext.structure(15f32 + x_offset, 14f32, StructureType::Lab, opacity);
        room_visual_ext.structure(15f32 + x_offset, 15f32, StructureType::Extension, opacity);
        room_visual_ext.structure(14f32 + x_offset, 15f32, StructureType::Extension, opacity);

        // No particular horizontal or vertical ordering is needed.
        room_visual_ext.structure(13f32 + x_offset, 9f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(13f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(12f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(11f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(10f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 9f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 10f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 11f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 12f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 13f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 14f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 15f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 16f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(9f32 + x_offset, 17f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(10f32 + x_offset, 14f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(10f32 + x_offset, 15f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(10f32 + x_offset, 16f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(10f32 + x_offset, 17f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(11f32 + x_offset, 17f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(11f32 + x_offset, 16f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(14f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(15f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(16f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(17f32 + x_offset, 8f32, StructureType::Rampart, opacity);
        room_visual_ext.structure(17f32 + x_offset, 9f32, StructureType::Rampart, opacity);

        room_visual_ext.structure(17f32 + x_offset, 10f32, StructureType::Wall, opacity);
        room_visual_ext.structure(17f32 + x_offset, 11f32, StructureType::Wall, opacity);
        room_visual_ext.structure(17f32 + x_offset, 12f32, StructureType::Wall, opacity);
        room_visual_ext.structure(17f32 + x_offset, 13f32, StructureType::Wall, opacity);
        room_visual_ext.structure(17f32 + x_offset, 14f32, StructureType::Wall, opacity);
        room_visual_ext.structure(17f32 + x_offset, 15f32, StructureType::Wall, opacity);
        room_visual_ext.structure(17f32 + x_offset, 16f32, StructureType::Wall, opacity);
        room_visual_ext.structure(17f32 + x_offset, 17f32, StructureType::Wall, opacity);
        room_visual_ext.structure(16f32 + x_offset, 14f32, StructureType::Wall, opacity);
        room_visual_ext.structure(16f32 + x_offset, 15f32, StructureType::Wall, opacity);
        room_visual_ext.structure(16f32 + x_offset, 16f32, StructureType::Wall, opacity);
        room_visual_ext.structure(16f32 + x_offset, 17f32, StructureType::Wall, opacity);
        room_visual_ext.structure(15f32 + x_offset, 16f32, StructureType::Wall, opacity);
        room_visual_ext.structure(15f32 + x_offset, 17f32, StructureType::Wall, opacity);
    }

    let spawn = game::spawns().values().next().unwrap();
    spawn.pos();
    let room_name = spawn.room().map(|r| r.name());
    let mut room_visual_ext = RoomVisualExt::new(room_name.unwrap());

    show_sample(&mut room_visual_ext, 1.0, 1.0);

    room_visual_ext.line((20f32, 7f32), (20f32, 19f32), None);

    // You may get the internal RoomVisual object and use it directly too.
    room_visual_ext.room_visual.text(26f32, 6.5f32, "Real, in-game objects".into(), None);

    room_visual_ext.line((32f32, 7f32), (32f32, 19f32), None);

    show_sample(&mut room_visual_ext, 25.0, 0.5);

    room_visual_ext.text(
        26f32, 20f32,
        "This image was generated using the examples/showcase.rs snippet".into(),
        None);
}