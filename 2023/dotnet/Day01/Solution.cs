using AdventOfCode.Common;

namespace AdventOfCode.Day01;

public class Solution : IDaySolution
{
    public Task<string> SolvePart1Async(string input)
    {
        var result = input
            .Split(Environment.NewLine)
            .Select(line =>
            {
                var firstNumber = GetFirstDigit(line, acceptTextForm: false);
                var lastNumber = GetLastDigit(line, acceptTextForm: false);
                var concatenatedNumber = $"{firstNumber}{lastNumber}";
                var number = int.Parse(concatenatedNumber);
                return number;
            })
            .Sum();

        return Task.FromResult(result.ToString());
    }

    public Task<string> SolvePart2Async(string input)
    {
        var result = input
            .Split(Environment.NewLine)
            .Select(line =>
            {
                var firstNumber = GetFirstDigit(line, acceptTextForm: true);
                var lastNumber = GetLastDigit(line, acceptTextForm: true);
                var concatenatedNumber = $"{firstNumber}{lastNumber}";
                var number = int.Parse(concatenatedNumber);
                return number;
            })
            .Sum();

        return Task.FromResult(result.ToString());
    }

    private static int? GetFirstDigit(string line, bool acceptTextForm = false)
    {
        for (var i = 0; i < line.Length; i++)
        {
            if (IsValidDigit(line[i..], acceptTextForm) is { } digit) return digit;
        }
        
        return null;
    }

    private static int? GetLastDigit(string line, bool acceptTextForm = false)
    {
        for (var i = line.Length - 1; i >= 0; i--)
        {
            if (IsValidDigit(line[i..], acceptTextForm) is { } digit) return digit;
        }

        return null;
    }

    private static int? IsValidDigit(string input, bool acceptTextForm = false)
    {
        if (int.TryParse(input.First().ToString(), out var digit)) return digit;

        if (!acceptTextForm) return null;

        if (input.StartsWith("one")) return 1;
        if (input.StartsWith("two")) return 2;
        if (input.StartsWith("three")) return 3;
        if (input.StartsWith("four")) return 4;
        if (input.StartsWith("five")) return 5;
        if (input.StartsWith("six")) return 6;
        if (input.StartsWith("seven")) return 7;
        if (input.StartsWith("eight")) return 8;
        if (input.StartsWith("nine")) return 9;

        return null;
    }
}
