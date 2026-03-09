import "server-only";

import type {
  LessonVerificationResult,
  VerificationMethod,
} from "@/lib/lesson-verification-types";

interface PlaygroundResponse {
  success: boolean;
  stdout: string;
  stderr: string;
}

interface Rule {
  message: string;
  test: (value: string) => boolean;
}

interface OutputVerifier {
  kind: "output";
  successMessage: string;
  sourceRules?: Rule[];
  outputRules: Rule[];
}

interface HiddenTestVerifier {
  kind: "tests";
  successMessage: string;
  sourceRules?: Rule[];
  hiddenTests: string;
}

type LessonVerifier = OutputVerifier | HiddenTestVerifier;

function sourceMatches(pattern: RegExp, message: string): Rule {
  return {
    message,
    test: (code) => pattern.test(code),
  };
}

function outputIncludes(fragment: string, message: string): Rule {
  return {
    message,
    test: (stdout) => normalize(stdout).includes(fragment),
  };
}

function outputMatches(pattern: RegExp, message: string): Rule {
  return {
    message,
    test: (stdout) => pattern.test(normalize(stdout)),
  };
}

function minimumNonEmptyLines(count: number, message: string): Rule {
  return {
    message,
    test: (stdout) => nonEmptyLines(stdout).length >= count,
  };
}

function normalize(value: string) {
  return value.replace(/\r\n/g, "\n").trim();
}

function nonEmptyLines(stdout: string) {
  return normalize(stdout)
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
}

function hiddenTests(body: string): string {
  return `
#[cfg(test)]
mod hidden_tests {
    use super::*;

${body}
}
`;
}

const VERIFIERS: Record<number, LessonVerifier> = {
  1: {
    kind: "output",
    successMessage: "Printed the required greeting and added a second line of context.",
    outputRules: [
      outputIncludes('Hello, Rustacean!', 'Print `Hello, Rustacean!`.'),
      minimumNonEmptyLines(2, "Print at least two non-empty lines."),
    ],
  },
  2: {
    kind: "output",
    successMessage: "Used typed variables, mutability, and shadowing with visible output.",
    sourceRules: [
      sourceMatches(/let\s+age\s*:\s*u8\b/, "Declare `age` as a `u8`."),
      sourceMatches(/let\s+mut\s+score\s*:\s*i32\b/, "Declare `score` as a mutable `i32`."),
      sourceMatches(/let\s+score\s*=\s*score\s*\*\s*2\s*;/, "Shadow `score` to double it."),
    ],
    outputRules: [
      outputIncludes("Age:", "Print the age value."),
      minimumNonEmptyLines(2, "Print at least two non-empty lines."),
    ],
  },
  3: {
    kind: "tests",
    successMessage: "The prime helper works for representative values.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(9));
        assert!(is_prime(97));
    }
`),
  },
  4: {
    kind: "tests",
    successMessage: "The mutable borrow helper updates strings correctly.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let mut value = String::from("Rust");
        append_exclamation(&mut value);
        assert_eq!(value, "Rust!");
    }
`),
  },
  5: {
    kind: "tests",
    successMessage: "The point constructor and distance method behave correctly.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < f64::EPSILON);
        assert_eq!(p2.x, 3.0);
        assert_eq!(p2.y, 4.0);
    }
`),
  },
  6: {
    kind: "tests",
    successMessage: "Coin pattern matching returns the correct values.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert_eq!(value_in_cents(&Coin::Penny), 1);
        assert_eq!(value_in_cents(&Coin::Nickel), 5);
        assert_eq!(value_in_cents(&Coin::Dime), 10);
        assert_eq!(value_in_cents(&Coin::Quarter(String::from("Ohio"))), 25);
    }
`),
  },
  7: {
    kind: "output",
    successMessage: "The frequency counter finds the right counts for repeated words.",
    outputRules: [
      outputIncludes("the: 3", "Count `the` three times."),
      outputIncludes("fox: 2", "Count `fox` twice."),
    ],
  },
  8: {
    kind: "tests",
    successMessage: "The parser handles valid, invalid, and negative values correctly.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert_eq!(parse_positive("42").unwrap(), 42);
        assert!(matches!(parse_positive("-3"), Err(AppError::NegativeNumber(-3))));
        assert!(matches!(parse_positive("nope"), Err(AppError::ParseError(_))));
    }
`),
  },
  9: {
    kind: "tests",
    successMessage: "Trait implementations summarize both article and tweet content.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let article = Article {
            title: String::from("Traits"),
            author: String::from("Ferris"),
            content: String::from("Traits let you define shared behavior."),
        };
        let tweet = Tweet {
            username: String::from("rustacean"),
            content: String::from("Traits are neat!"),
        };

        assert!(article.summarize().contains("Traits"));
        assert!(article.summarize().contains("Ferris"));
        assert!(tweet.summarize().contains("rustacean"));
        assert!(tweet.preview().ends_with("..."));
    }
`),
  },
  10: {
    kind: "tests",
    successMessage: "Lifetime annotations support the intended borrow scenarios.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let left = String::from("abcdef");
        let right = String::from("xyz");
        assert_eq!(longest(left.as_str(), right.as_str()), "abcdef");

        let note = Important { content: "borrowed text" };
        assert_eq!(note.announce(), "borrowed text");
    }
`),
  },
  11: {
    kind: "output",
    successMessage: "The iterator chain and closure produce the expected results.",
    outputRules: [
      outputIncludes("Sum of squares of even numbers: 220", "Compute the expected sum."),
      outputIncludes(
        "Shifted: [11, 12, 13, 14, 15, 16, 17, 18, 19, 20]",
        "Apply the closure across the input list."
      ),
    ],
  },
  12: {
    kind: "tests",
    successMessage: "Shapes expose correct area and perimeter calculations.",
    hiddenTests: hiddenTests(`
    use crate::geometry::{Circle, Rectangle, Shape};

    #[test]
    fn scenario_1() {
        let circle = Circle { radius: 2.0 };
        let rectangle = Rectangle { width: 3.0, height: 4.0 };
        assert!((circle.area() - std::f64::consts::PI * 4.0).abs() < 1e-10);
        assert!((circle.perimeter() - 2.0 * std::f64::consts::PI * 2.0).abs() < 1e-10);
        assert_eq!(rectangle.area(), 12.0);
        assert_eq!(rectangle.perimeter(), 14.0);
    }
`),
  },
  13: {
    kind: "output",
    successMessage: "The smart-pointer examples run and share mutable state correctly.",
    sourceRules: [
      sourceMatches(/Rc::new/, "Use `Rc::new` in the shared-state example."),
      sourceMatches(/RefCell::new/, "Use `RefCell::new` for interior mutability."),
    ],
    outputRules: [
      outputIncludes("Counter: 2", "Show that both owners updated the shared counter."),
      outputIncludes("Cons(1", "Print the linked-list structure."),
    ],
  },
  14: {
    kind: "output",
    successMessage: "The threaded collector runs correctly and includes message-passing code.",
    sourceRules: [
      sourceMatches(/mpsc(::channel)?|channel\s*\(/, "Demonstrate message passing with `std::sync::mpsc`."),
    ],
    outputRules: [
      outputIncludes("Results: [0, 1, 2, 3, 4]", "Collect each thread ID into the shared vector."),
    ],
  },
  15: {
    kind: "tests",
    successMessage: "The async helpers and minimal executor return the expected values.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert_eq!(block_on(fetch_data(7)), "Data for id=7");
        let pair = block_on(async {
            let first = fetch_data(1).await;
            let second = fetch_data(2).await;
            (first, second)
        });
        assert_eq!(pair.0, "Data for id=1");
        assert_eq!(pair.1, "Data for id=2");
    }
`),
  },
  16: {
    kind: "tests",
    successMessage: "The lesson macros expand to the expected collection values.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let scores = map!("a" => 1, "b" => 2);
        assert_eq!(scores.get("a"), Some(&1));
        assert_eq!(scores.get("b"), Some(&2));

        let values = vec_of_strings!["hello", "world"];
        assert_eq!(values, vec![String::from("hello"), String::from("world")]);
    }
`),
  },
  17: {
    kind: "tests",
    successMessage: "The unsafe wrapper preserves behavior while exposing a safe API.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let mut left = 10;
        let mut right = 20;
        unsafe_swap(&mut left, &mut right);
        assert_eq!((left, right), (20, 10));
        assert_eq!(safe_abs(-42), 42);
    }
`),
  },
  18: {
    kind: "tests",
    successMessage: "Task management operations and summary statistics work correctly.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let mut manager = TaskManager::new();
        let first = manager.add("Finish ownership", Priority::High);
        let second = manager.add("Write tests", Priority::Low);

        assert_eq!(first, 1);
        assert_eq!(second, 2);
        assert!(manager.complete(first));
        assert!(!manager.complete(99));
        assert_eq!(manager.list().len(), 2);
        assert_eq!(manager.stats()["total"], 2);
        assert_eq!(manager.stats()["done"], 1);
        assert!(format!("{}", &manager.list()[0]).contains("#1"));
    }
`),
  },
  19: {
    kind: "tests",
    successMessage: "Operator overloading and formatting behave like a vector type should.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let first = Vec2::new(1.0, 2.0);
        let second = Vec2::new(3.0, 4.0);
        assert_eq!(first + second, Vec2::new(4.0, 6.0));
        assert_eq!(first * 2.0, Vec2::new(2.0, 4.0));
        assert_eq!(format!("{}", Vec2::new(1.5, 2.5)), "(1.5, 2.5)");
    }
`),
  },
  20: {
    kind: "tests",
    successMessage: "The builder pattern constructs the expected SQL query.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let query = QueryBuilder::new("orders")
            .where_clause("status = 'open'")
            .where_clause("total > 100")
            .limit(5)
            .build();
        assert_eq!(
            query,
            "SELECT * FROM orders WHERE status = 'open' AND total > 100 LIMIT 5"
        );
    }
`),
  },
  21: {
    kind: "tests",
    successMessage: "The array and string algorithms return correct answers.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(max_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(longest_unique("abcabcbb"), 3);
        assert_eq!(longest_unique("bbbb"), 1);
    }
`),
  },
  22: {
    kind: "tests",
    successMessage: "The linked-list helpers preserve ordering through reversal.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
        list.head = LinkedList::reverse(list.head);
        assert_eq!(list.to_vec(), vec![3, 2, 1]);
    }
`),
  },
  23: {
    kind: "tests",
    successMessage: "The stack and heap exercises handle balanced brackets and top-k frequency.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert!(is_valid("()[]{}"));
        assert!(!is_valid("([)]"));

        let mut result = top_k_frequent(&[1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }
`),
  },
  24: {
    kind: "tests",
    successMessage: "Tree traversal and balance detection work for balanced and unbalanced trees.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let mut root = TreeNode::new(2);
        root.left = Some(TreeNode::new(1));
        root.right = Some(TreeNode::new(3));
        let tree = Some(root);
        assert_eq!(inorder(&tree), vec![1, 2, 3]);
        assert!(is_balanced(&tree));

        let mut skewed = TreeNode::new(1);
        skewed.left = Some(TreeNode::new(0));
        skewed.left.as_mut().unwrap().left = Some(TreeNode::new(-1));
        let skewed_tree = Some(skewed);
        assert!(!is_balanced(&skewed_tree));
    }
`),
  },
  25: {
    kind: "tests",
    successMessage: "Graph traversal and shortest-path calculations are correct.",
    hiddenTests: hiddenTests(`
    use std::collections::HashMap;

    #[test]
    fn scenario_1() {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        graph.insert(0, vec![1, 2]);
        graph.insert(1, vec![3]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);
        assert_eq!(bfs(&graph, 0), vec![0, 1, 2, 3]);

        let mut weighted: HashMap<usize, Vec<(usize, u32)>> = HashMap::new();
        weighted.insert(0, vec![(1, 4), (2, 1)]);
        weighted.insert(1, vec![(3, 1)]);
        weighted.insert(2, vec![(1, 2), (3, 5)]);
        weighted.insert(3, vec![]);
        let dist = dijkstra(&weighted, 0);
        assert_eq!(dist.get(&0), Some(&0));
        assert_eq!(dist.get(&1), Some(&3));
        assert_eq!(dist.get(&3), Some(&4));
    }
`),
  },
  26: {
    kind: "tests",
    successMessage: "Both sorting algorithms produce ordered results.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert_eq!(merge_sort(vec![5, 2, 4, 1, 3]), vec![1, 2, 3, 4, 5]);

        let mut values = vec![9, 4, 7, 1, 3, 2];
        let last = values.len() - 1;
        quick_sort(&mut values, 0, last);
        assert_eq!(values, vec![1, 2, 3, 4, 7, 9]);
    }
`),
  },
  27: {
    kind: "tests",
    successMessage: "The dynamic-programming solutions handle standard benchmark cases.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert_eq!(coin_change(&[1, 2, 5], 11), 3);
        assert_eq!(longest_common_subsequence("ABCBDAB", "BDCAB"), 4);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }
`),
  },
  28: {
    kind: "tests",
    successMessage: "Backtracking, rotated search, and interval merging all return correct results.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert_eq!(generate_permutations(vec![1, 2, 3]).len(), 6);
        assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(merge_intervals(vec![[1, 3], [2, 6], [8, 10], [15, 18]]), vec![[1, 6], [8, 10], [15, 18]]);
    }
`),
  },
  29: {
    kind: "tests",
    successMessage: "The custom iterators produce the expected sequence values.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let fibs: Vec<u64> = Fibonacci::new().take(6).collect();
        assert_eq!(fibs, vec![1, 1, 2, 3, 5, 8]);

        let stepped: Vec<i32> = StepBy { iter: 0..10, step: 3, count: 0 }.collect();
        assert_eq!(stepped, vec![0, 3, 6, 9]);
    }
`),
  },
  30: {
    kind: "tests",
    successMessage: "The study planner returns a valid order and a strong revision bundle.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let order = topological_sort(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]).expect("dag");
        let mut positions = vec![0usize; 4];
        for (index, node) in order.iter().enumerate() {
            positions[*node] = index;
        }
        assert!(positions[0] < positions[1]);
        assert!(positions[0] < positions[2]);
        assert!(positions[1] < positions[3]);
        assert!(positions[2] < positions[3]);
        assert!(topological_sort(2, &[(0, 1), (1, 0)]).is_none());

        let topics = vec![
            Topic { name: "a", hours: 2, value: 3 },
            Topic { name: "b", hours: 3, value: 4 },
            Topic { name: "c", hours: 4, value: 8 },
        ];
        let (score, chosen) = best_revision_plan(&topics, 6);
        assert_eq!(score, 11);
        assert_eq!(chosen.len(), 2);
        assert!(chosen.contains(&"a"));
        assert!(chosen.contains(&"c"));
    }
`),
  },
  31: {
    kind: "output",
    successMessage: "Manual JSON serialization matches the expected shape.",
    outputRules: [
      outputIncludes(
        '{"user_id":1,"name":"Alice","email":"alice@example.com"}',
        "Serialize a user with an email field."
      ),
      outputIncludes(
        '{"user_id":2,"name":"Bob"}',
        "Omit the email field when it is `None`."
      ),
      outputIncludes("serde_json::to_string(&user)?", "Mention the serde shortcut in the output."),
    ],
  },
  32: {
    kind: "tests",
    successMessage: "The in-memory REST simulation supports create, read, list, and delete.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let mut db = InMemoryDb::new();
        let alice = db.create("Alice", "alice@example.com").clone();
        let bob = db.create("Bob", "bob@example.com").clone();
        assert_eq!(alice.id, 1);
        assert_eq!(bob.id, 2);
        assert_eq!(db.get(1).map(|user| user.name.as_str()), Some("Alice"));
        assert_eq!(db.list().len(), 2);
        assert!(db.delete(1));
        assert!(!db.delete(99));
        assert_eq!(db.list().len(), 1);
    }
`),
  },
  33: {
    kind: "output",
    successMessage: "The CLI prints a usable help summary when run without arguments.",
    outputRules: [
      outputIncludes("Usage: tool <command> [args]", "Print the top-level usage line."),
      outputIncludes("add <title> [--priority low|medium|high]", "Document the add command."),
      outputIncludes("done <id>", "Document the done command."),
    ],
  },
  34: {
    kind: "tests",
    successMessage: "The repository abstraction supports lookup, persistence, and deletion.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        let mut repo = UserRepository::new();
        let first = repo.save(User { id: 0, name: "Alice".into(), email: "alice@example.com".into() });
        let second = repo.save(User { id: 0, name: "Bob".into(), email: "bob@example.com".into() });
        assert_eq!(first, 1);
        assert_eq!(second, 2);
        assert_eq!(repo.find(first).map(|user| user.name.as_str()), Some("Alice"));
        assert!(repo.find_by_email("bob@example.com").is_some());
        assert_eq!(repo.all().len(), 2);
        assert!(repo.delete(first));
        assert_eq!(repo.all().len(), 1);
    }
`),
  },
  35: {
    kind: "tests",
    successMessage: "The tested helpers return correct values across hidden cases.",
    sourceRules: [
      sourceMatches(/#\s*\[\s*test\s*\]/, "Include at least one Rust unit test in the lesson."),
    ],
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert_eq!(add(10, -3), 7);
        assert_eq!(divide(9.0, 3.0).unwrap(), 3.0);
        assert_eq!(divide(9.0, 0.0).unwrap_err(), "Division by zero");
        assert!(is_palindrome("Never odd or even"));
        assert_eq!(fibonacci(10), 55);
    }
`),
  },
  36: {
    kind: "tests",
    successMessage: "The capstone validation helpers enforce the expected input rules.",
    hiddenTests: hiddenTests(`
    #[test]
    fn scenario_1() {
        assert!(validate_email("alice@example.com"));
        assert!(!validate_email("not-an-email"));
        assert!(validate_name("Ferris").is_ok());
        assert!(validate_name("").is_err());
        assert!(validate_name(&"a".repeat(101)).is_err());
    }
`),
  },
};

async function runPlayground(code: string, tests: boolean): Promise<PlaygroundResponse> {
  const response = await fetch("https://play.rust-lang.org/execute", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      channel: "stable",
      mode: "debug",
      edition: "2021",
      crateType: "bin",
      tests,
      code,
      backtrace: false,
    }),
    signal: AbortSignal.timeout(20000),
  });

  if (!response.ok) {
    throw new Error(`Rust Playground request failed with HTTP ${response.status}.`);
  }

  return (await response.json()) as PlaygroundResponse;
}

function fail(
  method: VerificationMethod,
  summary: string,
  details: string
): LessonVerificationResult {
  return {
    passed: false,
    method,
    summary,
    details,
    progressSaved: false,
  };
}

function pass(
  method: VerificationMethod,
  summary: string,
  details: string
): LessonVerificationResult {
  return {
    passed: true,
    method,
    summary,
    details,
    progressSaved: false,
  };
}

function evaluateSourceRules(method: VerificationMethod, code: string, rules?: Rule[]) {
  const failures = (rules ?? [])
    .filter((rule) => !rule.test(code))
    .map((rule) => `- ${rule.message}`);

  if (failures.length === 0) {
    return null;
  }

  return fail(
    method,
    "The solution is missing required lesson constructs.",
    failures.join("\n")
  );
}

function buildFailureDetails(result: PlaygroundResponse) {
  const sections = [normalize(result.stdout), normalize(result.stderr)].filter(Boolean);
  return sections.join("\n\n");
}

export async function verifyLessonCode(
  lessonId: number,
  code: string
): Promise<LessonVerificationResult> {
  const verifier = VERIFIERS[lessonId];

  if (!verifier) {
    return fail(
      "hidden-tests",
      "No verifier is configured for this lesson yet.",
      "Add a verifier for this lesson before attempting automatic completion."
    );
  }

  const sourceFailure = evaluateSourceRules(
    verifier.kind === "tests" ? "hidden-tests" : "output-check",
    code,
    verifier.sourceRules
  );

  if (sourceFailure) {
    return sourceFailure;
  }

  try {
    if (verifier.kind === "tests") {
      const result = await runPlayground(`${code}\n${verifier.hiddenTests}`, true);

      if (!result.success) {
        return fail(
          "hidden-tests",
          "The hidden verification tests did not pass.",
          buildFailureDetails(result)
        );
      }

      return pass("hidden-tests", verifier.successMessage, normalize(result.stdout));
    }

    const result = await runPlayground(code, false);
    if (!result.success) {
      return fail(
        "output-check",
        "The code did not compile or run successfully.",
        buildFailureDetails(result)
      );
    }

    const failures = verifier.outputRules
      .filter((rule) => !rule.test(result.stdout))
      .map((rule) => `- ${rule.message}`);

    if (failures.length > 0) {
      const output = normalize(result.stdout);
      return fail(
        "output-check",
        "The program output does not match the lesson requirements yet.",
        `${failures.join("\n")}\n\nOutput:\n${output || "(no output)"}`
      );
    }

    return pass("output-check", verifier.successMessage, normalize(result.stdout));
  } catch (error) {
    return fail(
      verifier.kind === "tests" ? "hidden-tests" : "output-check",
      "Verification could not be completed.",
      error instanceof Error ? error.message : "Unknown verification error."
    );
  }
}
