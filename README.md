# rust-api-rest

Build a API REST

##### Language        : Rust
##### Routing         : Rocket
##### Database        : MONGODB

# CURLS

## Get All Users

curl -s -X GET -H "accept: application/json" -H "Content-Type: application/json" http://127.0.0.1:8000/v1/users/

## Get User by Id

curl -s -X GET -H "accept: application/json" -H "Content-Type: application/json" http://127.0.0.1:8000/v1/user/{id}


## Create User

curl -X POST http://127.0.0.1:8000/v1/user/ -H  "accept: application/json" -H  "Content-Type: application/json" -d '{"name":"Maria", "last_name":"Lopez","country":"Spain"}'

## Update User 

curl -X PUT http://127.0.0.1:8000/v1/user/{id} -H  "Content-Type: application/json" -d '{"name":"pepe","last_name":"dev","country":"Spain"}'

## Delete User

curl -H 'Accept: application/json' -X DELETE "http://127.0.0.1:8000/v1/user/{id}"
