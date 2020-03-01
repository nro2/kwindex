Nick Robinson
CS 410P Rust Programming Winter 2020
2/4/2020
HW 3

Things definitely got a little more challenging this week.  I struggled with even figuring out that my `impl <'a>KWIndex<'a>` needed the `<'a>` before AND after
KWIndex for over an hour.  As soon as I figured that out, the rest of the assignment was fairly straight forward with the help of clippy and the compiler.
After figuring out the impl, easily the next hardest thing to implement was the `extend_from_text()` function.  My solution ended up working, but I don't know
if there is a more efficient way (probably!) to do it.  The `trim_matches()` function took some time to get working, but once it did, my life was a lot easier.
The other functions were pretty easy to implement after the first couple of feats, so nothing to report there.

I tried something different this time, in that I wrote my code almost entirely in the Rust Playground and then copied it over to my IDE.  I found that it was
a little easier to do it that way, so I will probably do that from now on.  I like the one click instant feedback instead of having to go to my terminal and 
enter in cargo clippy or cargo run.

To test my library, I just tested each individual function except for the `extend_front_text()`.  I tried to do this originally with matching my desired
output, but found that I was not able to do that since the vector within the struct was not explicitly declared public.  My other tests were straight forward, and
were easy to port over since I was writing them while I was working on the actual code to make sure my output was correct.

Fun assignment, much more challenging than the last two!