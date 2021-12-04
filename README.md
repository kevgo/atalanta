# run

Conveniently runs activities in several automation systems.

```
Usage: 
run [options] <task name>
runp <task name>   # shortcut for run -p
```

- a `Makefile` with a recipe of the given name exists --> run it and exit
- a `package.json` file with a script of the given name exists --> run it


### Options

- **p:** run things in parallel: all lines in a Make recipe
