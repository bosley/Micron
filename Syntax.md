# Comments

Comments are basic. We don't allow fancy c-like comments right now. 

```

-- This is an example of a comment line
-- Using two dashes we let the parser know we don't give a rats about whats happening.
-- Comments are not permitted in the REPL... because thats just nonsense. 

```

# Integers
Integers can be just about any size negative or positive thanks to the rug crate. 

```
    a = 3
```

# Floats
Just like integers, floats use the rug crate so they can be just about any size

```
    a = 3.14159
```

# Strings

```
    a = "This is a string. There are many like it, but this one is mine!"
```

# Dicts

The dictionaries are pretty cool. They can be nested to any arbitrary depth, hold any of the primary data types (int float string dict) and the values of each key are set by expression. 

```
    my_dict = {}

    my_dict = {
        'key_1' : 4 + 7
        'key_2' : "Some string"
    }

    key = "key_1"

    -- Will display 11
    my_dict[key]     

    -- Will display "some string"   
    my_dict['key_2'] 
```

# Data priority
String -> Float -> Int

This means that if a string is present, the resulting value will be elevated to a string. Any operations prior to the string in the expression will be calculated before hand and then converted to string iff '+' is used between the numerical value and the string. Strings' only valid mathematical operator is '+'. In any given expression without strings, floats will overrule integers as well. Example : 

```

>> 3 + 2.14 + " : This will be a string "
String("5.1400000000000005684341886080801486968994140625000000 : This will be a string ")

>> "This whole thing will be a string : " + 3 + 9 
String("This whole thing will be a string : 39")

>> 5 * 10 + 3.14
Float(53.140000000000001)

>> 5 * (10 + 3.14)
Float(65.700000000000003)
```

# Conditionals

Conditionals are considered expressions rather than statements. This means that variables can be set to the result of an if condition in the event that one of the statement blocks is activated and contains a 'yield' statement.

```

>> result = if 10 < 0 { yield "This won't be hit" } else { yield "This will be the value!" }
>> result
String("This will be the value!")

```

If the expression is not assigned to anything, the engine will display the result in the event something is yielded:

```
>> if 10 < 0 { yield "This won't be hit" } else { yield "This will be the value!" }
String("This will be the value!")
```

Point to note : The engine displays whatever was put on the stack at the end of a "Bare Expression." That is, an expression that was placed without assignment or any purpose. If you attempt the following : 

```
>> if 10 < 0 { yield "This won't be hit" } else { 0 }
```

You will get the output : 

```
Integer(0)
```

This is not a returned value of the if expression, rather it is the value coming out from the bare expression in the else block. The *only* way to get a usable value out of an if expression is to use *yield*

This means that if you would like to yield the value of an inner if statement, the code would be as follows:

```
>> lhs = 0
>> rhs = 1
>> test = 10
>> result = if lhs < rhs { yield if test == 10 { yield 19 } }
>> result
Integer(19)
```

# Yield

The yield keyword can be dangerous. Yield's purpose is to hand off an expression from the current scope to the stack and let the above scope use the value it gives off as it sees fit. This means that after a yield, the current scope is demolished. If a yield is the global scope, everything will be dropped.

With this in mind, arbitrary scopes CAN be created to ensure a an outer scope us untouched, but a variable can not be assigned to something yielded FROM an arbitrary scope. 

```
{
    a = 10
    b = 11
    c = 12

    {
        p = 99

        - Will destroy the scope with p
        yield 0
    }
}
```

**Data Methods**

Methods prefixed with 'as' copy the value and attempt to convert it to the requested type.
Methods prefixed with 'to' attempt to convert the stored value to the requested type.

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
>> a = (3.14).as_string()
>> b = "4".as_int()
>> c = a.as_string()

>> a = (3.14).as_string()
>> a
String("3.1400000000000001243449787580175325274467468261718750")
>> a.to_float()
Integer(1)
>> a
Float(3.1400000000000001)

>> a = (3.14).as_string()
>> a
String("3.1400000000000001243449787580175325274467468261718750")
>> a.to_float()
>> a
Float(3.1400000000000001)
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