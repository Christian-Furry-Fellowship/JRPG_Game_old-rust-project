
Still under very heavy development. Not playable yet.

Data is loaded from campaigns/TestGame. Location does not matter, but you need a yaml file to describe each asset you want to load and use a relative path from the yaml file's location. Only type of asset supported as of writing this are sprite sheets. See campaigns/TestGame/sprite_sheets/sara.yml for an example.

Documentation coming soon.


# Compiling

You will need to Install rust, follow the instructions at the following URL to get it. https://www.rust-lang.org/tools/install.

Once installed download this repository, navigate to it in a terminal and run the following two commands to first fetch all dependencies then compile and run the program:

* cargo fetch
* cargo run --release

