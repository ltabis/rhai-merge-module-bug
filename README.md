# rhai-merge-module-bug
A reproductible bug when merging ASTs with rhai

## The bug

I discovered that creating an ast with `rhai::Engine::compile_scripts_with_scope`, followed by a merge of ast with another compiled with `rhai::Engine::compile_into_self_contained` seems to not keep embedded modules. (see `main.rs`)

## The test

I compile both ASTs and then evaluate the user's script.

## Another crash

While trying to reproduce the bug, I also discovered that rhai stack overflow when importing a module from the user (see `script3.rhai`) with this example, even though I use the "unchecked" feature.