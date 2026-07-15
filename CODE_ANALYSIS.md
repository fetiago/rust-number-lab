# Rust Number Lab — Source Code Analysis

## Scope

This review covers the Rust exercises in `src/main.rs` and `src/bin/`, from `ex01.rs` through the Week 15 exercises. It evaluates four things:

1. correctness;
2. numeric and memory safety;
3. idiomatic Rust;
4. value as a learning progression.

This is a snapshot of the project at its current beginner-learning stage. It does not judge the exercises as if they were production software.

## Honest Overall Assessment

The project shows real progress rather than disconnected copying. The exercises form a recognizable path: basic arithmetic, conditions, coordinate geometry, modular arithmetic, integer boundaries, arrays, indexing, mutation, and manual traversal.

Your code is generally clear and small. You often choose names that expose the calculation, such as `dx`, `dy`, `square_distance`, `remaining_until_max`, `middle`, and `safe_key`. That is an important strength: you are trying to model the problem, not merely make syntax compile.

The main weakness is that you often prove correctness only for the current literal values. Several programs work for their examples but fail or become ambiguous near the boundaries of their declared numeric types. The next stage is therefore not advanced Rust. It is learning to state an input domain, test its edges, and distinguish “works for this example” from “correct for every permitted input.”

## Strengths

### 1. You decompose problems into meaningful steps

The geometry exercises calculate `dx` and `dy` before using them. The modular exercises name the remaining capacity before overflow. The array exercises name indices and derive them from `len()`.

This improves readability and makes mistakes easier to locate. It is a better beginner habit than compressing everything into clever one-line expressions.

### 2. You recognize dangerous operations and guard them

Examples include:

- checking `dx == 0` before slope division;
- checking coordinate axes before quadrants;
- comparing unsigned operands before subtraction;
- checking an array index against `array.len()` in the Caesar loop;
- reducing the Caesar key with `% ALPHABET_SIZE` in the challenge version.

`w15_06_safe_subtraction_u8.rs` is a particularly clean improvement. Its condition establishes that the chosen subtraction cannot underflow, including when the operands are equal.

### 3. You explore the reason behind language features

The modular wraparound exercises do not stop at calling `wrapping_add()`. You also attempt to derive the operation manually. That curiosity can lead to a strong understanding of integer representation, carrying, overflow, and machine arithmetic.

### 4. You are building useful control-flow fundamentals

The project practices `if`, `else if`, block expressions, mutable variables, shadowing, `while`, and fixed-size arrays. These are appropriate fundamentals, and the later exercises reuse earlier concepts rather than abandoning them.

### 5. Your code is memory-safe

There is no `unsafe` Rust. The direct array indexing uses fixed indices or a loop condition that keeps `index < array.len()`. The current code does not reveal a memory-safety problem.

## Weaknesses

### 1. Input assumptions usually remain implicit

Examples:

- The clock exercise assumes non-negative time movement.
- The Caesar exercises assume numeric letters are within `0..26`.
- The grade classifier implicitly assumes grades stop at 100, but its `else` branch accepts values through 255 as `A`.
- The height formatter uses `u8`, silently limiting heights to 255 cm.
- Geometry calculations assume coordinates small enough that subtraction and squaring do not overflow `i32`.

The type alone does not always express the real domain. A value can be a valid `u8` and still be an invalid grade or alphabet position.

### 2. Numeric overflow reasoning is uneven

You clearly understand simple `u8` wrapping, but similar risks remain in other exercises:

- signed subtraction and negation in the one-dimensional distance exercise;
- coordinate squaring and addition in distance comparisons;
- adding two arbitrary `u8` array elements;
- adding the high bytes in the manual multi-byte arithmetic exercise;
- adding an unrestricted Caesar key before applying modulo.

This is not a reason to add complex error handling everywhere. It is a reason to write down the valid range for each exercise and test its boundaries.

### 3. Some names overstate what the calculation means

`ex15.rs` calls subtraction of a minimum “normalization.” It is more precisely an offset or translation unless a complete normalization range is defined.

Precise naming matters because names become part of your mental model. A mathematically inaccurate name can make a correct expression teach the wrong concept.

### 4. Output and calculation are sometimes too closely coupled

The first version of safe subtraction printed from every branch and repeated the arithmetic. You corrected this well by calculating `diff` first and printing once.

That same lesson applies elsewhere: calculate one result, then display it. This reduces duplicated logic and makes later testing easier.

### 5. There is almost no systematic verification

Most binaries contain one hardcoded example. One successful example is weak evidence, especially for conditions and overflow-sensitive arithmetic.

Even without Cargo installed, you can improve verification by writing a small table in comments before coding:

- ordinary case;
- equality case;
- smallest permitted value;
- largest permitted value;
- just below and just above a boundary.

## Urgent Attention

### 1. Pause the multi-byte modular arithmetic tunnel

`w15_05_modular_wraparound_madness.rs` is the most fragile source file.

The high-byte additions can overflow. The modular reduction subtracts the modulus only once, which is insufficient when the intermediate value is at least twice the modulus. The current values also avoid both a carry and a reduction, so they do not exercise the difficult code.

The exploration is valuable, but it is ahead of the project’s current verification skills. Do not expand it yet. Return after practicing functions, repeated test cases, and numeric invariants.

### 2. Separate allowed machine values from allowed problem values

Start writing constraints such as:

- `0 <= grade <= 100`;
- `0 <= letter < ALPHABET_SIZE`;
- `0 <= hour < 24`;
- coordinates must be small enough for the chosen arithmetic type.

This single habit will improve nearly every current exercise.

### 3. Test branch boundaries deliberately

For a threshold like 60, test 59, 60, and 61. For an alphabet size of 26, test 0, 25, and an invalid 26. For subtraction, test less than, equal to, and greater than.

Your code already shows the ability to construct correct branches. The missing skill is proving that the branch boundaries match the intended rules.

## Unaware Potential

### 1. You appear naturally interested in representations and invariants

Your repeated attention to `u8`, wraparound, carrying, arrays, indices, and Caesar transformations points toward a deeper interest in how data is represented and transformed.

That can later connect well to:

- binary representation and bitwise operations;
- parsers and encoders;
- checksums and simple cryptographic building blocks;
- image and audio byte processing;
- embedded programming;
- network protocols and serialization.

The potential is not that you should jump into those topics now. It is that your current curiosity has a coherent direction.

### 2. You are developing algorithmic thinking before abstraction

The manual Caesar loop demonstrates indexing, state change, termination, and output construction. Although iterators will later express this more idiomatically, understanding the manual mechanics first can make those abstractions meaningful instead of magical.

### 3. Your corrections show good learning responsiveness

The safe subtraction exercise improved from nested branches with repeated output into one block expression and one final print. That is evidence that you can simplify code while preserving the reasoning behind it. This skill will matter more than memorizing idioms.

## Suggestions

### Immediate next step: repeat array traversal once

Create one small exercise that transforms every element in a five-element `u8` array using a `while` loop.

Constraints:

- retain the source array;
- write results into a second array;
- use `index < array.len()`;
- perform an operation that is safe for the declared input range;
- print only after the transformation is complete.

Do not introduce iterators yet.

### After that: extract one function

Take a completed calculation such as safe unsigned difference and move only that calculation into a function. Keep input/output in `main`.

This will teach separation of calculation from presentation and will make it easier to try multiple examples without duplicating logic.

### Then: learn simple tests conceptually

Even if your notebook cannot support the Rust toolchain right now, start designing cases as if each were a test:

| Concept | Cases to consider |
|---|---|
| Safe subtraction | `a < b`, `a == b`, `a > b`, `0`, `255` |
| Grade boundaries | `59`, `60`, `69`, `70`, `79`, `80`, `89`, `90`, `100`, `101` |
| Caesar position | `0`, `25`, wraparound, invalid `26` |
| Array indexing | first, middle, last, attempted index equal to `len()` |

### Postpone for now

Do not prioritize:

- custom multi-byte number types;
- generic numeric functions;
- traits;
- iterator-heavy rewrites;
- optimization;
- production-grade error architecture.

Those topics will be easier and more useful after functions, loops, domains, and boundary tests are comfortable.

## Recommended Learning Path

1. Finish manual array traversal and transformation.
2. Extract small pure calculation functions.
3. Practice multiple cases for each function.
4. Learn `for` loops and compare them with the existing `while` loops.
5. Learn basic `Option` for operations that may not have a valid result.
6. Revisit modular arithmetic with explicit input constraints.
7. Only then return to multi-byte carrying and reduction.

## Final Feedback

You are not struggling with Rust syntax as much as you are beginning to meet the deeper problem of programming: defining exactly what values mean and proving that operations remain valid.

Your strongest qualities are decomposition, curiosity about numeric behavior, and willingness to work through mechanics manually. Your main risk is spending too much energy on one difficult representation problem before acquiring the tools needed to verify it.

The objective now is not to make the project more advanced. The smallest useful goal is to make each small exercise correct over a clearly stated domain. Do that consistently, then move from calculations in `main` to small functions and multiple test cases.

