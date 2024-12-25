// dacho/core/ecs/src/query.rs

use core::marker::PhantomData;

use super::{component::ComponentGroup, system::Argument};


pub struct Query<CG>
where
    CG: ComponentGroup
{
    pd: PhantomData<CG>
}

pub trait QueryMarker {}

impl<CG> QueryMarker for Query<CG>
where
    CG: ComponentGroup
{}

impl<CG> Argument for Query<CG>
where
    CG: ComponentGroup
{}

