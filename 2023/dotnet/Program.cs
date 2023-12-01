using AdventOfCode.Common;
using System.CommandLine;
using System.Diagnostics;

var rootCommand = new RootCommand("Advent of Code in .NET");

var dayOption = new Option<int>(
    name: "--day",
    description: "Day to run"
);
dayOption.AddAlias("-d");
dayOption.IsRequired = true;
rootCommand.AddOption(dayOption);

var partOption = new Option<int>(
    name: "--part",
    description: "Part to run"
);
partOption.AddAlias("-p");
dayOption.IsRequired = true;
rootCommand.AddOption(partOption);

var inputOption = new Option<string?>(
    name: "--input",
    description: "Input file to use"
);
inputOption.AddAlias("-i");
inputOption.IsRequired = false;
rootCommand.AddOption(inputOption);

rootCommand.SetHandler(async (int day, int part, string? input) =>
{
    var dayIdentifier = day.ToString("00");
    var actualInput = input ?? InputUtils.GetInputPath(day, part);
    Console.WriteLine($"Running solution for day {dayIdentifier}, part {part} with input: {actualInput}");

    if (!File.Exists(actualInput))
    {
        throw new ArgumentException($"Input file {actualInput} does not exist");
    }

    var solutionType = Type.GetType($"AdventOfCode.Day{dayIdentifier}.Solution");
    if (solutionType == null)
    {
        throw new ArgumentException($"Solution for day {dayIdentifier} does not exist");
    }

    var solver = Activator.CreateInstance(solutionType) as IDaySolution;
    if (solver == null)
    {
        throw new Exception($"Solution for day {dayIdentifier} does not implement IDaySolution");
    }

    var inputText = await InputUtils.ReadInputAsync(day, part);

    Func<string, Task<string>> solveFn = part switch
    {
        1 => solver.SolvePart1Async,
        2 => solver.SolvePart2Async,
        _ => throw new ArgumentException($"Invalid part {part}")
    };

    var stopwatch = Stopwatch.StartNew();
    var solution = await solveFn.Invoke(inputText);
    stopwatch.Stop();

    Console.WriteLine($"Solution took {stopwatch.ElapsedMilliseconds}ms");

    Console.WriteLine("--------");
    Console.WriteLine("SOLUTION:");
    Console.WriteLine();
    Console.WriteLine(solution);
}, dayOption, partOption, inputOption);

await rootCommand.InvokeAsync(args);