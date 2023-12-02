using AdventOfCode.Common;
using AdventOfCode.Day01;
using Xunit;

namespace AdventOfCode.Solutions.Tests;

public class Day01Tests_Part1
{
    [Fact]
    public async Task TestInput()
    {
        const string ExpectedSolution = "142";

        var testInput = await InputUtils.ReadInputAsync(day: 1, part: 1, test: true);

        var solution = new Solution();
        var actualSolution = await solution.SolvePart1Async(testInput);

        Assert.Equal(expected: ExpectedSolution, actual: actualSolution);
    }

    [Fact]
    public async Task RealInput()
    {
        const string ExpectedSolution = "54239";

        var testInput = await InputUtils.ReadInputAsync(day: 1, part: 1, test: false);

        var solution = new Solution();
        var actualSolution = await solution.SolvePart1Async(testInput);

        Assert.Equal(expected: ExpectedSolution, actual: actualSolution);
    }
}

public class Day01Tests_Part2
{
    [Fact]
    public async Task TestInput()
    {
        const string ExpectedSolution = "281";

        var testInput = await InputUtils.ReadInputAsync(day: 1, part: 2, test: true);

        var solution = new Day01.Solution();
        var actualSolution = await solution.SolvePart2Async(testInput);

        Assert.Equal(expected: ExpectedSolution, actual: actualSolution);
    }

    [Fact]
    public async Task RealInput()
    {
        const string ExpectedSolution = "55343";

        var testInput = await InputUtils.ReadInputAsync(day: 1, part: 2, test: false);

        var solution = new Day01.Solution();
        var actualSolution = await solution.SolvePart2Async(testInput);

        Assert.Equal(expected: ExpectedSolution, actual: actualSolution);
    }
}
