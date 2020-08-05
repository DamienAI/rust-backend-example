# Actix-web based rust backend example

This repository contains source code and the docker files in order to build and run a backend written in Rust and using mongodb.

## Building the code

There is a multi-stage dockerfile in order to build the backend and get a deployment ready image.

The first stage does the actual source code compilation using the official rust compiler image.

Since we do not want to deploy a huge image with the compilers, the different uncompiled libraries or even the source code there is a second stage which install the backend binary in a slim debian buster image.

## Running the backend with mongo

A docker-compose file is provided which will download and run both the backend and mongo. The Mongo URI is provided to the backend.

In order to build and run the images enter the following command in a terminal:
``` docker-compose build && docker-compose up ```
 
## Provided endpoints

Some simple endpoints are available:
 - [GET] localhost:4000/healthcheck
 - [POST] localhost:4000/api/persons
 - [GET] localhost:4000/api/persons/:person_id

## Security notes
  
Even if some security features are enabled (like cross-origin, max-age...) there are still some headers that should be set. Also there is no auth mechanism in place.

## TODO

- [ ] Tests

- [ ] More http headers enforcement, based on helmet js list

- [ ] Review the serialization of the Object ID

- [ ] OAuth2 auth support

- [ ] Connect to mongo using https

- [ ] Swagger UI serving ?
