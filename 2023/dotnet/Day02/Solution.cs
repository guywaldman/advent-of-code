using AdventOfCode.Common;
using Pidgin;
using static Pidgin.Parser;

namespace AdventOfCode.Day02;

public enum CubeColor
{
    Red,
    Green,
    Blue
};

public record GameStatistics(IDictionary<CubeColor, int> CubeCount)
{
    public IDictionary<CubeColor, int> CubeCount { get; set; } = CubeCount;
}

public record Game(int Id, IEnumerable<IDictionary<CubeColor, int>> Rounds)
{
    public int Id { get; set; } = Id;
    public IEnumerable<IDictionary<CubeColor, int>> Rounds { get; set; } = Rounds;
}

public class Solution: IDaySolution
{
    public Task<string> SolvePart1Async(string input)
    {
        var gameStatistics = new GameStatistics(new[] { (CubeColor.Red, 12), (CubeColor.Green, 13), (CubeColor.Blue, 14) }
            .ToDictionary(x => x.Item1, x => x.Item2));

        var possibleGameIdsSum = input
            .Split(Environment.NewLine)
            .Select(line => ParseGame(line))
            .Where(game => game.Rounds.All(x => x.All(x => x.Value <= gameStatistics.CubeCount[x.Key])))
            .Select(game => game.Id)
            .Sum();

        return Task.FromResult(possibleGameIdsSum.ToString());
    }

    public Task<string> SolvePart2Async(string input)
    {
        throw new NotImplementedException();
    }

    public static Game ParseGame(string line)
    {
        var gameRoundParser = GameRoundParser();
        var multiRoundParser = gameRoundParser.Separated(Char(';').Then(WhitespaceParser()));

        var parser = Map((_, _, GameId, _, _, Game) => (GameId, Game),
            String("Game"), WhitespaceParser(), Digit.AtLeastOnceString().Select(int.Parse), Char(':'), WhitespaceParser(), multiRoundParser);
        var result = parser.ParseOrThrow(line);

        var gameRound = result.Game.Select(x => x.ToDictionary());
        var game = new Game(result.GameId, gameRound);

        return game;
    }

    public static IEnumerable<(CubeColor Color, int Count)> ParseGameRound(string round)
    {
        var parser = GameRoundParser();
        var parserResult = parser.ParseOrThrow(round);
        return parserResult;
    }

    private static Parser<char, IEnumerable<(CubeColor, int)>> GameRoundParser()
    {
        var numParser = Digit.AtLeastOnceString().Select(int.Parse);
        numParser.Then(Whitespace.AtLeastOnce());
        var colorParser = 
            String("red").Select(_ => CubeColor.Red)
            .Or(String("green").Select(_ => CubeColor.Green))
            .Or(String("blue").Select(_ => CubeColor.Blue));

        var cubeCountParser = Map((num, _, color) => (color, num), numParser, WhitespaceParser(), colorParser);
        var parser = cubeCountParser.Separated(Char(',').Then(WhitespaceParser()));
        return parser;
    }

    private static Parser<char, Unit> WhitespaceParser() => Whitespace.AtLeastOnce().IgnoreResult();
}