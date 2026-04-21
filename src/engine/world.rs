
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use crate::engine::system::Entity;
use std::collections::HashSet;
use crate::engine::meta::SystemMeta;


// queryparam implemented, not queryparam MUT
macro_rules! impl_query_param {
    ($($name:ident),*) => {
        impl<$($name: QueryParam),*> QueryParam for ($($name,)*) {

            type Item<'w> = ($($name::Item<'w>,)*);

            fn get<'w>(world: &'w World, entity: Entity)
                -> Option<Self::Item<'w>>
            {
                Some(($($name::get(world, entity)?,)*))
            }

            fn meta(meta: &mut SystemMeta) {
                $($name::meta(meta);)*
            }
        }
    };
}

pub trait QueryParam {
    type Item<'w>;
    fn get<'w>(world: &'w World, entity: Entity) -> Option<Self::Item<'w>>;
    fn meta(meta: &mut SystemMeta);
}

impl<T: 'static> QueryParam for &T {
    type Item<'w> = &'w T;

    fn get<'w>(world: &'w World, entity: Entity) -> Option<Self::Item<'w>> {
        world.component_storages
            .get(&TypeId::of::<T>())
            .and_then(|s| s.downcast_ref::<HashMap<Entity, T>>())
            .and_then(|map| map.get(&entity))
    }
    fn meta(meta: &mut SystemMeta) {
        meta.reads.insert(TypeId::of::<T>());
    }
}

// need raw pointers 
impl<T: 'static> QueryParam for &mut T {
    type Item<'w> = &'w mut T;

    fn get<'w>(world: &'w World, entity: Entity) -> Option<Self::Item<'w>> {
 
        let storage_ptr = world.component_storages
            .get(&TypeId::of::<T>())
            .and_then(|s| s.downcast_ref::<HashMap<Entity, T>>())
            .map(|s| s as *const HashMap<Entity, T> as *mut HashMap<Entity, T>);

        if let Some(ptr) = storage_ptr {
            let storage = unsafe { &mut *ptr };
            storage.get_mut(&entity)
        } else {
            None
        }
    }
    fn meta(meta: &mut SystemMeta) {
        meta.writes.insert(TypeId::of::<T>());
    }
}


impl_query_param!(A);
impl_query_param!(A, B);
impl_query_param!(A, B, C);







use std::marker::PhantomData;

pub struct Query<'w, Q> {
    world: &'w World,
    _marker: PhantomData<Q>,
}

pub struct QueryMut<'w, Q> {
    world: &'w mut World,
    _marker: PhantomData<Q>,
}

impl<'w, Q> Query<'w, Q> {
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }
}
impl<'w, Q> QueryMut<'w, Q> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }
}

impl<'w, A> Query<'w, (A,)>
where
    A: QueryParam,
{
    pub fn iter(&self) -> Vec<(Entity, A::Item<'w>)> {
        let mut result = Vec::new();

        for entity in 0..self.world.next_entity_id {
            if let Some(item) = A::get(self.world, entity) {
                result.push((entity, item));
            }
        }

        result
    }
}

impl<'w, A, B> Query<'w, (A, B)>
where
    A: QueryParam,
    B: QueryParam,
{
    pub fn iter(&self) -> Vec<(Entity, (A::Item<'w>, B::Item<'w>))> {
        let mut result = Vec::new();

        for entity in 0..self.world.next_entity_id {
            if let Some(a) = A::get(self.world, entity) {
                if let Some(b) = B::get(self.world, entity) {
                    result.push((entity, (a, b)));
                }
            }
        }

        result
    }
}

impl<'w, A, B, C> Query<'w, (A, B, C)>
where
    A: QueryParam,
    B: QueryParam,
    C: QueryParam,
{
    pub fn iter(&self) -> Vec<(Entity, (A::Item<'w>, B::Item<'w>, C::Item<'w>))> {
        let mut result = Vec::new();

        for entity in 0..self.world.next_entity_id {
            if let Some(a) = A::get(self.world, entity) {
                if let Some(b) = B::get(self.world, entity) {
                    if let Some(c) = C::get(self.world, entity) {
                        result.push((entity, (a, b, c)));
                    }
                }
            }
        }

        result
    }
}









pub struct World {
    next_entity_id: usize,
    component_storages: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            component_storages: HashMap::<TypeId, Box<dyn Any>>::new(),
        }
    }

    pub fn spawn(&mut self) -> usize {
        let return_id = self.next_entity_id;
        self.next_entity_id += 1;

        // TODO: Might want to save which entities are currently alive if we want to be able to delete entities later 

        return_id
    }

    /// Adds a component onto an entity. Adds any new components into the world's storage. 
    pub fn add_component<T: 'static>(&mut self, entity_id: usize, component: T) {
        // Generates a unique ID based on a 'static component struct type. 
        // Example: every unique component of type 'struct Player' will generate the same component_id!
        let component_id = TypeId::of::<T>();

        // Gets the entry with the right component id.
        // If an entry does not exist, create one for component_id.
        let component_storage_box = self.component_storages
                .entry(component_id)
                .or_insert_with(|| Box::new(HashMap::<usize, T>::new()));
        
        // 'component_storage' in this case is a hashmap that stores which entities who has the 
        // 'component_storage' specific component assigned to it and:
        // - KEY is the unique ID of an entity.
        // - VALUE is the unique component data storage for that entity

        // Downcast component_storage_box
        let component_storage = component_storage_box.downcast_mut::<HashMap<usize, T>>()
            .expect("FATAL ERROR: This should not happen... ");

        component_storage.insert(entity_id, component);
    }


    /// Gets a reference to a component that is assigned to entity_id
    pub fn get_component<T: 'static>(&self, entity_id: usize) -> Option<&T>{
        self.component_storages.get(&TypeId::of::<T>())?
        .downcast_ref::<HashMap<usize, T>>()?
        .get(&entity_id)
    }

    /// Gets a mutable reference to a component that is assigned to entity_id
    pub fn get_component_mut<T: 'static>(&mut self, entity_id: usize) -> Option<&mut T> {
        self.component_storages.get_mut(&TypeId::of::<T>())?
        .downcast_mut::<HashMap<usize, T>>()?
        .get_mut(&entity_id)
    }



    pub fn query<Q>(&self) -> Query<Q>
    where
        Q: QueryParam,
    {
        Query::new(self)
    }

    pub fn query_mut<Q>(&mut self) -> QueryMut<Q>
    where
        Q: QueryParam,
    {
        QueryMut::new(self)
    }

}
// pub struct Query<A,B> {}

