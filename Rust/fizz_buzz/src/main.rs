type Rule = fn(u32) -> bool;
type Builder = fn(u32) -> &'static str;

const RULES: [(Rule, Builder); 2] = [(|i| i % 3 == 0, |_| "Fizz"), (|i| i % 5 == 0, |_| "Buzz")];

fn main() {
    let result: Vec<_> = (1..=100).map(build_number).collect();

    println!("{}", result.join("\n"));
}

fn build_number(n: u32) -> String {
    let result = RULES.iter().filter(|r| r.0(n)).fold(None, |acc, rule| {
        let mut acc: String = acc.unwrap_or_default();
        acc.push_str(rule.1(n));
        Some(acc)
    });

    result.unwrap_or_else(|| n.to_string())
}
