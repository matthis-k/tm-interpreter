# Goal
A simple way to simulate a turing machine.

# Usage
`tm-interpreter <path-to-tm> [OPTIONS]`  
[OPTIONS]: `-v` -> verbose mode (prints the TM step by step)

# How to define a TM
In a file. For example:
```
A Turing machine that accepts if the input is divisible by 3
alphabet: 0123456789
tape: 14526
tape_offset: 0
start_state: S0
accepted_states: S0
rule: S0 0369 none r S0
rule: S0 147 none r S1
rule: S0 258 none r S2
rule: S1 0369 none r S1
rule: S1 147 none r S2
rule: S1 258 none r S0
rule: S2 0369 none r S2
rule: S2 147 none r S0
rule: S2 258 none r S1
```

Let's break that down:
The basic structure is: `keyword: <values>`. Any line that does not match this will be ignored.  

## Alphabet
Sets the alphabet to be used, here all possible characters are digits.
```
alphabet: 0123456789
```
## Tape
Sets the input word.
```
tape: 14526
```
## Tape Offset
Defines where the tape starts. An offset of 3 would shift the tape 3 characters to the LEFT(for example if you want to write an oracle to the left of the head, the offset yould be the length of that oracle)
```
tape_offset: 0
```
## States
To control a turing machine a state pattern is used.  
Here we do not need a complete definition of all possible states, as a starting state and all state transitions (I will refer to them as rules) are sufficient.  
A state is unambiguously defined by its name.  
### Start state
In which state the TM starts.
```
starting_state: S0
```
### Accepting States
Defines which states are accepting end states. If there are multiple ones they are separated by a space like so `S0 S1`.
```
accepted_states: S0
```
### Rules (state transition)
A rule consists of 5 things.  
  * The state the rule is 'active' in
  * On what it reads
  * What does it write
  * Where it moves after
  * The next State

The first 2 are conditions to be met, the last 3 are actions that are taken if those conditions are met.
Generally speaking it is: `rule: <in-state> <read-condition> <write-action> <head_movement> <next-state>`, where:  
`states` are just a `String`  
`<read-condition>` is one of the following:
 * `any` -> if any character is written on the tape at the heads position
 * `empty` -> if there is character at the head
 * `12345` -> if this contains the character at the head (this would rule would apply for [1-5])  

`<write-action>` is one of:
  * `none` -> leave it as is
  * `delete` -> erase the current char
  * `a` (a single char) -> write `a`

`<movement>` is any singular char, `l` moves to left, `r` to right, the rest stays in place.
```
rule: S1 258 none r S0
      ^^^^^^ if in state S1 and read 2, 5 or 8
             ^^^^^^^^^^ then leave the tape as is, move to the right and switch to state S0
```
