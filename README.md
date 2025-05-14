# Eat Keys

Nom!

There's two programs here.

`cargo run --bin keybd` - Run to display keyboard key presses for ten seconds.

`cargo run --bin mouse` - Run to display mouse button presses for ten seconds.

The purpose of these programs are to help with debugging a remap tool for Ori speedruns.

Note if you use something like AHK or X-Mouse,
**it matters what order you launch things**. To see what Ori sees:

1. Close out of all remap software (not including driver-level software like Razer Synapse)

2. Run this software

3. Start up the other remap software

If you're curious why, it's because Windows hooks apply in the reverse order they are registered. Both X-Mouse and this software use hooks. AHK also uses hooks but only [sometimes](https://www.autohotkey.com/boards/viewtopic.php?style=23&f=96&t=127074). Note AHK may repeatedly unregister and re-register hooks.
