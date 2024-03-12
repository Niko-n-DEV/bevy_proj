use godot::prelude::*;

use crate::core::World;
use crate::core::Chunk;

pub fn place(mut world: Gd<World>, mouse_pos: Vector2i) {
    /*
        Эта функция скорее будет как макрос или что-то типо того.
        Данная функция будет промежуточным вызовом между игроко и "системой".
        Размещение чего-то на tilemap.
        Этапы выполнения: 
            Определение чанка, куда направлен курсор;
            Определение места в чанке, куда направлен курсор :);
            Замена плитки на указанный вариант.
    */

    let tile_aim_pos = world.bind_mut().get_current_chunk_tile(mouse_pos);
    let chunk_aim_pos = world.bind_mut().get_current_chunk(tile_aim_pos);
    let mut local_chunk_aim_pos: Vector2i = Vector2i::ZERO;
    if let Some(chunk) = world.bind_mut().local_chunk_coord(chunk_aim_pos).into() {
        if let Ok(mut ch) = chunk.try_cast::<Chunk>().into() {
            local_chunk_aim_pos = ch.bind_mut().get_local_coord_chunk(mouse_pos)
        } else {
            godot_error!("Error")
        }
    }
    godot_print!("{} | {} | {}",chunk_aim_pos, tile_aim_pos , local_chunk_aim_pos);
    world.bind_mut().tile_chunk(chunk_aim_pos, local_chunk_aim_pos)
}