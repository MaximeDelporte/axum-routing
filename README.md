# axum-routing

The goal of this project is to learn every basic concepts about axum.

The project contains the list of the following endpoints:

- **/** : Return a basic "Hello World" message
- **/mirror_body_string** : Mirror the body sent through the HTTP request.
- **/mirror_body_json** : Mirror the json sent through the HTTP request.
- **/path_variables/8** : Read the static path variable 8.
- **/path_variables/:id** : Read the dynamic path variable id.
- **/headers** : Read the default header's properties.
- **/mirror_custom_header** : Read the customs header's properties.
- **/middleware_message** : Return a message created inside mod.rs and transmitted inside middleware_message.rs

Cors middleware is added inside mod.rs thanks to tower-http crate.