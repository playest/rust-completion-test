This repository is an example of an auto completion that doesn't work in any IDE.

In main.rs t.<ctrl + space> should get_attribute (from trait Device obtained via #derive). Currently, it doesn't. I tested with vscode + RLS, vscode + rust-analyzer, intelij rust

What's weird is that is lists .clone which is also obtained via #derive