> running mutations requires us to know the queries and filter status
> calculating the score or filter requires us to know the tasks dependants and depends_on
> depends_on is inline, and can be calculated once we've loaded all the tasks
> dependants must be collected from all the tasks

To achieve the required opperations, it's becoming increasingly clear that there's not really any point having the engine draw in tasks through an itterator like it currently does. In fact, the whole context thing is becoming more cumbersome by the day. Here's a better idea:

```rust
fn run_engine(
   command: Command,
   tasks: HashSet<Rc<Id>, Rc<Task>>,
   score_machine: StackMachine,
   filter_machine: StackMachine,
   now: DateTime<Utc>,
) -> EngineOutcome {
   write_tasks: Vec<Rc<Task>>,
   display_tasks: Vec<(Score, Rc<Task>)>,
}
```

remove `Create` altogether, so that the `Id` has to be generated outside the engine, and have the engine just gracefully handle the case where the `Id` doesn't exist yet
