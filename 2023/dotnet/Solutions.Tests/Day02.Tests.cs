using AdventOfCode.Common;
using AdventOfCode.Day02;
using Xunit;

namespace AdventOfCode.Solutions.Tests;

public class Day02Tests_Part1
{
    [Fact]
    public async Task TestInput()
    {
        const string ExpectedSolution = "8";

        var testInput = await InputUtils.ReadInputAsync(day: 2, part: 1, test: true);

        var solution = new Solution();
        var actualSolution = await solution.SolvePart1Async(testInput);

        Assert.Equal(expected: ExpectedSolution, actual: actualSolution);
    }

    [Fact]
    public async Task RealInput()
    {
        const string ExpectedSolution = "2685";

        var testInput = await InputUtils.ReadInputAsync(day: 2, part: 1, test: false);

        var solution = new Solution();
        var actualSolution = await solution.SolvePart1Async(testInput);

        Assert.Equal(expected: ExpectedSolution, actual: actualSolution);
    }

    [Theory]
    [MemberData(nameof(Day02_UT_GameRoundParsing_Data))]
    public void Day02_UT_GameRoundParsing(string gameRound, IEnumerable<(CubeColor, int)> cubeCounts)
    {
        var parsedRound = Solution.ParseGameRound(gameRound);
        Assert.Equal(cubeCounts, parsedRound);
    }

    public static IEnumerable<object[]> Day02_UT_GameRoundParsing_Data()
    {
        yield return new object[] { "3 blue", new[] { (CubeColor.Blue, 3) } };
        yield return new object[] { "3 blue, 4 red", new[] { (CubeColor.Blue, 3), (CubeColor.Red, 4) } };
        yield return new object[] { "1 red, 2 green, 13 blue", new[] { (CubeColor.Red, 1), (CubeColor.Green, 2), (CubeColor.Blue, 13) } };
    }

    [Theory]
    [MemberData(nameof(Day02_UT_GameParsing_Data))]
    public void Day02_UT_GameParsing(string gameRound, Game game)
    {
        var parsedGame = Solution.ParseGame(gameRound);
        Assert.Equal(game.Id, parsedGame.Id);
        Assert.Equal(game.Rounds, parsedGame.Rounds);
    }

    public static IEnumerable<object[]> Day02_UT_GameParsing_Data()
    {
        yield return new object[] {
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            new Game(1, new[] {
                new Dictionary<CubeColor, int> { { CubeColor.Blue, 3 }, { CubeColor.Red, 4 } },
                new Dictionary<CubeColor, int> { { CubeColor.Red, 1 }, { CubeColor.Green, 2 }, { CubeColor.Blue, 6 } },
                new Dictionary<CubeColor, int> { { CubeColor.Green, 2 } }
            })
        };
        yield return new object[] {
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            new Game(2, new[] {
                new Dictionary<CubeColor, int> { { CubeColor.Blue, 1 }, { CubeColor.Green, 2 } },
                new Dictionary<CubeColor, int> { { CubeColor.Green, 3 }, { CubeColor.Blue, 4 }, { CubeColor.Red, 1 } },
                new Dictionary<CubeColor, int> { { CubeColor.Green, 1 }, { CubeColor.Blue, 1 } }
            })
        };
    }
}

public class Day02Tests_Part2
{
    [Fact]
    public async Task TestInput()
    {
        const string ExpectedSolution = "2286";

        var testInput = await InputUtils.ReadInputAsync(day: 2, part: 2, test: true);

        var solution = new Solution();
        var actualSolution = await solution.SolvePart2Async(testInput);

        Assert.Equal(expected: ExpectedSolution, actual: actualSolution);
    }

    [Fact]
    public async Task RealInput()
    {
        const string ExpectedSolution = "83707";

        var testInput = await InputUtils.ReadInputAsync(day: 2, part: 2, test: false);

        var solution = new Solution();
        var actualSolution = await solution.SolvePart2Async(testInput);

        Assert.Equal(expected: ExpectedSolution, actual: actualSolution);
    }
}