# run

Runs tasks in common automation systems like Makefiles, package.json, Cargo, Bundler, etc.

```
Usage: 
run [options] <task name>
runp <task name>   # shortcut for run -p
```

- a `Makefile` with a recipe of the given name exists --> run it and exit
- a `package.json` file with a script of the given name exists --> run it
- a `Cargo.toml` file exists --> run the respective Cargo command


### Options

- **p:** run things in parallel, for example runs all lines in a Make recipe concurrently
