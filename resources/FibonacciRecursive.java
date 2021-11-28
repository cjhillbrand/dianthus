public class FibonacciRecursive {
    public static int main(String[] args) {
        int a = 10;
        return FibonacciRecursive.fib(a);
    }

    public static int fib(int a) {
        if (a < 2) return a;
        return FibonacciRecursive.fib(a - 1) + FibonacciRecursive.fib(a - 2);
    }

    public void init(FibonacciRecursive obj) {

    }
}