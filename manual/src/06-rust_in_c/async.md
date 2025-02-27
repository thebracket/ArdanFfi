# Async!

A surprisingly large amount of cool stuff in Rust uses async. That's an argument we should probably avoid unless everyone's feeling fighty!

I recently worked on a project that wanted to use LanceDb (a vector embedding database) in C#. C# doesn't have the greatest FFI story, but it does have great async. Unfortunately, it's not the same async Rust uses - so you can't just magically call async functions. There really isn't a great story for creating bindings other than the C ABI, either. We're not going to look directly at that project (it's huge, but it's at https://github.com/thebracket/LanceDbCSharp/ if you're bored). But we can look at how it works - and how you can use it to bridge the FFI gap and still use async.

So there's an "external" set of functions:

* When you make a `Connection`, if the runtime isn't initialized it lazilly spawns a thread and launches Tokio on it.
    * It's fun to make sure that Tokio is ready, so a "oneshot" channel calls back to say "I'm here!".
    * We just hand out connection handles (id numbers) and keep the connections in a state table.
* When you try to perform a database operation with the connection:
    * You call a function in the `external` (C ABI).
    * The function puts everything you want to do into an enum.
        * This includes callbacks:
            * For function completion.
            * For passing results back to the client when they are available.
    * That enum is passed into Tokio as a channel call.
    * A big `match` statement spawns tokio tasks to handle the actual call.
    * Each task returns results by calling the provided callback. Rust owns the data - C# has to copy it (in this case that's unavoidable anyway, the format has to be marshalled).
    * When the function finishes, it replies on a `oneshot` to the calling function in `external` - which can then return.

It's not perfect, but it works---and you can use this pattern. Meta even use it with their async C++ and async Rust to bridge the divide between the two. There was a talk on that here last year.
