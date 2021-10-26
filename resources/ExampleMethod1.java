public class ExampleMethod1
{
    public final static int MY_INT = 1;
    public static void main(String[] args)
    {
        int rhs = 5;
        int result = ExampleMethod1.add(MY_INT, rhs);
    }

    private static int add(int one, int two)
    {
        return one + two;
    }
}