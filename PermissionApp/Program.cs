// See https://aka.ms/new-console-template for more information
using My.Company;

internal class Program
{
    private static void Main(string[] args)
    {
        Console.WriteLine("Hello, World!");

        var v = new Vec2();
        v.x = 1;
        v.y = 2;
        var w = InteropClass.my_function(v);
        Console.WriteLine(v.x);
        Console.WriteLine(w.x);

    }
}