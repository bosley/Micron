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

|   Method         |  Params     |    Applicable Types
|---               |---          |---
|  .as_int         |   None      |    Integer, Float, String
|  .as_float       |   None      |    Integer, Float, String
|  .as_string      |   None      |    Integer, Float, String
|  .with_precision |   Integer   |    Float
|  .at             |   Integer   |    String

Examples:
```
    let a = (3.14).with_precision(10);
    let b = "4".as_string();
    let c = a.as_int();
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