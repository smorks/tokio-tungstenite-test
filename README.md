# tokio-tungstenite-test

this is a test app based on the example from the tokio-tungstenite crate.

## What's wrong?

when i try writing to the tx channel, the fwd_to_ws ends, and no messages are read.

also, if i don't write to the tx channel, the fwd_to_ws ends as soon as the the do_stuff thread is done.

if i add a delay, and don't write to tx, i can see Ping messages being read in, so it's just the writing that appears to be not working how i expect it to.

## Other Notes

I've also tested with the async-tungstenite crate, with the exact same results. will push those changes to a new branch.
