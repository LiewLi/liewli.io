# Functioal Programming Languages

* Haskell
* SML
* OCaml
* Scheme
* Clojure
* F#
* Scala
* Erlang
* ...

# Functions vs Procedures
*	Function == expression, Procedure == statement
*  Function关注的是computation而Procedure关注的是instruction
*  Function 是一种映射，Domain-->Range


# Functional Programming (OCaml)
***
* 1996年由 Xavier Leroy在 INRIA，France开发
* ML Family
* 支持，Functional，Imperative，和OOP范式
* F#, ReasonML, BuckleScript, Flow
* Facebook, Jane Street, Bloomberg

***
*Sample Code*

```OCaml
(* Hello World *)
print_string "hello world"
```

```OCaml
(* Binary tree with leaves car­rying an integer. *)
type tree = Leaf of int | Node of tree * tree

let rec exists_leaf test tree =
  match tree with
  | Leaf v -> test v
  | Node (left, right) ->
      exists_leaf test left
      || exists_leaf test right

let has_even_leaf tree =
  exists_leaf (fun n -> n mod 2 = 0) tree
```

***
*Type Inference*

```OCaml
let add x y = x + y;;
(* val add : int -> int -> int = <fun> *)
```
***

*Recursion*

```OCaml
let rec sum n = if (n = 0) then 0 else n + sum(n-1) ;;
```
*Tail Recursion*

```OCaml
let rec sum n acc = if (n = 0) then acc else sum (n-1) (acc + n);;
```

***

*High Order Function*

```OCaml
let compose f g  = fun x -> f (g x) ;;
let double = fun x -> 2 * x;;
let square = fun x -> x * x ;;
let f = compose square double ;;
let a = f 3;;
(* val a : int = 36 *)
```
***

*Currying*

In mathematics and computer science, currying is the technique of translating the evaluation of a function that takes multiple arguments (or a tuple of arguments) into evaluating a sequence of functions, each with a single argument. 

```OCaml
let add x y = x + y ;;
(* val add : int -> int -> int = <fun> *)
let f = add 2;;
(* val f : int -> int = <fun> *)
let a = f 3;;
(* val a : int = 5 *)
```

***

*Algebraic Data Type & Pattern Matching*

```OCaml
type tree = Leaf of int | Node of tree * tree
```

```OCaml
type shape = Circle of float | Square of float | Rectangle of (float * float);;

let area s = match s with
| Circle r -> 3.14 *. r *. r
| Square x -> x *. x
| Rectangle (x, y) -> x *. y;;
(* val area : shape -> float = <fun> *)

let a = area (Circle 1.0);;
(* val a : float = 3.14 *)
let b = area (Square 2.0);;
(* val b : float = 4. *)
let c = area (Rectangle (3.0, 4.0));;
(* val c : float = 12. *)
```

***

*Immutability*

```OCaml
let a = [1; 2; 3];;
let b = [4; 5; 6];;
let c = a @ b ;;
(* val c : int list = [1; 2; 3; 4; 5; 6] *)
a;;
(* int list = [1; 2; 3] *)
```

***

*Strict Evaluation (Lazy Evaluation)*

```OCaml
let f1 () = print_string "f1\n"; 1 ;;
let f2 () = print_string "f2\n"; 2 ;;
let _if cond _then _else = if cond then _then else _else ;;
let a = _if true (f1()) (f2()) ;;
(*f2
*f1
*val a : int = 1
*)
```

***

*Pure Function*

* none-pure function : malloc ,random
* pure function: let add x y = x + y;
* 函数输出值由输入值唯一确定并且没有side effects
