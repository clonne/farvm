# An Language(.fa) Virtual-Machine write by Rust
* it's incomplete now
* stage on initation
<br>

## ! Develop About
> Rust 2021
>
> Language Standard-v1 for fa
>
>

## ! Usage
```shell
cargo run -- <source-path>
```
> example
```shell
cargo run -- hello.fa
```

## ? What is fa
> Project Orientation
>
> Static Typed
>
> Multiple Dispatch
>
> Functional
>
> Generic and Macro
>
> Object for Prototype Based
>
> It's immature and not implemented now
>
> 'fa' is abbreviation be cause full-name isn't public now
>
> Other...
>

## ? Target of fa
> Easy to use in Practical Engineering
>
> Combine functional(High-Abstract) and for(Details)
>
> Fundamentally advance of Language Thory
>

## : Examples of fa
```
# The 'Hello World'
(main) = {
    (println! "hello world!")
}
```

```
# Multiple Dispatch
for apply (md ...) {
    (a:Number) = {"Number:{a}"}
    (a:Float) = {"Float:{a}"}
    (a:Bool) = {"Bool:{a}"}
}

(main) = {
    s = [1i32 1.0 true]
    (each (pipe md println!) s)
}
```

## ! Thank
> hope me persist
>
