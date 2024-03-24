#![allow(unused)]
use bevy::prelude::*;

#[derive(Component, Resource)]
pub struct ItemsAtlas;

#[derive(Component, Resource)]
pub struct MaterialAtlas;

#[derive(Component, Resource)]
pub struct BlockAtlas;

#[derive(Component, Resource)]
pub struct VehicleAtlas;

#[derive(Component, Resource)]
pub struct ObjectAtlas;



// [Test]
#[derive(Resource)]
pub struct TestTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

impl Default for TestTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

/* 
    Реализовать тут основные компоненты атласов, а точнее определение их и индексирование внутренних элементов, 
    чтобы в дальнейшем по id/имени к ним можно было обратиться, и применить их к объекту.
    Так же, нужно реализовать индексацию анимированных элементов, путём запоминания их с общей таблицы в отдельные, но с друг-другом взаимосвязанными
*/