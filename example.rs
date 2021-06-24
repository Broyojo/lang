func main() {
    let x = 10
    let y = 20
    print(x + y)
}

struct Person {
    name: str
    age: int
}

enum Weekday {
    Sunday
    Monday
    Tuesday
    Wednesday
    Thursday
    Friday
    Saturday
}

macro sayHello() {
    print("hello!")
}

func fib(n: int) -> int {
    if n < 2 {
        return 1
    } else {
        return fib(n-1) + fib(n-2)
    }
}

match safeDiv(1, 2) {
    Some(x) => print("success:", x)
    None    => print("error: divide by zero")
}

import math
import math as x
import math.sin as y

# generic data structures
struct Vec⟨T: Num⟩ {
    X: T
    Y: T
    Z: T
}

{
    # lambdas and map
    let xs = [1, 2, 3, 4, 5]
    xs.map(x -> x + 1)
    print(xs)
}
