# live_compile_hitex

Update `.tex` into `.hnt` by `hilatex`, whenever changes happen. Do not forget to set the autoreload functionality of hintview.

Usage: live_compile_hitex [DIR]

Arguments:
  [DIR]  directory to watch [default: .]

## CHANGELOG

Version 1.0.3: Now if you want to create a new `.tex` file, you can enter the path into the command line, and it will create the file and thread for you (not perfect yet).

Version 1.0.2: Now it will compile everything, open the `.hnt` (mac only), and then start to watch the changes.

Version 1.0.1: Now it can detect the `ref.bib` and run `biber main` automatically.

## BUGs

To be found.

### BUGs to check

* The `output-dir` seems not work. I will check if it is my problem or `hilatex`'s problem.