
## About

Regression tests for the winit crate.

This is a work-in-progress.

The idea is that the executables under `tests-previous` and `tests-current` open a winit window and sythesize vairous input. While this happens, the winit window records the events into a file. We execute the same tests depending on different versions of winit (previous/current) and compare the output. There is a regression iff the output is different.

The `framework` crate is used to automate executing and comparing the `tests-previous` and `tests-current`.

The `tests-current` uses a symlink for the `src` folder found in `tests-previous`
