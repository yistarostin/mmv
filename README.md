# MMV
## Mass move files with 1 command using simple pattern-matching technique.
* Pattern must be a string with some `*` (star) symbols. Each of this characters will be replaced with a substings to make pattern a name of to be moved
* Template is a string with some `#` symbols followed by natural numbers. Is some file matched pattern, each of `#` things will be replaced with corresponding pattern substings to construct a new filename
* If you want `mmv` to overwrite existing files, you should provide a `-f` flag, otherwise `mmv` will do nothing.

## Here are a few examples on how `mmv` can be used:
* `mmv prefix*suffix new_prefix#1new_suffix` will move all files with `prefix` and `suffix` to same names, but with `new_prefix` and `new_suffix`.
* `mmv old new` will behave just as an usual `mv` command and rename a single file if such exists.
* `mmv a*c* a#1c` will trunk suffixes of all files matching `a*c*` template (meaning such files should start with `a` letter and containg `c` further).
* `mmv 1*3 a#1`. If current directory contains files like 123, 1113, 13, 143, they will be renamed to a2, a11, a, a4 respectively
## Build instructions

* use `cargo build --release` for release-ready binary
