namespace AdventOfCode.Common;

public class InputUtils
{
    public static string GetInputPath(int day, int part, bool test = false)
    {
        var dayIdentifier = day.ToString("00");
        var partIdentifier = part.ToString("0");
        var inputPath = test
            ? Path.Join("Inputs", $"Day{dayIdentifier}_Part{partIdentifier}.Test.txt")
            : Path.Join("Inputs", $"Day{dayIdentifier}_Part{partIdentifier}.txt");
        return inputPath;
    }

    public async static Task<string> ReadInputAsync(int day, int part, bool test = false)
    {
        var inputPath = GetInputPath(day, part, test);
        var inputText = await File.ReadAllTextAsync(inputPath);
        return inputText;
    }
}
