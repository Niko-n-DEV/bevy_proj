#![allow(unused)]
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Resource)]
pub struct ItemsAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub ids: Option<HashMap<String, usize>>,
}

#[derive(Component, Resource)]
pub struct MaterialAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub ids: Option<HashMap<String, usize>>,
}

// Атлас для хранения текстур местности
#[derive(Component, Resource)]
pub struct TerrainAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub ids: Option<HashMap<String, usize>>,
}

impl TerrainAtlas {
    /// Получить индекс текстуры в атласе по его имени
    pub fn get_index(name: &str, atlas: &Self) -> usize {
        if let Some(ids) = &atlas.ids {
            if let Some(index) = ids.get(name) {
                return *index;
            }
        }
        0
    }

    pub fn set_sprite(name: &str, atlas: &Self) -> SpriteSheetBundle {
        let sprite_sheet = SpriteSheetBundle {
            texture: atlas.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: atlas.layout.clone().unwrap(),
                index: Self::get_index(&name, &atlas),
            },
            ..default()
        };
        sprite_sheet
    }
}

#[derive(Component, Resource)]
pub struct VehicleAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub ids: Option<HashMap<String, usize>>,
}

#[derive(Component, Resource)]
pub struct ObjectAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub ids: Option<HashMap<String, usize>>,
}

// [Test]
#[derive(Resource)]
pub struct TestTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub ids: Option<HashMap<String, usize>>,
}

impl Default for TestTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
            ids: None,
        }
    }
}

impl TestTextureAtlas {
    /// Получить индекс текстуры в атласе по его имени
    pub fn get_index(name: &str, atlas: &Self) -> usize {
        if let Some(ids) = &atlas.ids {
            if let Some(index) = ids.get(name) {
                return *index;
            }
        }
        0
    }

    pub fn set_sprite(name: &str, atlas: &Self) -> SpriteSheetBundle {
        let sprite_sheet = SpriteSheetBundle {
            texture: atlas.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: atlas.layout.clone().unwrap(),
                index: Self::get_index(&name, &atlas),
            },
            ..default()
        };
        sprite_sheet
    }
}

#[derive(Component, Reflect, PartialEq)]
pub enum OrientationState {
    South,
    North,
    West,
    East,
}

// для сторонозависимых атласов
#[derive(Resource, Component)]
pub struct DirectionAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub ids: Option<HashMap<String, usize>>,
}

impl Default for DirectionAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
            ids: None,
        }
    }
}
// ! Сделать функцию, которая будет возвращать текстуру
impl DirectionAtlas {
    /// Получить индекс текстуры в атласе по его имени
    pub fn get_index(name: &str, atlas: &Self) -> usize {
        if let Some(ids) = &atlas.ids {
            if let Some(index) = ids.get(name) {
                return *index;
            }
        }
        0
    }

    pub fn set_sprite(name: &str, atlas: &Self) -> (Handle<Image>, TextureAtlas) {
        let texture = atlas.image.clone().unwrap();
        let atlas = TextureAtlas {
            layout: atlas.layout.clone().unwrap(),
            index: Self::get_index(name, &atlas),
        };

        (texture, atlas)
    }
}

/*
    Реализовать тут основные компоненты атласов, а точнее определение их и индексирование внутренних элементов,
    чтобы в дальнейшем по id/имени к ним можно было обратиться, и применить их к объекту.
    Так же, нужно реализовать индексацию анимированных элементов, путём запоминания их с общей таблицы в отдельные, но с друг-другом взаимосвязанными
*/