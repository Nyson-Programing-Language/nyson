### print statements
```
log("hello world");
```

### print math 
```
log(math(5+5));
```

### loop 5 times
```
loop(5) {
    log("loops");
}
```

### set variables
```
dec str hello: "bob";
```
this makes a string variable called hello set to bob

### print variables
```
dec str name: "bob";
loop(5) {
    log("Hello, " name);
}
```

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