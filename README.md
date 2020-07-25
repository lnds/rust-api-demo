# rust-api-demo
This is a simple demo of a REST API using Actix 2.x.

In this demo we have 2 GET methods:

    / -> retrieve a random value
    
    /api/{id} -> retrieves the value asociated to id



The server loads a file ("data.csv") with a list of pairs (id, value) into a shared memory Hashmap and responds the values via the api requests.

The response is JSON:

    { id: <usize-value>, value: <u8-vale> }

The script gendata.pl in perl creates a masive file for testing.


# Demo de API en Actic

Esta es una demo muy sencilla usando Actix 2.x y Rust

Esta demo tenemos 2 métodos GET:

    / -> obtiene un valor al azar
        
    /api/{id} -> obtiene el valor asociado a id
    
El servidor carga un archivo ("data.csv") con una lista ed pares (id, value) en una variable compartida en memoria de tipo Hashmap y responde a través de los requests de la api.


La respuesta es un JSON con la siguiente estructura:

        { id: <usize-value>, value: <u8-vale> }

El script gentdata.pl en perl crea un archivo grande para efectos de testing.


    