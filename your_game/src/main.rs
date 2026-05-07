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



// ==============================================
// Skapa Systems här nedanför:

// - Steg 2.1: Definiera ett system som skapar 10 entiteter och lägger på en "Position"-component.



// - Steg 3.1: Skapa ett nytt system som hämtar alla entities med en PositionComponent och flyttar på varje entity, varje update.



fn main() {
    println!("Welcome to Copper! \n");

    // Initierar motorn.
    let mut engine = Engine::new();

    
    // - Steg 2.2: Lägg till systemet i motorn.


    // - Steg 3.2: Lägg till systemet i motorn.
    

    // - Steg 4: Kör spelet. Förslagsvis lägg till några 'println!()' för att kontrollera vad som händer!
    // För att köra: § cargo run --bin your_game

}