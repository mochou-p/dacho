// dacho/core/ecs/src/query.rs

use core::marker::PhantomData;

use super::{
    component::{ComponentMask, ComponentGroup},
    system::Argument,
    world::World
};


pub struct Query<CG>
where
    CG: ComponentGroup
{
    world: *mut World,
    pd:         PhantomData<CG>
}

impl<CG> Query<CG>
where
    CG: ComponentGroup
{
    pub fn one(&self) -> &CG {
        // SAFETY: pointer always valid
        unsafe {
            &*((*self.world).components[&CG::mask()][0].1.as_ref()
                as *const dyn ComponentGroup
                as *const CG)
        }
    }
}

pub trait QueryMarker: Sized {
    fn validate();
    fn mask() -> ComponentMask;
    fn pes(world: *mut World) -> Self;
}

impl<CG> QueryMarker for Query<CG>
where
    CG: ComponentGroup
{
    #[inline]
    fn validate() {
        CG::validate();
    }

    #[inline]
    #[must_use]
    fn mask() -> ComponentMask {
        CG::mask()
    }

    #[inline]
    #[must_use]
    fn pes(world: *mut World) -> Self {
        Self { world, pd: PhantomData }
    }
}

impl<CG> Argument for Query<CG>
where
    CG: ComponentGroup
{}

