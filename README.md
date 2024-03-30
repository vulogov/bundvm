# BUND Virtual Machine

Before we address the nature of the BUND Virtual Machine, let's first explore the concept of Virtual Machines in general.

## What is the "Virtual Machine"

A Virtual Machine represents a computational paradigm that interprets a specific language to execute computations. Numerous virtual machine implementations have been developed throughout computing history, employing various calculation methods. Several virtual machines utilize primitives for data storage before computation, such as storage registers, which have been widely adopted in the industry for an extended period. Another fundamental is "a stack," which, in specific architectures, is dedicated to preserving the history of procedural calls, while in others, it can serve for both data storage and call history. The third alternative involves using the stack for data storage. In the proposed BUND Virtual Machine, this approach forms the architectural foundation. Loosely classified as a virtual machine implementing a concatenative computational paradigm, the BUND Virtual Machine stands as an example of this computational approach.

For almost 90 years, we have known the Turing Machine, a mathematical computational model first proposed by British mathematician Alan Turing. The foundational principle of the Turing Machine is an endless tape separated into discrete cells. Each cell could hold a piece of data. The Turing Machine can read data from a cell, write data to the cell, move left or right to the previous or the next cell, or halt the execution. This simple model, nevertheless, provides a foundation for most modern computational machines. While the architecture of the Turing Machine is minimalistic and elegant, building it in real life is nearly impossible. And one of the limitations is the requirement for an "endless tape." But while creating an authentic Turing Machine "in metal" is an impossible task, with a few approximations, we can build a practical stack-based Turing Machine that supports data isolation. BUND virtual machine's first principal architectural element is an endless stack that forms the ring. The stack has no beginning or end, only a "current element."  The Machine can move left or right in this "paper tape" without reaching the end. BUND Virtual Machine can perform the specific basic operations with the stack:

- Push data into the current position. After this operation, the current position pointed to the freshly pushed data elements.
- Pull data from the stack. The stack size is reduced.
- Look at the data in the current position without removal.
- Circularly rotate the stack left or right, changing the current position in the stack.
- Duplicate data in the current position; after this operation, two copies of the same data will be located in adjacent cells of the stack.

## And now let's start to explore the features of the BUND.

Operating with data stored in circular ring structures will help us build a real-world Turning Machine by approximating an "endless tape" using a "ring buffer."  All concatenative computation models with a single stack share the same problem: weak data isolation. If we store data for multiple computational contexts in the same stack, we can avoid a lack of proper data protection from rogue code. The BUND Virtual Machine brings a paradigm called a "stack-of-stacks" to impose data isolation for different computation branches. The root stack of the Virtual Machine is a ring stack, and it doesn't store the data. It stores a reference on a ring stack, each cell storing a dynamic data element. Operations with data storage are logically divided into two levels:

Operations with a data stack storing a reference to the stacks that store the data.
Operations with data stored in the circular stack that stores the actual data. Each data stack is referenced through a first level, the "stack-of-stacks."

Principles of operations with "stack-of-stacks" and "stack-of-data" are essentially the same.

The rationale for building the Virtual Machine built on those principles goes into the following:

- I am experimenting with a Virtual Machine and concatenative algorithms inside an execution model close "in the spirit" to the ideas behind a Turing Machine.
- I am experimenting with concatenative algorithms in the execution model that provide data isolation.
- I am bringing ideas that remained behind concatenative programming into a world of modern computation.
- BUND Virtual Machine is a foundation behind the concatenative-functional programming language BUND.

While the BUND Virtual Machine's "stack-of-stacks" is designed to hold data elements, let's discuss which data elements it could store and use. The single data element is a dynamic structure that can hold any supported datatype. The first definition of the BUND is that this is a Virtual Machine optimized for dynamically typed datatype processing. Currently, the following data types are supported by VM: numeric integers and floats of size 64 bits, strings, boolean, and lists. In addition to those "basic" datatypes, the following ones have a special meaning and change the code execution flow:

- CALL - datatype that contains information related to the execution of either immediate stack execution applicative, delayed stack execution applicative, or registered lambda function.
- LAMBDA - datatype containing instructions for execution when the Virtual Machine executor calls the Lambda function.

While BUND Virtual Machine has limited support for immediate execution, it uses delayed execution for most functions or primitive calls. The difference is that if a function or primitive is defined in a special Applicative call table, where this table is simply a reference between names and applicative functions when called, the function defined in this call is explicitly executed immediately. The functions described in this table and, therefore, destined for immediate execution are usually tasks that change the state of the Virtual Machine itself. The primitives defined in a generic functor table and user-defined lambda functions are designed for delayed execution. If the function is called for a delayed execution, the reference on this function is temporarily placed by Virtual Machine into a call stack. When a Virtual Machine receives a call for execution, and this function is registered in the Applicative table, the Virtual Machine takes reference to the function from a call stack and executes that function. Every time a Virtual Machine executes a registered internal function, it refers to the properties of that function that require a certain number of arguments to be taken from the stack and passed to the function. The options for converting stack arguments to function arguments are defined in the function signature and are NO EXTRA - this will leave the stack unchanged, and the function itself will interact with the current stack in the way it fits the function. JUSTONE - A single value will be taken from the stack and passed to the function as parameters. JUSTTWO - Virtual Machine will remove two values from the stack and passed to the function. TAKEALL - Virtual Machine will take all previously pushed values into a current stack and passed to the function. If the function returns the value, Virtual Machine will push it into a current stack.

## Show me how it is works.

Each Virtual Machine requires some input instructions, which it can execute to perform the operations its user needs. Currently, I choose JSON as a primary format for instructions. I will add other, more optimized formats in the future, but for now, it will do. The first classic example that demonstrates how the Machine operates is the famous "Hello World."

```JSON
{"type": "STRING", "value": "Hello world!"}
{"type": "CALL", "value": "print"}
```

The first instruction will take the value passed through the "value" key and push it as a dynamic element of type "STRING" into a stack. The second instruction will search for the CALL named "print." This call will be found in the Applicative table, and therefore, it will be executed immediately. The call signature includes a requirement for the function parameter to have just one value, so before calling this function, a single value will be taken from the stack and passed to the function.

To execute those instructions, first, you have to build the Virtual Machine. You have to have Rust toolchain installed on your build host. At this time, I am using Rust version 1.76.0, so you shall use this version or newer. In order to get all required libraries, you have to be connected to the Internet. First, check out the newest version of the BUND VM from [https://github.com/vulogov/bundvm](https://github.com/vulogov/bundvm)

```shell
git clone git@github.com:vulogov/bundvm.git
cd bundvm
make
```

After build is successful, you will have an executable bundvm in the directory ./target/debug . You can use instructions from the distribution, or write two-line JSON file in any file of your choosing. Then execute those instruction:

```shell
./target/debug/bundvm -dd vm --file {I mask full path to the file with instructions}/examples/helloworld.json
```

Some notes for executing instructions:

- Path to the file passed through --file must be a full path, not a relative path. You can pass instructions for execution from STDIN, then you must pass an argument --stdin instead of --file, or you can put your instructions somewhere on the Web and pass URL to your instructions to argument --url of option vm.
- -d (or -dd or -ddd) will instruct VM to display informational, debug or trace level messages from the Virtual Machine. Use this optiuon if you curious to see how VM works.

The output of this command will be

```shell
[2024-03-30T21:00:36Z DEBUG bundvm::cmd::setloglevel] Set loglevel=debug
[2024-03-30T21:00:36Z DEBUG bundvm::cmd::setloglevel] setloglevel::setloglevel() reached
[2024-03-30T21:00:36Z DEBUG bundvm::stdlib] Running STDLIB init
[2024-03-30T21:00:36Z DEBUG bundvm::cmd] ZENOH bus set to: tcp/127.0.0.1:7447
[2024-03-30T21:00:36Z DEBUG bundvm::cmd] ZENOH listen set to default
[2024-03-30T21:00:36Z DEBUG bundvm::cmd] ZENOH configured in PEER mode
[2024-03-30T21:00:36Z DEBUG bundvm::cmd] ZENOH config is OK
[2024-03-30T21:00:36Z DEBUG bundvm::cmd] Configuring VM
[2024-03-30T21:00:36Z DEBUG bundvm::vm_stdlib] Init VM standard library
[2024-03-30T21:00:36Z DEBUG bundvm::vm_stdlib::stdlib_print] Init VM standard library: print
[2024-03-30T21:00:36Z DEBUG bundvm::vm_stdlib::stdlib_stack] Init VM standard library: stack
[2024-03-30T21:00:36Z DEBUG bundvm::vm_stdlib::stdlib_terminate_and_execute] Init VM standard library: terminate_and_execute
[2024-03-30T21:00:36Z DEBUG bundvm::vm_stdlib::stdlib_lambda] Init VM standard library: lambda
[2024-03-30T21:00:36Z DEBUG bundvm::cmd] VM is ready
[2024-03-30T21:00:36Z DEBUG bundvm::cmd] Execute VM instructions
[2024-03-30T21:00:36Z DEBUG bundvm::cmd::bund_vm] If there is an error, VM will enter into interactive shell: false
[2024-03-30T21:00:36Z DEBUG bundvm::stdlib::vm_execute] Received 79 bytes of instructions
[2024-03-30T21:00:36Z DEBUG bundvm::vm::vm_call] BUND VM: applicative found: print
[2024-03-30T21:00:36Z DEBUG bundvm::vm::vm_call] BUND VM: calling applicative: print
Hello world!
```

## How I can play with VM interactively

You can request an interactive REPL prompt and interact with the Virtual Machine by entering shell commands. BUND shell is a LISP interpreter, and for adventurous users, it provides the full power of the LISP to automate and explore the BUND Virtual Machine. I will not venture into teaching you a LISP course, but I will tell you the commands you can use to change the Machine state and execute primitives and, eventually, lambdas.

### My first interactive command in VM shell

First, how you can execute the shell and check the status of VM:

```
% ./target/debug/bundvm -d shell
  ____    _   _   _   _   ____      _        ___         ___
 | __ )  | | | | | \ | | |  _ \    / |      / _ \       / _ \
 |  _ \  | | | | |  \| | | | | |   | |     | | | |     | | | |
 | |_) | | |_| | | |\  | | |_| |   | |  _  | |_| |  _  | |_| |
 |____/   \___/  |_| \_| |____/    |_| (_)  \___/  (_)  \___/


[2024-03-30T21:20:56Z WARN  bundvm::cmd::bund_shell] No previous history discovered
[BUND > (full-status)
╭─────────────────────────────────────┬───────────────╮
│ Message                             ┆ (full-status) │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Instruction                         ┆ N/A           │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Number of stacks                    ┆ 1             │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Size of call stack                  ┆ 0             │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Number of functors                  ┆ 0             │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Lambda scaffolding                  ┆ 0             │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Registered lambdas                  ┆ 0             │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Number of elements in current stack ┆ 0             │
╰─────────────────────────────────────┴───────────────╯

╭───────────────────────╮
│ List of stacks        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ TdCpag2wTL-qG6BxLgRzG │
╰───────────────────────╯

╭───────────────────────────╮
│ Elements in current stack │
╰───────────────────────────╯

╭────────────────────────╮
│ Elements in call stack │
╰────────────────────────╯
```

The command (full-status) will display you the following information:

- Number of stacks. This is what it is - the number of the stacks in the "stack-of-stacks" structure. Each stack is "stack-of-data"
- Size of call stack. This number represents the number of the CALL elements stored for delayed execution
- Number of functors. Size of the table that holds references to the functions that is designed for delayed execution.
- Lambda scaffolding. The current number of lambda functions in the process of creation that are not finished.
- Registered lambdas. Number of finalized and registered lambda functions.
- Number of elements in current stack. Number of data elements in the "stack-of-data" that is current.

Then you will see the list of names of the "stack-of-data" elements
Next you will see the dump of the elements in your current "stack-of-data"
And last, you will see the elements in call stack.

You can call *(full-status)* at any time.

### What is my other shell commands?

| Command | Description | Example |
|--------:|:------------|:--------|
| (push-integer <number>) | Pushing integer value to the current position of current "stack-of-data" | (push-integer 42) |
 
