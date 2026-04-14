mod ecs;
use ecs::entity::*;
use ecs::component::*;
use ecs::system::*;
use ecs::world::*;

struct System {
    component_read: String,
}

fn main() {

    let mut test_entity = Entity::new(1);

    println!("{:?}", test_entity);

    let mut test_component = Component::new();

    let prop1 = (String::from("Prop1"), PropertyData::Integer(50));

    let prop2 = (
        String::from("Prop2"),
        PropertyData::Text(String::from("Hejsan")),
    );

    let prop3 = (String::from("Prop3"), PropertyData::Flag(true));

    test_component.add_property(prop1);
    test_component.add_property(prop2);
    test_component.add_property(prop3);
    
    println!("{:?}", test_component);
    println!("{:?}", test_component.get_id());

    test_entity.add_component(test_component);


}
