## Possible Storage Backends

I could create a more generalised _"driver"_ system for storage of tasks
Each client could connect to any number of _drivers_ to store the tasks.
This would mean that I'd need to bake reconcilation into the hypertask engine somehow.
This is a cool idea, but almost definitly one I want to do after v0.2 has shipped

- Plain file system
  - Simple
  - Integrates well with Dropbox, etc
  - Would need a custom server to share with a web client
  - No sharing mechanism built in
- Git
  - Can slot on top of file system
  - Possible to use with web client, if hosted properly
  - Not really purpose built for this
- Hyperdrive
  - Not stable
  - Or rusty
  - Has sharing built in
    - But only in a deamon-y way
    - would need a static service
  - Couldn't work with HTTP web client
- Mongo
  - nice simple DB interface
  - would allow for a hosted db to store your tasks
  - equal access for cli and web client
  - SQL databases probably wouldn't work, I want to use a schema-less system so the schema is defined by the client
