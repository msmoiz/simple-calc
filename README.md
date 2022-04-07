# simple-calc

simple-calc is a command line calculator that consumes input expressions and outputs their evaluated results. It served as a nice sandbox project for:

* Setting up a command line interface
* Parsing and validating arbitrary input
* Learning more about parsing terminology, like postfix notation, and algorithms, like the shunting-yard algorithm
* Getting comfortable with error handling using `Result`s instead of exceptions

It can be used for scripting or as an interactive tool. Below is a example of the tool in action.

```shell
$ simple-calc "128 - (8 * 16 / ((3 + 1) / 2))"
64
$ simple-calc "7 + 2 ~ 3"
Error: "encountered invalid character ~ in expression at position 6"
$ simple-calc "4 * 9 + ("
Error: "input expression contains mismatched parentheses"
$ simple-calc
Using simple-calc in interactive mode.
Use '$?' to access the previous result.
Use '${N}' to go further back in history.
Enter 'exit' to quit.
>> 0 + 1
The result is 1.
>> $? + 1
The result is 2.
>> $0 + $1
The result is 3.
>> exit
Thanks for using simple-calc. Peace.
```
