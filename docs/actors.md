# Actors

`Actors` are built from `ActorParams`, either pulled from JSON or constructed in code.
The `id` field of an `ActorParams` instance will be the prototypes id, if pulled from a file such as `assets/actors.json` or the instance id if pulled from a file such as a save game.
When instantiating an actor from a prototype it is therefore important to either set the `id` field of the `ActorParams` to `None` or to provide a unique id of your own, so that it does not instantiate with the prototype id as its uid.

More info to come
