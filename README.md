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


The Micron (*uM*) language is a small toy language being used to help me explore rust and language creation. The Micron language uses [LALRPOP](https://github.com/lalrpop/lalrpop) along with [rug](https://gitlab.com/tspiteri/rug) to do the heavy lifting of language operations. It all started with me wanting to play with [LALRPOP](https://github.com/lalrpop/lalrpop) and then it seemed obvious that [rug](https://gitlab.com/tspiteri/rug) was the only good way to handle numbers. 

## Operations 

**Assignments**

Variables can be assigned with the 'let' keyword. 

```
    let <variable> = <expression>;
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
    let a = (3.14).with_precision(10);
    let b = "4".as_string();
    let c = a.as_int();
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
>> let a = 3;
>> a
Integer(MInteger { value: 3 })
>> $to_string(a)
String(MString { value: "Modifications Complete" })
>> a
String(MString { value: "3" })
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