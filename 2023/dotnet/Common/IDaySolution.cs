namespace AdventOfCode.Common;

public interface IDaySolution
{
    Task<string> SolvePart1Async(string input);
    Task<string> SolvePart2Async(string input);
}
