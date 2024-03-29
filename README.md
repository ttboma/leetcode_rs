# Table of Content

- [Table of Content](#table-of-content)
  - [Introduction](#introduction)
  - [Build](#build)
  - [Usage](#usage)
  - [Developer's Note](#developers-note)
    - [How to debug with CodeLLDB](#how-to-debug-with-codelldb)
    - [How to debug with lldb](#how-to-debug-with-lldb)
  - [TODO](#todo)

## Introduction

Hi,

This crate is a collection of solutions to [LeetCode](https://leetcode.com/) problems provided by SHIEH, YUEH-CHANG.
The goal of this project is to gain experience in developing a Rust project and learning about data structures and algorithms.

## Build

The project only relies on [rustup](https://www.rust-lang.org/learn/get-started) as a dependency. Once it's installed, please execute `cargo test` from the project's root directory to verify that all tests pass successfully.

## Usage

The following shows an example that how to use this library crate. And please run `cargo doc --open` to access the documentation

   ```rust
   use leetcode_rs::Solution;

   let ans = Solution::fib(5);
   assert_eq!(ans, 5);
   ```

To access the command line interface, please run

   ```bash
   % cargo run --bin solution -- --help
   % cargo run --bin solution -- <COMMAND> --help
   % cargo run --bin solution -- <COMMAND>
   ```

   For example:

   ```bash
   % cargo run --bin solution -- --help
   % cargo run --bin solution -- fib --help
   % echo 10 | cargo run --bin solution -- fib
   55
   ```

## Developer's Note

Please follow the conventions, as noted below, to contribute to this project.

- Use chrome and [Clip LeetCode](https://chrome.google.com/webstore/detail/clip-leetcode/cnghimckckgcmhbdokjielmhkmnagdcp/related)
   extension to maintain documentation of each method of [`Solution`]
- Please install the [git hook](https://git-scm.com/book/zh-tw/v2/Customizing-Git-Git-Hooks) scripts by the following [pre-commit](https://pre-commit.com/) command:

   ```bash
   % pre-commit install --hook-type commit-msg --hook-type pre-commit --hook-type pre-push
   ```

To maintain your code quality, the following is a list of recommended [Visual Studio Code](https://code.visualstudio.com/) plugins:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Rust Doc Viewer](https://marketplace.visualstudio.com/items?itemName=JScearcy.rust-doc-viewer)
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- [markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)
- [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)
- [GitLens — Git supercharged](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)
- [pre-commit](https://marketplace.visualstudio.com/items?itemName=elagil.pre-commit-helper)

### How to debug with CodeLLDB

To debug using CodeLLDB, press `ctrl+shift+D` and choose names besides the green triangle button from `Debug unit tests in library 'leetcode_rs'`, `Debug executable 'solution'` and `Debug unit tests in executable 'solution'`, if you got `.vscode/launch.json` as the following.
Note that if you need to pass arguments to binary run by cargo, just change the field `args`

```json
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in library 'leetcode_rs'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--lib",
                    "--package=leetcode_rs"
                ],
                "filter": {
                    "name": "leetcode_rs",
                    "kind": "lib"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'solution'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=solution",
                    "--package=leetcode_rs"
                ],
                "filter": {
                    "name": "solution",
                    "kind": "bin"
                }
            },
            "args": ["fib"], // so that it work like `cargo run --bin=solution -- fib`
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'solution'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=solution",
                    "--package=leetcode_rs"
                ],
                "filter": {
                    "name": "solution",
                    "kind": "bin"
                },
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
    ]
}
```

### How to debug with lldb

For some reason, the CodeLens shortcut `Debug` is not working for binary.
If you need to use lldb on command line:

```bash
$ lldb ./target/debug/solution -- fib
(lldb) target create "./target/debug/solution"
Current executable set to '/Users/shiehyuehchang/Projects/leetcode/leetcode_rs/target/debug/solution' (arm64).
(lldb) settings set -- target.run-args  "fib"
(lldb) breakpoint set -n main
Breakpoint 1: 2 locations.
(lldb) r
Process 85616 launched: '/Users/shiehyuehchang/Projects/leetcode/leetcode_rs/target/debug/solution' (arm64)
Process 85616 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2
    frame #0: 0x00000001000421d0 solution`main
solution`main:
->  0x1000421d0 <+0>:  stp    x29, x30, [sp, #-0x10]!
    0x1000421d4 <+4>:  mov    x29, sp
    0x1000421d8 <+8>:  mov    x2, x1
    0x1000421dc <+12>: mov    x8, x0
Target 0: (solution) stopped.
(lldb) c
Process 85616 resuming
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100027bc4 solution`solution::main::he36a6f7c97382b90 at main.rs:241:9
   238  }
   239
   240  fn main() {
-> 241      let cli = Cli::parse();
   242
   243      // read from file or stdin
   244      let input = if let Some(path) = cli.file.as_deref() {
Target 0: (solution) stopped.
(lldb) n
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100027c98 solution`solution::main::he36a6f7c97382b90 at main.rs:244:37
   241      let cli = Cli::parse();
   242
   243      // read from file or stdin
-> 244      let input = if let Some(path) = cli.file.as_deref() {
   245          read_from_file(path).unwrap()
   246      } else {
   247          read_from_stdin().unwrap()
Target 0: (solution) stopped.
(lldb) n
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100027d24 solution`solution::main::he36a6f7c97382b90 at main.rs:247:9
   244      let input = if let Some(path) = cli.file.as_deref() {
   245          read_from_file(path).unwrap()
   246      } else {
-> 247          read_from_stdin().unwrap()
   248      };
   249
   250      // You can check for the existence of subcommands, and if found use their
Target 0: (solution) stopped.
(lldb) n
10
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100027d58 solution`solution::main::he36a6f7c97382b90 at main.rs:252:11
   249
   250      // You can check for the existence of subcommands, and if found use their
   251      // matches just as you would the top level cmd
-> 252      match &cli.command {
   253          Commands::AllPathsSourceTarget {} => {
   254              let graph = parse_2d_i32_list(&input);
   255              println!("{:?}", Solution::all_paths_source_target(graph));
Target 0: (solution) stopped.
(lldb) n
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100027f28 solution`solution::main::he36a6f7c97382b90 at main.rs:325:31
   322              println!("{:?}", Solution::divisor_game(n));
   323          }
   324          Commands::Fib {} => {
-> 325              let n = parse_i32(&input);
   326              println!("{:?}", Solution::fib(n));
   327          }
   328          Commands::FindBall {} => {
Target 0: (solution) stopped.
(lldb) n
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x000000010002f16c solution`solution::main::he36a6f7c97382b90 at main.rs:326:30
   323          }
   324          Commands::Fib {} => {
   325              let n = parse_i32(&input);
-> 326              println!("{:?}", Solution::fib(n));
   327          }
   328          Commands::FindBall {} => {
   329              let grid = parse_2d_i32_list(&input);
Target 0: (solution) stopped.
(lldb) n
55
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x00000001000288e0 solution`solution::main::he36a6f7c97382b90 at main.rs:746:1
   743              println!("{:?}", Solution::max_area(heights));
   744          }
   745      }
-> 746  }
Target 0: (solution) stopped.
(lldb) q
```

## TODO

- [ ] How to implement Iterator for [`TreeNode`]?
- [ ] How to build [`TreeNode`] by macro? And modify those [`TreeNode`] unit tests by macro
