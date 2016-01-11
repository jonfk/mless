#my-less
[![Build Status](https://travis-ci.org/jonfk/mless.svg)](https://travis-ci.org/jonfk/mless)

My implementation of the less command to learn the Rust programming language.

##TODO
- more navigation
- searching
- command line flags:
  - -n  -N  ....  --line-numbers  --LINE-NUMBERS
  - -S  ........  --chop-long-lines
                    Chop (truncate) long lines rather than wrapping.
- Added better error messages on common errors:
  - opening a directory
  - file not found
- support more than one file
- profile memory footprint for large file support