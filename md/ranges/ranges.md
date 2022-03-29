# C++20 overview - ranges
Ranges are an extension of the Standard Template Library that makes its iterators and algorithms more powerful by making them composable. Ranges is the future of STL, or namely STL 2.0. 

## What is ranges?
```
 range | adapt1 | adapt2...
```
[Ranges-v3](https://github.com/ericniebler/range-v) forms the basis of range support in the C++20 standard library. Ranges-v3 is built on three pillars: Views, Actions, and Algorithms.

### Range

A range is object on which you can call begin() and end(), where begin() returns a iterator, which can be incremented until it is equal to the thing returned from end().
Ranges don't replace iterators, they build on them. Ranges formalize many of the notions already implicit in the existing STL.

### View

View is a lightweight wrapper that presents a view of underlying sequence of elements in some custom way without mutating or copying it. View is intended for combination and lazy evaluation.

### Actions

Actions are used to mutate a container in-place, or forward it through a chain of mutating operation;

### Algorithm

Ranges provides constrained versions of most algorithms. In these algorithms, a range can be specified as either a iterator-sentinel pair or as a single range argument, and projections and pointer-to-member callables are supported.

## Examples

### Fibonacci sequence
```cpp
    //fibonacci sequence
    rng::copy(views::generate([p = std::pair{0, 1}]() mutable {
        auto [p0, p1] = p;
        p = {p1, p0 + p1};
        return p0;
    }) | views::take(10), rng::ostream_iterator(std::cout, " "));
```

```
Program returned: 0

Program stdout

0 1 1 2 3 5 8 13 21 34 
```

### Filter & Map

```cpp
    rng::copy(views::iota(1, 20) //generate [1, 2, 3, ..., 19]
    | views::filter([](auto a) { return a % 2 == 0;}) //filter [2, 4, ...]
    | views::transform([](auto a) {return a * a;}), //map [4, 16, ...]
    rng::ostream_iterator(std::cout, " ")
    );
```
```
Program returned: 0

Program stdout

4 16 36 64 100 144 196 256 324 
```

### snake_case to CamelCase
```cpp
 //snake_case to CamelCase
    auto s = std::string("make_epic_shit");
    rng::copy(s
    | views::split('_') 
    | views::transform([](auto w) {
        auto head = w 
                    | views::take(1) 
                    | views::transform([](auto a) {return std::toupper(a);});
        return views::concat(head, w | views::tail);
    })
    | views::join 
    | rng::to<std::string>(),
    rng::ostream_iterator(std::cout)
    );
```

```
Program returned: 0

Program stdout

MakeEpicShit
```



## Conclusion
Ranges, combined with [Concepts](https://en.cppreference.com/w/cpp/concepts) lay down the foundation for STL2.0. Ranges is highly efficient, and helps writing expressive c++ code. 

## Reference
1. [Ranges-v3](https://github.com/ericniebler/range-v3)
2. [NanoRange](https://github.com/tcbrindle/NanoRange)
3. [cmcstl2](https://github.com/CaseyCarter/cmcstl2/blob/master/include/stl2/view/iota.hpp)
4. [Ranges-v3 user manual](https://ericniebler.github.io/range-v3/index.html)

