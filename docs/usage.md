## Print
```
log("hello world");
```

## Math
```
log(math(5+5));
```

## Loop
```
loop(5) {
    log("loops");
}
```

## Variables
#### Set
```
dec str hello: "bob";
```
this makes a string variable called hello set to bob

#### Call
```
dec str name: "bob";
log("Hello, " name);
```

## Random Number
#### Print Random Number
```
log(math(random));
```
this will give you a float between 0 and 1

#### Print Random Number Between 0 and 10

```
log(math(random*10));
```
get something like 5.5539

#### Print Rounded Random Number
```
log(round(math(random*10)));
```
get something like 3

## Functions
#### Without Variables
```
func(sayHello()) {
    log("hello");
}
sayHello();
```

#### With Variables
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
if condition : "true" {
    log("true");
}
```

## Sleep
```
log("Hi");
sleep(5000);
log("I was sent 5 seconds later");
```

## Replace
```
log(replace("I am really bad", "bad", "good"));
```
will give "I am really good"

## Trim
```
log(trim("        Hello      "));
```
will give "Hello"

## Import
FILE: hello.nys
```
func(sayhello()) {
    log("hello")
}
```
FILE: main.nys
```
imp("hello.nys")
sayhello()
```
Returns: hello
