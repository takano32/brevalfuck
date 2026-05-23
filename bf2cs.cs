using System;
using System.Collections.Generic;

public class Bf2Cs
{
    private static int nest = 2;

    public static void Main()
    {
        Header();

        string src = Console.In.ReadToEnd();
        foreach (char ch in src)
        {
            PrintCode(ch);
        }

        Footer();
    }

    private static void Header()
    {
        Console.WriteLine(@"using System;
using System.Collections.Generic;

public class Brainfuck
{
    public static void Main()
    {
        var data = new List<int> { 0 };
        int index = 0;
");
    }

    private static void Footer()
    {
        Console.WriteLine(@"    }
}");
    }

    private static void PrintCode(char ch)
    {
        switch (ch)
        {
            case '>':
                Line("// >");
                Line("index++;");
                Line("if (data.Count <= index)");
                Line("{");
                nest++;
                Line("data.Add(0);");
                nest--;
                Line("}");
                Line();
                break;
            case '<':
                Line("// <");
                Line("index--;");
                Line();
                break;
            case '+':
                Line("// +");
                Line("data[index]++;");
                Line();
                break;
            case '-':
                Line("// -");
                Line("data[index]--;");
                Line();
                break;
            case '.':
                Line("// .");
                Line("Console.Write((char)data[index]);");
                Line();
                break;
            case ',':
                Line("// ,");
                Line("int input = Console.Read();");
                Line("data[index] = input < 0 ? 0 : input;");
                Line();
                break;
            case '[':
                Line("// [");
                Line("while (data[index] != 0)");
                Line("{");
                nest++;
                Line();
                break;
            case ']':
                Line("// ]");
                nest--;
                Line("}");
                Line();
                break;
        }
    }

    private static void Line(string text = "")
    {
        if (text.Length > 0)
        {
            Console.Write(new string('\t', nest));
        }

        Console.WriteLine(text);
    }
}
