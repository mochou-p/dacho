// dacho/core/ecs/src/query.rs

use {
    core::{any::{Any, TypeId}, marker::PhantomData},
    alloc::collections::BTreeSet,
    std::collections::HashMap
};

use super::{entity::Entity, world::World, MESH_TI};

use dacho_mesh_c::MeshComponent;


pub struct Query<T> {
    world: *mut World, // could be a Weak<RefCell<World>>, but idk maybe Arc<Mutex/RwLock<World>> when multithreading
    pd:         PhantomData<T>
}

// tl;dr: there is currently no BC for using Query::*mut*,
//        allowing multiple mutable references, therefore wrong usage = UB
//        (be careful to not work on the same mutable references)
//
// right now the system arg is `&(Query,)`
// changing it to `&(&mut Query,)` would bring the BC back by having `&mut self` in `Query::*mut*`
// but first_mut and iter_mut return owned tuples of mutable references to components, e.g. (&mut T,)
// since for these 2 functions there is no BC (thanks to no '&' on the outside)
// a fix could be expecting a `Fn(&mut T::MutRef)`, but when testing this i ran into SIGSEGV
//
// TODO: revisit and refactor into safe functions
#[expect(clippy::mut_from_ref, reason = "read the comment above")]
impl<T> Query<T>
where
    T: QueryT
{
    #[must_use]
    pub fn first<'component>(&self) -> T::Ref<'component> {
        // SAFETY: raw pointer dereference inside
        unsafe { T::first(self.world) }
    }

    /// # Safety
    ///
    /// Multiple calls to this, or other `Query::*mut*` that return a mutable reference to the same data are possible,
    /// because there is currently no borrow checker, therefore wrong usage will result in undefined behaviour.
    /// Make sure that any previous `mut`s to the same data are dropped
    #[must_use]
    pub unsafe fn first_mut<'component>(&self) -> T::RefMut<'component> {
        // SAFETY: raw pointer dereference inside
        unsafe { T::first_mut(self.world) }
    }

    #[must_use]
    pub fn first_entity(&self) -> &Entity {
        // SAFETY: raw pointer dereference inside
        unsafe { T::first_entity(self.world) }
    }

    /// # Safety
    ///
    /// Multiple calls to this, or other `Query::*mut*` that return a mutable reference to the same data are possible,
    /// because there is currently no borrow checker, therefore wrong usage will result in undefined behaviour.
    /// Make sure that any previous `mut`s to the same data are dropped
    #[must_use]
    pub unsafe fn first_mut_entity(&self) -> &mut Entity {
        // SAFETY: raw pointer dereference inside
        unsafe { T::first_mut_entity(self.world) }
    }

    pub fn iter<'component>(&self) -> impl Iterator<Item = T::Ref<'component>> {
        // SAFETY: raw pointer dereference inside
        unsafe { T::iter(self.world) }
    }

    /// # Safety
    ///
    /// Multiple calls to this, or other `Query::*mut*` that return a mutable reference to the same data are possible,
    /// because there is currently no borrow checker, therefore wrong usage will result in undefined behaviour.
    /// Make sure that any previous `mut`s to the same data are dropped
    pub unsafe fn iter_mut<'component>(&self) -> impl Iterator<Item = T::RefMut<'component>> {
        // SAFETY: raw pointer dereference inside
        unsafe { T::iter_mut(self.world) }
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = &Entity> {
        // SAFETY: raw pointer dereference inside
        unsafe { T::iter_entities(self.world) }
    }

    /// # Safety
    ///
    /// Multiple calls to this, or other `Query::*mut*` that return a mutable reference to the same data are possible,
    /// because there is currently no borrow checker, therefore wrong usage will result in undefined behaviour.
    /// Make sure that any previous `mut`s to the same data are dropped
    pub unsafe fn iter_mut_entities(&self) -> impl Iterator<Item = &mut Entity> {
        // SAFETY: raw pointer dereference inside
        unsafe { T::iter_mut_entities(self.world) }
    }
}

pub trait QueryT {
    type Ref   <'component>;
    type RefMut<'component>;

    fn       get_set()      -> BTreeSet<TypeId>;
    fn to_components(self)  ->  HashMap<TypeId, Vec<Box<dyn Any>>>;

    fn get_meshes(&mut self, mesh_id_counter: &mut HashMap<u32, u32>) -> Vec<(u32, [f32; 16])>;

    /// # Safety
    ///
    /// Dereferences a raw pointer to `World`. It is pinned in `Application`,
    /// and systems are only called from `Application`, so the pointer remains valid
    unsafe fn first<'component>(world: *mut World) -> Self::Ref<'component>;

    /// # Safety
    ///
    /// Dereferences a raw pointer to `World`. It is pinned in `Application`,
    /// and systems are only called from `Application`, so the pointer remains valid
    unsafe fn first_mut<'component>(world: *mut World) -> Self::RefMut<'component>;

    /// # Safety
    ///
    /// Dereferences a raw pointer to `World`. It is pinned in `Application`,
    /// and systems are only called from `Application`, so the pointer remains valid
    unsafe fn first_entity<'entity>(world: *mut World) -> &'entity Entity;

    /// # Safety
    ///
    /// Dereferences a raw pointer to `World`. It is pinned in `Application`,
    /// and systems are only called from `Application`, so the pointer remains valid
    unsafe fn first_mut_entity<'entity>(world: *mut World) -> &'entity mut Entity;

    /// # Safety
    ///
    /// Dereferences a raw pointer to `World`. It is pinned in `Application`,
    /// and systems are only called from `Application`, so the pointer remains valid
    unsafe fn iter<'component>(world: *mut World) -> impl Iterator<Item = Self::Ref<'component>>;

    /// # Safety
    ///
    /// Dereferences a raw pointer to `World`. It is pinned in `Application`,
    /// and systems are only called from `Application`, so the pointer remains valid
    unsafe fn iter_mut<'component>(world: *mut World) -> impl Iterator<Item = Self::RefMut<'component>>;

    /// # Safety
    ///
    /// Dereferences a raw pointer to `World`. It is pinned in `Application`,
    /// and systems are only called from `Application`, so the pointer remains valid
    unsafe fn iter_entities<'entity>(world: *mut World) -> impl Iterator<Item = &'entity Entity>;

    /// # Safety
    ///
    /// Dereferences a raw pointer to `World`. It is pinned in `Application`,
    /// and systems are only called from `Application`, so the pointer remains valid
    unsafe fn iter_mut_entities<'entity>(world: *mut World) -> impl Iterator<Item = &'entity mut Entity>;
}

macro_rules! impl_query_t {
    ($l:tt, $($i:tt $ty:tt),+) => {
        #[expect(clippy::allow_attributes, reason = "to silence intended unused parentheses")]
        impl<$($ty),+> QueryT for ($($ty,)+)
        where
            $($ty: 'static),+
        {
            #[allow(unused_parens, reason = "',' after ')' so (T,) makes T")]
            type Ref   <'component> = ($(&'component     $ty),+);
            #[allow(unused_parens, reason = "',' after ')' so (T,) makes T")]
            type RefMut<'component> = ($(&'component mut $ty),+);

            fn get_set() -> BTreeSet<TypeId> {
                BTreeSet::from([
                    $(TypeId::of::<$ty>()),+
                ])
            }

            fn to_components(self) -> HashMap<TypeId, Vec<Box<dyn Any>>> {
                let mut map = HashMap::new();

                $(
                    map.entry(TypeId::of::<$ty>())
                        .or_insert(Vec::with_capacity(1))
                        .push(Box::new(self.$i) as Box<dyn Any>);
                )+

                map
            }

            // could be just a ::CONST when Component is a Trait, made with a proc-macro
            fn get_meshes(&mut self, mesh_id_counter: &mut HashMap<u32, u32>) -> Vec<(u32, [f32; 16])> {
                let mut vec = Vec::with_capacity($l);

                $(
                    if TypeId::of::<$ty>() == MESH_TI {
                        let mesh = (&mut self.$i as &mut dyn Any).downcast_mut::<MeshComponent>().unwrap();

                        mesh.nth = *mesh_id_counter.entry(mesh.id)
                            .and_modify(|count| *count += 1)
                            .or_insert(0);

                        vec.push((mesh.id, mesh.model_matrix.to_cols_array()));
                    }
                )+

                vec
            }

            unsafe fn first<'component>(world: *mut World) -> Self::Ref<'component> {
                let entity = Self::first_entity(world);

                ($(
                    entity.components
                        [&TypeId::of::<$ty>()]
                        [0]
                        .downcast_ref::<$ty>()
                        .unwrap()
                ),+)
            }

            unsafe fn first_mut<'component>(world: *mut World) -> Self::RefMut<'component> {
                let entity = Self::first_mut_entity(world);

                ($(
                    &mut *(
                        entity.components
                            .get_mut(&TypeId::of::<$ty>())
                            .unwrap()
                            [0]
                            .downcast_mut::<$ty>()
                            .unwrap()
                        as *mut _
                    )
                ),+)
            }

            unsafe fn first_entity<'entity>(world: *mut World) -> &'entity Entity {
                (*world).first_match(Self::get_set())
            }

            unsafe fn first_mut_entity<'entity>(world: *mut World) -> &'entity mut Entity {
                (*world).first_mut_match(Self::get_set())
            }

            unsafe fn iter<'component>(world: *mut World) -> impl Iterator<Item = Self::Ref<'component>> {
                Self::iter_entities(world)
                    .map(|entity|
                        ($(
                            entity.components
                                [&TypeId::of::<$ty>()]
                                [0]
                                .downcast_ref::<$ty>()
                                .unwrap()
                        ),+)
                    )
            }

            unsafe fn iter_mut<'component>(world: *mut World) -> impl Iterator<Item = Self::RefMut<'component>> {
                Self::iter_mut_entities(world)
                    .map(|entity|
                        ($(
                            &mut *(
                                entity.components
                                    .get_mut(&TypeId::of::<$ty>())
                                    .unwrap()
                                    [0]
                                    .downcast_mut::<$ty>()
                                    .unwrap()
                                as *mut _
                            )
                        ),+)
                    )
            }

            unsafe fn iter_entities<'entity>(world: *mut World) -> impl Iterator<Item = &'entity Entity> {
                (*world).matches_iter(Self::get_set())
            }

            unsafe fn iter_mut_entities<'entity>(world: *mut World) -> impl Iterator<Item = &'entity mut Entity> {
                (*world).matches_iter_mut(Self::get_set())
            }
        }
    }
}

impl_query_t!( 1, 0 A);
impl_query_t!( 2, 0 A, 1 B);
impl_query_t!( 3, 0 A, 1 B, 2 C);
impl_query_t!( 4, 0 A, 1 B, 2 C, 3 D);
impl_query_t!( 5, 0 A, 1 B, 2 C, 3 D, 4 E);
impl_query_t!( 6, 0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
impl_query_t!( 7, 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
impl_query_t!( 8, 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
impl_query_t!( 9, 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
impl_query_t!(10, 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
impl_query_t!(11, 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
impl_query_t!(12, 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);

pub trait QueryTuple {
    fn          new(world: *mut World) -> Self;
    fn get_all_sets()                  -> Vec<(BTreeSet<TypeId>, u32)>;
}

macro_rules! impl_query_tuple {
    ($($ty:tt),+) => {
        impl<$($ty),+> QueryTuple for ($(Query<$ty>,)+)
        where
            $($ty: QueryT),+
        {
            fn new(world: *mut World) -> Self {
                ($(Query::<$ty> { world, pd: PhantomData },)+)
            }

            fn get_all_sets() -> Vec<(BTreeSet<TypeId>, u32)> {
                vec![$(($ty::get_set(), 0)),+]
            }
        }
    }
}

impl_query_tuple!(A);
impl_query_tuple!(A, B);
impl_query_tuple!(A, B, C);
impl_query_tuple!(A, B, C, D);
impl_query_tuple!(A, B, C, D, E);
impl_query_tuple!(A, B, C, D, E, F);
impl_query_tuple!(A, B, C, D, E, F, G);
impl_query_tuple!(A, B, C, D, E, F, G, H);
impl_query_tuple!(A, B, C, D, E, F, G, H, I);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

