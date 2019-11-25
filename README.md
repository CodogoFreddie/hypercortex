## Releases

Each consumable binary should be at the same version, which should be the current released version, they should be released in lockstep so that an end user can be certain that they're using the correct version of each crate.
The internal crates can have their versions incremented one at a time.

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

Does it actually make more sense to have a local cache, and an `upstream` driver that can be used to propagate changes?
So each client has two components: one that handles updating the local cache, and one that syncronises the local cache with the `upstream`?

## Notes to Self

Having a client/daemon architecture actually does lead to a more stateless and fault-tolerant solution.
The client only ever reads and writes to the local file-system, which means it can always operate, regardless of network status or state consistency.
The daemon should be able to sync the local file-system in a conflict free way with other clients, and it is controlled by the local file-system state.

## Network Interchange Format v1.0

When performing a sync, the client will perform the following actions:

1. Request a map of ids -> hashes for every task the server is aware of
2. Generate its own id -> hash map for its own tasks
3. Compare hashmaps.
4. If there are no conflicts, stop syncing, otherwise, resolve conflicts for each task:
   1. There are three possible types of conflicts:
      1. Both have an id, but hashes conflict
      2. The client has a task that the server does not
      3. The server has a task that the client does not
         Whatever the case, we can follow the same logic:
   2. The client sends what it thinks the current state of a task is, or `null` if it lacks it
   3. The server resolves the conflict, updates its DB if nescisary, and responds with what it _now_ considers to be the correct state for the task
   4. The client receives this up-to-date correct state for the task, resolves any conflicts if nescisary, and updates its own DB
5. Onces every task has had its conflicts resolved, go to `1.` again.

### `GET /hashes`

#### Request

`empty`

#### Response

```json
{
  "pb67wndxge8293xd": "CONTENT_HASH",
  "px6zrs3z46b8e5pg": "CONTENT_HASH",
  "wbyzkrrrp7xb2rps": "CONTENT_HASH",
  "c6yx4mdzxm9cz7nm": "CONTENT_HASH",
  "dmnrx9gmdxhneq2x": "CONTENT_HASH",
  "tzthweareg9r6y5p": "CONTENT_HASH"
}
```

### `POST /task/:id`

### Request

```
{
   "client_state": null | {
      "id": "pb67wndxge8293xd",
      "description": "example description",
      "tags": [ "test" ]
   }
}
```

### Response

```
{
   "server_state": null | {
      "id": "pb67wndxge8293xd",
      "description": "example description",
      "tags": [ "test" ]
   }
}
```
