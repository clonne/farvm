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
cargo run <path of  .fa file>
```
> example
```shell
cargo run hello.fa
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
main: fn() {
    (println! "hello world!")
}
```

```
# Multiple Dispatch
md: method {
    <>(<Int>a)        = "Int:{a}"
    <>(<Float>a)      = "Float:{a}"
    <>(<Bool>a)       = "Bool:{a}"
}

main: fn() {
    let s = [(<Int> 1) (<Float> 1.0) true]
    (each (pipe md println!) s)
}
```

## ! Thank
> hope me persist
>
