ok, I'm going to abandon the hypercortex thing for now:

+ pin is the only other thing it uses, and that's better done as a set of shell scripts and a few text files 
+ all the other things I wanted are actually already handled with my personal server, what's really the point 
+ the whole hypercortex thing was probably feature creep from the begining 

## architecture

Apps will interface with the tasks through the engine, applying queries, running mutations and getting results

`Task`s will be simple structs that can respond with `bool` to `Queries` and update themselves with `Mutations`

## Notes
The Engine needs to be aware of every task so that it can statelessly defined the minimum needed id prefix to uniquly identify a `Task`. For this reason there's no real reason to implement it as a full iterator. it can still output an itterator once it's done though...

The uniq id prefix thing is a platform specific issue, it should be handled by the cli, not the engine. The engine doesn't need to produce a vector because of IDs, it need to produce it because of sorting
