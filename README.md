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

|   Method         |  Params     |  In Expresison? |    Applicable Types
|---               |---          |---              |---
|   as_int         |   None      |       Yes       |    Integer, Float, String
|   as_float       |   None      |       Yes       |    Integer, Float, String
|   as_string      |   None      |       Yes       |    Integer, Float, String
|   with_precision |   Integer   |       Yes       |    Float
|   at             |   Integer   |       Yes       |    String

Examples:
```
    a = (3.14).with_precision(10)
    b = "4".as_int()
    c = a.as_string()
```

These data methods copy and convert data in-place and do not update the item it references. For instance, the line setting variable "c" sets "c" to the string representation of "a" and does not convert "a" to string. 

**Built in Functions**

|  Function  |  Params           |  In Expression? |   Applicable Types
|--          |--                 |--               |--
|  to_int    | existing var name |       No        |    Integer, Float, String
|  to_float  | existing var name |       No        |    Integer, Float, String
|  to_string | existing var name |       No        |    Integer, Float, String

_

Built in functions must be prefixed by a '$'. These methods can not be in an expression themselves, but they convert the stored variable(s) to the given types. 

Example output:
```
>> a = 3
>> a
Integer(MInteger { value: 3 })
>> $to_string(a)
String(MString { value: "Modifications Complete" })
>> a
String(MString { value: "3" })
>> 

```

**Built in Functions**

|  Function  |  Params           |  In Expression? |   Applicable Types
|--          |--                 |--               |--
|  to_int    | existing var name |       No        |    Integer, Float, String
|  to_float  | existing var name |       No        |    Integer, Float, String
|  to_string | existing var name |       No        |    Integer, Float, String

_

Built in functions must be prefixed by a '$'. 

Example output:
```
>> a = 3
>> a
Integer(MInteger { value: 3 })
>> $to_string(a)
String(MString { value: "Modifications Complete" })
>> a
String(MString { value: "3" })
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