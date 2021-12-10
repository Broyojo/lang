: // used for assigning types
:= // used for value assignment with type inference
= // is used for assignment
\ // lambda statement
#struct // struct macro
#module // module macro
#import // import macro
{} // for encapsulating sequential code
for // for loops
match // pattern matching
if else // 

f : int -> int
f = \x -> x + 1

f : int -> int = \x -> x + 1

f := \x -> x + 1

Data b := struct {
    x: b,
}

thing T := module {
    thing: type = T
    a := \x -> print x
}

thing::a 2

c: chan i32 = thing

c: thing

c <- thing

_ |> _ : a -> (a -> b) -> b = \x f -> f x

thing = left | right

thing := #struct {
    x: 0
    y: 1
}

