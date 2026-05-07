use component_macro_derive::*;
use copper::Component;
use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::resource::Resources;
use copper::*;
use std::any::TypeId;



// ==============================================
// ===== Rör ingenting ovanför denna linje! =====
// ==============================================


// ==============================================
// Skapa Components här nedanför:

// - Steg 1: Skapa en "Position"-component som innehåller en variabel för X-koordinat samt en för Y-koordinat.
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

// ==============================================
// Skapa Systems här nedanför:

// - Steg 2.1: Definiera ett system som skapar 10 entiteter och lägger på en "Position"-component.
struct EntitySpawnSys;
impl System for EntitySpawnSys {
    components_read!();
    components_write!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        println!("EntitySpawnSys runnnig... ");

        for i in 1..10 {
            let entity_id = world.spawn();
            
            world.add_component(entity_id, Position { x: i, y: i});
        }
    }
}


// - Steg 3.1: Skapa ett nytt system som hämtar alla entities med en PositionComponent och flyttar på varje entity, varje update.
struct RepositionSys;
impl System for RepositionSys {
    components_read!();
    components_write!(Position);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        println!("RepositionSys runnnig... ");
        let entities = query!(self, world);

        for entity_id in entities {
            let mut position_comp = world.get_component_mut::<Position>(entity_id).unwrap();
            println!("Entity {}: X = {} | Y = {}", entity_id, position_comp.x, position_comp.y);

            position_comp.x *= 2;
            position_comp.y *= 2;

            println!("Entity {}: X = {} | Y = {}", entity_id, position_comp.x, position_comp.y);
            println!();
        }
    }
}



fn main() {
    println!("Welcome to Copper! \n");

    // Initierar motorn.
    let mut engine = Engine::new();

    
    // - Steg 2.2: Lägg till systemet i motorn.
    engine.add_system(Startup, EntitySpawnSys);


    // - Steg 3.2: Lägg till systemet i motorn.
    engine.add_system(Update, RepositionSys);
    

    // - Steg 4: Kör spelet. Förslagsvis lägg till några 'println!()' för att kontrollera vad som händer!
    // För att köra: § cargo run --bin your_game
    engine.run_cycles(1);
}