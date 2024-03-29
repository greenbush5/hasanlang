## Parser/Compiler notes
- constructor function should always be named `new`
- default return type `this` for constructor function is automatically set (can optionally be changed)
- constructor function is public by default
- class members are private by default
- class function attributes are to be checked in the semantic analysis stage
- modifiers are to be checked in the semantic analysis stage
- constant variables in classes can only be assigned once in the constructor function *(**only** if they don't have a default value)*
- `pub` modifiers are only allowed in module files *(will be checked during semantic analysis)*
- interface-restricted types should be checked for interface collisions