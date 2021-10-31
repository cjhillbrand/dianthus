public class ExampleMethod2
{
    public static void main(String[] args)
    {
        int a = MyOtherClass.MY_CONSTANT;
        int b = 4;
        int myResult = MyOtherClass.add(a, b);
    }
}

class MyOtherClass {
    public static int MY_CONSTANT = 111;
    public static int add(int a, int b) { return a + b; }
}