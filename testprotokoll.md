# Testprotokoll
Testprotokoll för UX workshop, 5 maj.

## Förberedelser
Din uppgift är att skapa början till ett spel i Copper. Om du inte är bekant med Entity Component System, vänligen läs ECS.md. Sedan läs README.md

## Steg 1
Du är en spelutvecklare som vill testa Copper. Din plan är att göra början till ett spel. Spelet skall till en början generera 10 stycken entiteter. Dessa entiteter skall i framtiden representera någon slags soldat eller dylikt, så du börjar med att skapa en "health component" som innehåller en variabel med befintliga mängden health points.
  
__Uppgift:__ Definiera en "HealthComponent" som innehåller en variabel för befintlig HP.

## Steg 2
Nu har du en component och vill gå vidare med att ordna så att 10 entiteter (med varsin HealthComponent) skapas vid start av spelet.

__Uppgift:__ Definiera ett system som skapar 10 entiteter och lägger på en HealthComponent (med valfritt start-HP). (att detta sker vid start av spelet görs i senare steg)

## Steg 3
Nu har du dina entities med HealthComponents. För att vidare testa Copper vill du nu göra ett system som reducerar HP på varje entitet varje uppdatering (frame).

__Uppgift:__ Skapa ett system som hämtar alla entities med en HealthComponent och reducerar HP med ett varje update.

## Steg 4.
Nu inser du kanske att HP kommer reduceras tills det blir en negativ overflow (beroende på vilken datatyp som används, dock).

__Uppgift:__ Skapa en "Death" component. Uppdatera systemet som reducerar HP att registrera denna komponenten när HP når 0 för respektive entitet. Uppdatera systemets query så att det filterar bort entiteter som redan har en DeathComponent registrerad.
