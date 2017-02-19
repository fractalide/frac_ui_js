# UI_JS

## Project Description:

Server side rendering of HTML pages that are constructed using dataflow graphs.

## Problem:

No way to make a unified dataflow Ui library.

## Solution:

This is a highly experimental library to build out websites.

## Stability Status:

- [x] Raw
- [ ] Draft
- [ ] Stable
- [ ] Deprecated
- [ ] Legacy

## Build Instructions
Ensure you've installed [nix](https://nixos.org/nix).
```
$ export NIX_PATH+=:fractalide=https://github.com/fractalide/fractalide/archive/v20170218.tar.gz
$ git clone git://github.com/fractalide/fractal_ui_js.git
$ cd fractal_ui_js
$ nix-build --argstr rs test
```
