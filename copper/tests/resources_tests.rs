#[cfg(test)]
mod resources_basic_type_tests{
    // import test components
    use::copper::*;
    use copper::engine::Engine;
    use std::any::{TypeId,Any};
    use std::collections::HashMap;
    use copper::engine::world::*;

    pub struct ResourcesMock{
        resources: HashMap<TypeId, Box<dyn Any>>,
        // key becomes the type of the resource that has been saved , and the box contains on 
        // heap the accessible data associated.
    }
    pub struct TestRenderer;
    #[test]
    pub fn box_example(){
    // Box<T> is a smart pointer -- stores variable on heap instead of directly on stack 
    // dyn Any , unknown size at compile time 
        let mut x = Box::new(5);
        *x += 1;println!("{}", x);
        // x is a box, need dereferencing
        assert_eq!(*x,6);
    }

    #[test]
    pub fn typeid_example(){
        // typeid
        // type fingerprint
        let a = TypeId::of::<u32>(); //type of u32
        let b = TypeId::of::<String>(); //type of string
        assert_ne!(a,b);
    }
    #[test]
    pub fn hash_map_example(){
        // key is type id
        // value stored is a smart pointer to a struct of any type
        let mut resources: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
        // with this we've defined
        let mock_resources: TestRenderer = TestRenderer{}; 
        // hashmap insert
        resources.insert(
        TypeId::of::<TestRenderer>(),
            Box::new(mock_resources),
        );
        // hashmap get
        let mock = resources.get(&TypeId::of::<TestRenderer>()).unwrap();
        // unwrap needed due to an option, needs to handle found and not found safely. 
        let renderer = mock.downcast_ref::<TestRenderer>();
        // now it is Box<dyn Any> -- needs to recover original type
        // checks if the value inside the dyn Any is actually type T, in this case TestRenderer
        assert!(renderer.is_some());
    }

}


#[cfg(test)]
mod resources_refactor_tests{

    use::copper::*;
    use copper::engine::Engine;
    use std::any::{TypeId,Any};
    use std::collections::HashMap;
    use copper::engine::world::*;
    use crate::resources_refactor_tests::resource::camera::Camera2D;
    use crate::resources_refactor_tests::renderer::test_components_renderer::{TextureAsset,TextureHandle};
    use crate::resources_refactor_tests::resource::{RenderQueue,RenderCommand};
    use crate::resources_refactor_tests::grid::Grid;

    pub struct ResourcesMock{
        resources: HashMap<TypeId, Box<dyn Any>>,
    }
    impl ResourcesMock{
        pub fn new() -> Self {

            Self{
                resources: HashMap::new(),
            }

        }

        pub fn insert<T:Any>(&mut self, value: T){
            self.resources.insert(TypeId::of::<T>(),Box::new(value));
        }

        pub fn get<T: Any>(&self) -> Option<&T>
        {
            self.resources
                .get(&TypeId::of::<T>())
                .and_then(|v| v.downcast_ref::<T>())
        }
        pub fn get_mut<T: Any>(&mut self) -> Option<&mut T>
        {
            self.resources
                .get_mut(&TypeId::of::<T>())
                .and_then(|v| v.downcast_mut::<T>())
        }


    }
    pub struct TestRenderer;

    #[test]
    pub fn resources_refactor_test(){
        let mut resources = ResourcesMock::new();
        resources.insert(RenderQueue{commands: Vec::new(),});
        resources.insert(TextureAsset{textures: HashMap::new(),});
        resources.insert(Camera2D::new());
        resources.insert(Grid::new(32,32,16.0));

        resources
        .get_mut::<RenderQueue>()
            .unwrap()
            .commands
            .push(RenderCommand{texture:TextureHandle(1),x:2.0,y:3.0});

        let render_commands = &resources.get::<RenderQueue>().unwrap().commands;
        assert_eq!(render_commands[0].x,2.0);
    }

}
