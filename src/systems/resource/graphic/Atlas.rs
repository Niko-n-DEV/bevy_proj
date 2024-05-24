#![allow(unused)]
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct AtlasRes {
    pub items:      ItemsAtlas,
    pub material:   MaterialAtlas,
    pub terrain:    TerrainAtlas,
    pub objects:    ObjectAtlas,
    pub particle:   ParticleAtlas,
    pub entity:     DirectionAtlas,
    pub test:       TestTextureAtlas
}

impl AtlasRes {
    pub fn init() -> Self {
        Self {
            items:      ItemsAtlas::default(),
            material:   MaterialAtlas::default(),
            terrain:    TerrainAtlas::default(),
            objects:    ObjectAtlas::default(),
            particle:   ParticleAtlas::default(),
            entity:     DirectionAtlas::default(),
            test:       TestTextureAtlas::default(),
        }
    }

    pub fn get_entity_spritesheet(&self, name: &str) -> Option<SpriteSheetBundle> {
        if let Some(index) = &self.entity.ids {
            if let Some(index) = index.get(name) {
                let sprite = SpriteSheetBundle {
                    texture: self.entity.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: self.entity.layout.clone().unwrap(),
                        index: *index
                    },
                    ..default()
                };
                return Some(sprite);
            }
        }
        None
    }

    pub fn get_object_spritesheet(&self, name: &str) -> Option<SpriteSheetBundle> {
        if let Some(index) = &self.objects.ids {
            if let Some(index) = index.get(name) {
                let sprite = SpriteSheetBundle {
                    texture: self.objects.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: self.objects.layout.clone().unwrap(),
                        index: *index
                    },
                    ..default()
                };
                return Some(sprite);
            }
        }
        None
    }

    pub fn get_item_spritesheet(&self, name: &str) -> Option<SpriteSheetBundle> {
        if let Some(index) = &self.items.ids {
            if let Some(index) = index.get(name) {
                let sprite = SpriteSheetBundle {
                    texture: self.items.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: self.items.layout.clone().unwrap(),
                        index: *index
                    },
                    ..default()
                };
                return Some(sprite);
            }
        }
        None
    }

    pub fn get_test_spritesheet(&self, name: &str) -> Option<SpriteSheetBundle> {
        if let Some(index) = &self.test.ids {
            if let Some(index) = index.get(name) {
                let sprite = SpriteSheetBundle {
                    texture: self.test.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: self.test.layout.clone().unwrap(),
                        index: *index
                    },
                    ..default()
                };
                return Some(sprite);
            }
        }
        None
    }
}

/// Атлас для хранения текстур предметов
#[derive(Resource, Default)]
pub struct ItemsAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

impl ItemsAtlas {
    pub fn extruct_texture(&self, name: &str) -> Option<(TextureAtlas, Handle<Image>)> {
        if let Some(index) = &self.ids {
            if let Some(index) = index.get(name) {
                if let Some(atlas_texture) = &self.image {
                    if let Some(atlas_layout) = &self.layout {
                        let atlas = TextureAtlas {
                            layout: atlas_layout.clone(),
                            index: *index
                        };
                        // let img = UiImage::new(atlas_texture.clone());
                        return Some((atlas, atlas_texture.clone()));
                    }
                }
            }
        }
        None
    }
}

#[derive(Bundle)]
pub struct UiImageAtlas {
    atlas:  TextureAtlas,
    ui_img: UiImage
}

/// Атлас текстурных палетт (Наслоение на текстуру, как маска)
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

/// Атлас для хранения текстур объектов
#[derive(Resource, Default)]
pub struct ObjectAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

/// Атлас для хранения текстур эффектов
#[derive(Resource, Default)]
pub struct ParticleAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

// [Test]
/// Атлас для хранения тестовых и буферных текстур
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
/// Атлас для хранения текстур для сущностей (8-ми направленных движенияй (3на3 атласы))
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
