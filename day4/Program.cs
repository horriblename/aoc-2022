// See https://aka.ms/new-console-template for more information

namespace Day4;
using System;
using System.IO;

internal class Program
{
    static bool task1(int from1, int to1, int from2, int to2)
    {
        return (from1 <= from2 && to1 >= to2) || (from1 >= from2 && to1 <= to2);
    }

    static bool task2(int from1, int to1, int from2, int to2)
    {
        return !(to1 < from2 || to2 < from1);
    }

    static void Main(string[] args)
    {
        string[] lines = File.ReadAllLines("input.txt");
        int counter = 0;
        foreach (string line in lines)
        {
            string[] ranges = line.Split(',');
            string[] range1 = ranges[0].Split('-');
            string[] range2 = ranges[1].Split('-');

            int from1 = Convert.ToInt32(range1[0]);
            int to1 = Convert.ToInt32(range1[1]);
            int from2 = Convert.ToInt32(range2[0]);
            int to2 = Convert.ToInt32(range2[1]);

            if (task2(from1, to1, from2, to2))
            {
                counter++;
            }
        }

        Console.WriteLine(counter);
    }
}

