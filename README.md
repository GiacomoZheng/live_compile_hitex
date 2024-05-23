# live_compile_hitex

update `.tex` into `.hnt` by hilatex, whenever changes happen

Usage: live_compile_hitex [DIR]

Arguments:
  [DIR]  directory to watch [default: .]

## CHANGELOG

v1.0.1: Now it can detect the `ref.bib` and run `biber main` automatically.

## BUGs

* If you create a new `.tex` file, you need to rerun this program.

### BUGs to check

* The `output-dir` seems not work. I will check if it is my problem or `hilatex`'s problem.