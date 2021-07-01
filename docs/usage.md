## print
```
log("hello world");
```

## math
```
log(math(5+5));
```

## loop
```
loop(5) {
    log("loops");
}
```

## variables
### set
```
dec str hello: "bob";
```
this makes a string variable called hello set to bob

### call
```
dec str name: "bob";
log("Hello, " name);
```

## random number
### print random number
```
log(math(random));
```
this will give you a float between 0 and 1

### print random number between 0 and 10

```
log(math(random*10));
```
get something like 5.5539

### print rounded random number
```
log(round(math(random*10)));
```
get something like 3

## Functions
### with out variables
```
func(sayHello()) {
    log("hello");
}
sayHello();
```

### with variables
```
dec str name: "bob";
func(sayHello()) {
    log("hello " name);
}
sayHello();
```

## If Statements
```
dec str condition : "true";
if condition : "true" <
    log("true")
>
"""
