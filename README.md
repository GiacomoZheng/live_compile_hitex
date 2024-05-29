# live_compile_hitex

Update `.tex` into `.hnt` by `hilatex`, whenever changes happen. Do not forget to set the autoreload functionality of hintview.

Usage: live_compile_hitex [DIR]

Arguments:
  [DIR]  directory to watch [default: .]

## CHANGELOG

Version 1.0.2: Now it will compile everything, open the `.hnt` (mac only), and then start to watch the changes.
Version 1.0.1: Now it can detect the `ref.bib` and run `biber main` automatically.

## BUGs

* If you create a new `.tex` file, you need to rerun this program.
* The `hilatex` and `biber` should run once as soon as the program starts.

### BUGs to check

* The `output-dir` seems not work. I will check if it is my problem or `hilatex`'s problem.