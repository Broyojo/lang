fn main() {
    var a = 0
    var b = 1
    var c = a + b
    var f = fib(c)
    var t = true
    var f = false
    var x = a == b
    var y = a != b
}

fn fib(n int) int {
    if n < 2 {
        return 1
    } else {
        return fib(n-1) + fib(n-2)
    }
}