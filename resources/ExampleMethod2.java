public class ExampleMethod2
{
    public static void main(String[] args)
    {
        int a = 9;
        int b = 4;
        int myResult = MyOtherClass.add(a, b);
    }
}

class MyOtherClass {
    public static int add(int a, int b) { return a + b; }
}