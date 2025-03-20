var rules = new (Func<int, bool> Rule, Func<int, string> Builder)[]
{
    (i => i % 3 == 0, _ => "Fizz"),
    (i => i % 5 == 0, _ => "Buzz"),
};

var result = string.Join("\n", FizzBuzz(100));

Console.WriteLine(result);

string[] FizzBuzz(int n)
{
    return Enumerable.Range(1, n).Select(BuildNumber).ToArray();

    string BuildNumber(int n)
    {
        var result = rules
            .Where(r => r.Rule(n))
            .Aggregate("", (acc, rule) => acc += rule.Builder(n));

        return result == "" ? n.ToString() : result;
    }
}
