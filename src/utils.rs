use bevy::{
    math::Rect,
    prelude::{Commands, Component},
};
use bevy_ecs_ldtk::app::{LdtkEntityAppExt, LdtkIntCellAppExt};

use crate::sealed_trait;

pub trait SystemDefault {
    fn sdefault() -> Self;
}

pub fn spawn_default<T>(mut commands: Commands)
where
    T: Default + Component,
{
    commands.spawn(T::default());
}

pub fn spawn_sdefault<T>(mut commands: Commands)
where
    T: SystemDefault + Component,
{
    commands.spawn(T::sdefault());
}

sealed_trait! {
    pub trait RectExt {
        fn intersects(&self, other: Rect) -> bool;
    } =>
    impls Rect
}

sealed_trait! {
    pub trait LdtkAppTraitExt {
        fn register_ldtk_entity<B>(&mut self)
        -> &mut Self
        where B: bevy::prelude::Bundle + bevy_ecs_ldtk::prelude::LdtkEntity + crate::world::Entity;

        fn register_ldtk_int_cell<B>(&mut self)
        -> &mut Self
        where B: bevy::prelude::Bundle + bevy_ecs_ldtk::prelude::LdtkIntCell + crate::world::IntCell;
    } =>
    impls bevy::app::App
}

impl RectExt for Rect {
    fn intersects(&self, other: Rect) -> bool {
        !self.intersect(other).is_empty()
    }
}

impl LdtkAppTraitExt for bevy::app::App {
    fn register_ldtk_entity<B>(&mut self) -> &mut Self
    where
        B: bevy::prelude::Bundle + bevy_ecs_ldtk::prelude::LdtkEntity + crate::world::Entity,
    {
        <Self as LdtkEntityAppExt>::register_ldtk_entity::<B>(self, B::IDENTIFIER)
    }

    fn register_ldtk_int_cell<B>(&mut self) -> &mut Self
    where
        B: bevy::prelude::Bundle + bevy_ecs_ldtk::prelude::LdtkIntCell + crate::world::IntCell,
    {
        <Self as LdtkIntCellAppExt>::register_ldtk_int_cell::<B>(self, B::INTCELL_ID)
    }
}
