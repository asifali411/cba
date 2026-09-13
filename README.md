![banner](./assets/banner.png)

# cba

**cba** is a lightweight build automation tool that lets you define build tasks, dependencies, variables, compiler flags, and shell commands in a simple configuration file.

It is designed to keep build scripts small, readable, and easy to understand.

## Features

- Simple task-based configuration
- Task dependencies with `needs`
- Variables for reusable configuration
- Command execution with `run`
- Command-line argument support
- Minimal syntax
- Suitable for C/C++ and other projects that can be built from shell commands

## Example

Create a `.cba` file in your project:

```
var compiler = "gcc";
var flags = "-Wall -O2";

task build {
  run "{compiler} {flags} src/main.c -o app";
}

task test {
  needs build;
  run "./app --test";
}

task clean {
  run "rm -f app";
}
```

You can then run individual tasks using `cba`.

For example:

```
cba build
cba test
cba clean
```

## Configuration

A cba configuration file consists of variables and tasks.

### Variables

Variables can be declared with `var`:

```
var compiler = "gcc";
var flags = "-Wall -O2";
```

Variables can then be referenced inside commands using `{name}`:

```
run "{compiler} {flags} src/main.c -o app";
```

This makes it easy to change build configuration without modifying every command.

### Tasks

Tasks are declared using `task`:

```
task build {
  run "gcc src/main.c -o app";
}
```

A task contains one or more commands that cba executes when the task is invoked.

### Dependencies

Tasks can depend on other tasks using `needs`:

```
task test {
  needs build;
  run "./app --test";
}
```

Running:

```
cba test
```

will first execute `build`, then execute the test command.

This allows you to create task pipelines without manually running every step.

## Command Arguments

Commands can use the special `{args}` variable to receive arguments passed to the task.

For example:

```
task build {
  run "{compiler} {flags} {args} src/main.c -o app";
}
```

This allows additional arguments to be passed from the command line.

For example:

```
cba build -- -g
```

can result in a command similar to:

```
gcc -Wall -O2 -g src/main.c -o app
```
Note that arguments must be seperated by a "--".

## A More Complete Example

A small C project might use:

```
var compiler = "gcc";
var flags = "-Wall -Wextra -O2";
var output = "app";

task build {
  run "{compiler} {flags} src/main.c -o {output}";
}

task test {
  needs build;
  run "./{output} --test";
}

task run {
  needs build;
  run "./{output}";
}

task clean {
  run "rm -f {output}";
}
```

Then the development workflow becomes:

```
cba build
cba test
cba run
cba clean
```

## Why cba?

Traditional build systems can be powerful, but they can also introduce a lot of syntax and configuration for small projects.

cba aims to provide a smaller alternative:

```
task build {
  run "gcc main.c -o app";
}

task test {
  needs build;
  run "./app --test";
}
```

The configuration stays close to the commands you would run manually.

## Project Structure

A typical project might look like:

```
my-project/
├── .cba
├── src/
│   └── main.c
└── README.md
```

The `.cba` file describes how the project is built and tested.

## Commands

The basic usage is:

```
cba <task>
```

For example:

```
cba build
cba test
cba clean
```

## Design Goals

cba is built around a few simple ideas:

1. **Readable configuration** — build files should be easy to understand.
2. **Minimal syntax** — common build tasks should require very little configuration.
3. **Explicit dependencies** — task relationships should be easy to see.
4. **Shell-friendly** — cba should work naturally with existing command-line tools.
5. **Small projects first** — simple projects should not need a complicated build system.

## Status

cba is currently under development.

The syntax and available features may change as the project evolves.

## Contributing

Contributions, bug reports, feature requests, and improvements are welcome.

When contributing, please try to keep the syntax and implementation aligned with cba's goal of being **simple, readable, and predictable**.