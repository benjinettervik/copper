# COPPER Demo: 

## Demospel 

Även demospelet kör concurrently, däremot med en viss spärr för read-read conflicter då vi inte hann göra en refactor för detta.

För att köra demospelet kör:  

```make demo```

## Prestandatest och concurrencymätningar 

För att köra prestandatesten och concurrencymätningarna använd placeholderargumenten 1000, 10000 eller 100000 i följande kommandon:

```make performance_noninteractive_%```

```make performance_nonconcurrent_interactive_%```

```make performance_concurrent_interactive_%```
