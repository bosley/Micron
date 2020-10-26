```
           ██████   ██████   
          ░░██████ ██████    
█████ ████ ░███░█████░███    
░░███ ░███ ░███░░███ ░███    
░███ ░███  ░███ ░░░  ░███     Micron Language  
░███ ░███  ░███      ░███     --------------------  
░░████████ █████     █████    Author : Josh A. Bosley
░░░░░░░░   ░░░░░     ░░░░░    License: MIT          
```
[![Build Status](https://travis-ci.com/bosley/Micron.svg?branch=main)](https://travis-ci.com/bosley/Micron) 
![](https://img.shields.io/badge/Micron-Built%20with%20Rust-red)
![](https://img.shields.io/badge/Status-WIP-yellow)


The Micron (*uM*) language is a small toy language being used to help me explore rust and language creation. The Micron language uses [LALRPOP](https://github.com/lalrpop/lalrpop) along with [rug](https://gitlab.com/tspiteri/rug) to do the heavy lifting of language operations. It all started with me wanting to play with [LALRPOP](https://github.com/lalrpop/lalrpop) and then it seemed obvious that [rug](https://gitlab.com/tspiteri/rug) was the only good way to handle numbers. With [rug](https://gitlab.com/tspiteri/rug) we can do silly things like raise 7 to the 99th power (4.6206807e+83): 

```
>> 7 ** 99
Integer(MInteger { value: 462068072803536855906378252728602401551029028414946485847699333055955922805275437143 })
```

## Operations 

**Assignments**

Variables can be assigned with the 'let' keyword. 

```
     <variable> = <expression>
```

**Expressions**

Any expression found without the 'let' keyword will be computed, with the result printed to the screen. Expressions can contain variables, integers, and floats. Any expression with a float will elevate the resulting value to a float. 

```
a + 3 * 2
```

**Data Methods**

|   Method         |  Params          |   Return Value                |    Applicable Types
|---               |---               |---                            |---
|   as_int         |   None           |  New item as representation   |    Integer, Float, String
|   as_float       |   None           |  New item as representation   |    Integer, Float, String
|   as_string      |   None           |  New item as representation   |    Integer, Float, String
|   at             |   Integer        |  String                       |    String
|   to_int         |   None           |  Integer 1 = Success          |    Integer, Float, String
|   to_float       |   None           |  Integer 1 = Success          |    Integer, Float, String
|   to_string      |   None           |  Integer 1 = Success          |    Integer, Float, String

Examples:
```
    a = (3.14).as_string()
    b = "4".as_int()
    c = a.as_string()

```

**Built In Methods**

|   Method         |  Params          |   Return Value 
|---               |---               |---           
|   drop           |  Existing Var    |     Integer 1 = Success     

_

Built in functions must be prefixed by a '#'. These methods can not be in an expression themselves.

Example output:
```
>> a = 3
>> a
Integer(3)
>> #drop(a)
Integer(1)
>> a
Error: UnknownVariable
>> 

```

**Dictionaries**

```

<var> = {
    '<key>': <expr>,
    '<key1>': <expr>
}

<var>['key']

<var>[<var>]    // Where the [<var>] must be a string

```

Dictionaries can be nested to an arbitrary depth and used just as you'd expect them to

```
>> a = { 'moot' : 44,  'josh': 28, 'inner': { 'object': "Hello } }
Error >>> Invalid token at 53
>> a = { 'moot' : 44,  'josh': 28, 'inner': { 'object': "Hello" } }
>> a['inner']['object']
String("Hello")
>> a['inner']['object'] = { 'now' : { 'a' : { 'dict': 44 } } }
>> a['inner']['object']['now']['a']['dict']
Integer(44)
>> a['inner']['object']['now']['a']['dict'].as_string()
String("44")
>> a['inner']['object']['now']['a']['dict'].to_float()
Integer(1)
>> a['inner']['object']['now']['a']['dict']
Float(44.000000000000000)
>> #drop(a)
Integer(1)
>> a = {'hello' : "world", 'goodbye': "everyone" }
>> b = "hello"
>> a[b]
String("world")
>> 
>> c = "goodbye"
>> a[c]
String("everyone")
>> 
```

**Operations**

Current operations are bellow and aim to follow the [C++ operation precedence](https://en.cppreference.com/w/cpp/language/operator_precedence).

| Operation | Description
|--         |--         
|    +      |    Add
|    -      |    Subtract
|    /      |    Divide
|    *      |    Multiply
|   **      |    Power
|   <       |    Less Than
|   >       |    Greater Than
|   <=      |    Less Than or Equal to
|   >=      |    Greater Than or Equal to
|   ==      |    Equal to
|   !=      |    Not Equal
|  \|\|     |    Or
|  &&       |    And
|   %       |    Modulus
|   <<      |    Left Shift
|   >>      |    Right Shift
|   ^       |    Exclusive Or
|   \|      |    Bitwise Or
|   &       |    Bitwise And
|   ~       |    Bitwise Not
|   !       |    Negate