# Testprotokoll
Testprotokoll för UX workshop, 5 maj.

## Förberedelser
Din uppgift är att skapa början till ett spel i Copper. Om du inte är bekant med Entity Component System, vänligen läs ECS.md. Sedan läs README.md för att se hur Copper används.

## Steg 1
Du är en spelutvecklare som vill testa Copper. Din plan är att göra början till ett spel. Spelet skall till en början generera 10 stycken entiteter. Dessa entiteter skall i framtiden representera något objekt som kan röra sig runt i en värld, så du börjar med att skapa en "position" component som innehåller en variabel för X-koordinat samt en för Y-koordinat.

__Uppgift:__ Definiera en "Position" component som innehåller koordinatdata.

## Steg 2
Nu har du en component och vill gå vidare med att ordna så att 10 entiteter (med varsin PositionComponent) skapas vid start av spelet.

__Uppgift:__ Definiera ett system som skapar 10 entiteter och lägger på en PositionComponent. (At detta sker vid start av spelet görs i senare steg.)

## Steg 3
Nu har du dina entities med PositionComponent. För att vidare testa Copper vill du nu göra ett system som flyttar varje på varje entity vid varje update(frame). Entiteterna kan flyttas på valfritt sätt.

__Uppgift:__ Skapa ett system som hämtar alla entities med en PositionComponent och flyttar på varje entity, varje update.
