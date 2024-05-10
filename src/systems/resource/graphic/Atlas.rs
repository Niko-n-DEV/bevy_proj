#![allow(unused)]
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct AtlasRes {
    pub items:      Option<ItemsAtlas>,
    pub material:   Option<MaterialAtlas>,
    pub terrain:    Option<TerrainAtlas>,
    pub objects:    Option<ObjectAtlas>,
    pub entity:     Option<DirectionAtlas>,
    pub test:       Option<TestTextureAtlas>
}

impl AtlasRes {
    pub fn init() -> Self {
        Self {
            items:      Some(ItemsAtlas::default()),
            material:   Some(MaterialAtlas::default()),
            terrain:    Some(TerrainAtlas::default()),
            objects:    Some(ObjectAtlas::default()),
            entity:     Some(DirectionAtlas::default()),
            test:       Some(TestTextureAtlas::default()),
        }
    }
}

#[derive(Resource, Default)]
pub struct ItemsAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

#[derive(Resource, Default)]
pub struct MaterialAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

// Атлас для хранения текстур местности
#[derive(Resource, Default)]
pub struct TerrainAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
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

#[derive(Resource, Default)]
pub struct ObjectAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

// [Test]
#[derive(Resource, Default)]
pub struct TestTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
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
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

impl Default for DirectionAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image:  None,
            ids:    None,
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

    pub fn create_canvas(
        mut commands: Commands,
        // кол-во entity и их текстур
    ) {
        // Создание холста атласа по кол-ву ентити 
    }
}

/*
    Реализовать тут основные компоненты атласов, а точнее определение их и индексирование внутренних элементов,
    чтобы в дальнейшем по id/имени к ним можно было обратиться, и применить их к объекту.
    Так же, нужно реализовать индексацию анимированных элементов, путём запоминания их с общей таблицы в отдельные, но с друг-другом взаимосвязанными
*/
