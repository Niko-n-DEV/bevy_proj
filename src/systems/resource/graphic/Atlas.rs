#![allow(unused)]
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct AtlasRes {
    pub items:      AtlasData,
    pub material:   AtlasData,
    pub terrain:    AtlasData,
    pub objects:    AtlasData,
    pub con_obj:    AtlasData,
    pub particle:   AtlasData,
    pub entity:     AtlasData,
    pub test:       AtlasData,
    pub ui:         AtlasData,
}

#[derive(Resource, Clone, Default)]
pub struct AtlasData {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

#[derive(Debug, Clone)]
pub enum AtlasType {
    Items,
    Material,
    Terrain,
    Objects,
    ConnectObj,
    Particle,
    Entity,
    Test,
    Ui,
}

impl AtlasRes {
    pub fn init() -> Self {
        Self {
            items:      AtlasData::default(),
            material:   AtlasData::default(),
            terrain:    AtlasData::default(),
            objects:    AtlasData::default(),
            con_obj:    AtlasData::default(),
            particle:   AtlasData::default(),
            entity:     AtlasData::default(),
            test:       AtlasData::default(),
            ui:         AtlasData::default(),
        }
    }

    pub fn get_spritesheet(&self, atlas_type: AtlasType, name: &str) -> Option<SpriteSheetBundle> {
        let atlas_data = match atlas_type {
            AtlasType::Items        => &self.items,
            AtlasType::Material     => &self.material,
            AtlasType::Terrain      => &self.terrain,
            AtlasType::Objects      => &self.objects,
            AtlasType::ConnectObj   => &self.con_obj,
            AtlasType::Particle     => &self.particle,
            AtlasType::Entity       => &self.entity,
            AtlasType::Test         => &self.test,
            AtlasType::Ui           => &self.ui,
        };

        if let Some(index) = &atlas_data.ids {
            if let Some(index) = index.get(name) {
                let sprite = SpriteSheetBundle {
                    texture: atlas_data.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: atlas_data.layout.clone().unwrap(),
                        index: *index
                    },
                    ..default()
                };
                return Some(sprite);
            }
        }
        None
    }

    pub fn get_texture(&self, atlas_type: AtlasType, name: &str) -> Option<(TextureAtlas, Handle<Image>)> {
        let atlas_data = match atlas_type {
            AtlasType::Items        => &self.items,
            AtlasType::Material     => &self.material,
            AtlasType::Terrain      => &self.terrain,
            AtlasType::Objects      => &self.objects,
            AtlasType::ConnectObj   => &self.con_obj,
            AtlasType::Particle     => &self.particle,
            AtlasType::Entity       => &self.entity,
            AtlasType::Test         => &self.test,
            AtlasType::Ui           => &self.ui,
        };

        if let Some(index) = &atlas_data.ids {
            if let Some(index) = index.get(name) {
                if let Some(atlas_texture) = &atlas_data.image {
                    if let Some(atlas_layout) = &atlas_data.layout {
                        let atlas = TextureAtlas {
                            layout: atlas_layout.clone(),
                            index: *index
                        };
                        return Some((atlas, atlas_texture.clone()));
                    }
                }
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
                        return Some((atlas, atlas_texture.clone()));
                    }
                }
            }
        }
        None
    }
}

#[derive(Resource, Default)]
pub struct UiAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image:  Option<Handle<Image>>,
    pub ids:    Option<HashMap<String, usize>>,
}

impl UiAtlas {
    pub fn extruct_texture(&self, name: &str) -> Option<(TextureAtlas, Handle<Image>)> {
        if let Some(index) = &self.ids {
            if let Some(index) = index.get(name) {
                if let Some(atlas_texture) = &self.image {
                    if let Some(atlas_layout) = &self.layout {
                        let atlas = TextureAtlas {
                            layout: atlas_layout.clone(),
                            index: *index
                        };
                        return Some((atlas, atlas_texture.clone()));
                    }
                }
            }
        }
        None
    }
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

/// `Test`
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
