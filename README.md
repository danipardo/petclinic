# Axum Petclinic 

This project is a port of Spring Framework's Petclinic (https://spring-petclinic.github.io/)  to Rust and Axum, which is a simple CRUD webapp where you can keep a list of Pets and Veterinarians.



[screenshot-home]: screenshot.png

![screenshot-home]

## Features 

This project is aimed at showcasing how you could do:

* Cookie based server-side sessions
* Using Redis as a session storage mechanism
* Form based Authentication
* Integration with Tera templates for rendering HTML
* Separate DEV/QA/PROD configurations
* Live reloading of Tera templates in Dev 
* Integration of session data with Tera templates
* Database access using Rbatis

## Build

 There is a schema creation script for Mysql in res/schema.sql
 
 Modify src/lib.rs to specify credentials for Mysql and Redis settings.
 
 run with
 
 ```
 $ cargo run 
 ```
 
 or 
 
 ```
 $ cargo run -- --env dev
 ```

Open the url http://localhost:3000 where you can login with username *admin*, and password *admin*.
