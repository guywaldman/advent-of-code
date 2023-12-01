# Advent of Code 2023 (.NET)

## Prerequisites

- .NET 8 SDK

You could optionally use Visual Studio.

## Starting a new day

- Create a new `DayX` folder (replace `X` with `01`, `13` etc. per the day number)
- Create a new `Solution` class which implements `Common.IDaySolution`
- Add test cases in `Solutions.Tests`
- Add the inputs in `Inputs/` (e.g., `Day01_Part1.txt`, `Day01_Part1.Test.txt`)

## Running a solution

```shell
dotnet run -- --day 1 --part 2
```

> Replace with the day and part you desire to run the solution for.

## Running tests

Run tests using Visual Studio, or on the commandline with the .NET CLI:

```shell
dotnet test
```

> Optionally, use `dotnet test --filter Day01` for testing specific days.  