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
- **/middleware_message** : Return a message created inside mod.rs and transmitted inside middleware_message.rs
- **/middleware_custom_header** : Return a message set by the request's header. The message is read by the read_middleware_custom_header function. This function is used inside a route_layer.
- **/always_error** : This endpoint returns a 418 error using the Result type.
- **/returns_201** : This endpoint returns the 201 status code alongside a message thanks to the Response type.
- **/get_json** : This endpoint returns a JSON created inside get_json.rs. We use serde to Serialize the JSON.
- **/validate_data** : Validate the user's password. We use serde to Deserialize the body request and to Serialize the entity as the response. If the password is not "Azerty123", then we return an error.
- **/custom_json_extractor** : Use of a custom extractor define inside custom_json_extractor.rs. We validate body request thanks to validator crate.

Cors middleware is added inside mod.rs thanks to tower-http crate.