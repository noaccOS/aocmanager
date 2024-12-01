import argv
import gleam/io
import gleam/list
import gleam/option
import gleam/result
import gleam/string
import gleeunit/should
import simplifile
import solution

type Variant {
  A
  B
  Both
  SamplesA
  SamplesB
  SamplesBoth
}

fn input() -> String {
  simplifile.read("assets/input") |> result.unwrap("")
}

fn read_samples(variant: Variant) -> List(#(String, String)) {
  let assert option.Some(subdir) = case variant {
    SamplesA -> option.Some("a")
    SamplesB -> option.Some("b")
    _ -> option.None
  }

  let samples_dir = "assets/samples/" <> subdir <> "/"

  simplifile.read_directory(samples_dir)
  |> result.unwrap([])
  |> list.map(fn(sample) {
    let input =
      simplifile.read(samples_dir <> sample <> "/input") |> result.unwrap("")
    let expected =
      simplifile.read(samples_dir <> sample <> "/result") |> result.unwrap("")

    #(input, expected)
  })
}

fn run_samples(variant: Variant) {
  let assert option.Some(solution_variant) = case variant {
    SamplesA -> option.Some(solution.solution_a)
    SamplesB -> option.Some(solution.solution_b)
    _ -> option.None
  }
  read_samples(variant)
  |> list.map(fn(sample) {
    let #(input, expected) = sample

    solution_variant(input)
    |> should.equal(expected)
  })

  Nil
}

pub fn main() {
  let variant = case argv.load().arguments {
    [] -> option.Some(Both)
    [x] ->
      case x |> string.lowercase() {
        "a" -> option.Some(A)
        "b" -> option.Some(B)
        "both" -> option.Some(Both)
        "samples" -> option.Some(SamplesBoth)
        "samples-a" -> option.Some(SamplesA)
        "samples-b" -> option.Some(SamplesB)
        _ -> option.None
      }
    _ -> option.None
  }

  case variant {
    option.Some(A) -> input() |> solution.solution_a() |> io.println()
    option.Some(B) -> input() |> solution.solution_b() |> io.println()
    option.Some(Both) -> {
      let input = input()
      "Solution A:" |> io.println()
      input |> solution.solution_a() |> io.println()

      io.println("")

      "Solution B:" |> io.println()
      input |> solution.solution_b() |> io.println()
    }
    option.Some(SamplesBoth) -> {
      "Samples A" |> io.println()
      SamplesA |> run_samples()

      io.println("")

      "Samples B" |> io.println()
      SamplesB |> run_samples()
    }
    option.Some(samples_a_or_b) -> samples_a_or_b |> run_samples()
    option.None -> Nil
  }
}
