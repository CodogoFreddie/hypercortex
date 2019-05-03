ok, I'm going to abandon the hypercortex thing for now:

+ pin is the only other thing it uses, and that's better done as a set of shell scripts and a few text files 
+ all the other things I wanted are actually already handled with my personal server, what's really the point 
+ the whole hypercortex thing was probably feature creep from the begining 

## architecture

Apps will interface with the tasks through the engine, applying queries, running mutations and getting results

`Task`s will be simple structs that can respond with `bool` to `Queries` and update themselves with `Mutations`
