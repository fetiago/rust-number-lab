Sim, podemos ir para a **Week 2**. Mas eu não iria para `String`/`stdin` ainda.

O feedback do agent está certo no ponto central: seu próximo salto não é “Rust mais avançado”, é **domínio de entrada, funções pequenas, casos de borda e verificação**. Ele também acertou em segurar o “multi-byte modular arithmetic tunnel” por enquanto; isso é interessante, mas ainda exige ferramentas melhores de teste e invariantes. 

Minha decisão: **Week 2 = Functions, Domains, Boundary Cases, and Loops**.

Nada de `String`, nada de `stdin`, nada de `Vec`, nada de iterators ainda.

---

## Week 2 — objetivo

Você vai sair de programas onde tudo fica dentro do `main` para programas assim:

```text
main:
  define values
  call functions
  print results

functions:
  calculate
  classify
  validate
  return values
```

A regra pedagógica é:

```text
Calculation first.
Printing later.
One function, one responsibility.
```

---

## 1. Entre no projeto

```bash
cd ~/rust/rust-number-lab
```

Rode uma verificação geral:

```bash
cargo fmt
cargo check --bins
```

Faça um checkpoint da Week 1.5:

```bash
git add .
git commit -m "Finish week 1.5 numeric arrays and boundaries"
```

Se não tiver Git configurado ainda:

```bash
git config --global user.name "Tiago Peres"
git config --global user.email "your-email@example.com"
```

---

## 2. Crie os arquivos da Week 2

```bash
touch src/bin/w2_01_abs_difference_fn.rs
touch src/bin/w2_02_grade_classifier_fn.rs
touch src/bin/w2_03_valid_grade_domain.rs
touch src/bin/w2_04_caesar_encode_fn.rs
touch src/bin/w2_05_caesar_decode_fn.rs
touch src/bin/w2_06_distance_squared_fn.rs
touch src/bin/w2_07_closest_point_fn.rs
touch src/bin/w2_08_array_transform_while.rs
touch src/bin/w2_09_array_sum_while.rs
touch src/bin/w2_10_boundary_case_table.rs
```

Coloque um esqueleto que compila em todos:

```bash
for file in src/bin/w2_*.rs; do
  cat > "$file" <<'RS'
fn main() {
    // TODO
}
RS
done
```

Cheque:

```bash
cargo check --bins
```

---

# Week 2 exercises

Vou passar **sem solução**, para você tentar primeiro. Depois você me manda o `cat src/bin/w2_*.rs` e eu reviso.

---

## 01 — `abs_difference_fn`

### Concepts

```text
function
i32
return value
absolute difference
```

### Goal

Create a function that receives two `i32` values and returns the absolute distance between them.

Example:

```text
a = 3
b = -10
distance = 13
```

### Restrictions

```text
Use one function named abs_difference.
The function must return i32.
Do not print inside the function.
Do not use mut.
Do not use arrays.
Use .abs() only after trying the manual if version.
```

### Run

```bash
cargo run --bin w2_01_abs_difference_fn
```

---

## 02 — `grade_classifier_fn`

### Concepts

```text
function
u8
if as expression
classification
```

### Goal

Create a function that receives a `u8` grade and returns a classification:

```text
A: 90..100
B: 80..89
C: 70..79
D: 60..69
F: below 60
```

### Restrictions

```text
Function name: classify_grade
Input type: u8
Return type: &'static str
Do not validate grade yet.
Do not use match.
Do not print inside the function.
```

### Important trap

For now, `101` will still become `A` if you only check `grade >= 90`.

That is acceptable in this exercise because validation comes next.

### Run

```bash
cargo run --bin w2_02_grade_classifier_fn
```

---

## 03 — `valid_grade_domain`

### Concepts

```text
domain
validation
bool
function composition
```

### Goal

Create one function that checks whether a grade is valid:

```text
0 <= grade <= 100
```

Since the type is `u8`, the lower bound is already guaranteed. The real check is:

```text
grade <= 100
```

Then use it before classifying the grade.

### Restrictions

```text
Use a function named is_valid_grade.
Use a function named classify_grade.
Do not classify invalid grades.
Do not use Option yet.
Print "invalid grade" for values above 100.
```

### Test values

Manually test:

```text
59
60
69
70
79
80
89
90
100
101
255
```

### Run

```bash
cargo run --bin w2_03_valid_grade_domain
```

---

## 04 — `caesar_encode_fn`

### Concepts

```text
u8
const
function
modular arithmetic
domain assumption
```

### Goal

Create a function that encodes a numeric letter using Caesar cipher.

```text
letter: 0..25
key: 0..25
alphabet size: 26
```

Example:

```text
letter = 24
key = 5
encoded = 3
```

### Restrictions

```text
Function name: caesar_encode
Use u8.
Use const ALPHABET_SIZE: u8 = 26.
Do not validate yet.
Do not use arrays.
Do not use String.
Do not use char.
Do not print inside the function.
```

### Run

```bash
cargo run --bin w2_04_caesar_encode_fn
```

---

## 05 — `caesar_decode_fn`

### Concepts

```text
u8
safe subtraction
modular arithmetic
function
```

### Goal

Create a function that decodes a Caesar-encoded numeric letter.

Example:

```text
encoded = 3
key = 5
decoded = 24
```

### Restrictions

```text
Function name: caesar_decode
Use u8.
Do not use i32.
Do not use wrapping_sub.
Do not use arrays.
Use the safe pattern: encoded + ALPHABET_SIZE - safe_key.
Reduce key with % ALPHABET_SIZE.
```

### Test values

```text
encoded = 3, key = 5 -> 24
encoded = 0, key = 1 -> 25
encoded = 25, key = 25 -> 0
encoded = 10, key = 3 -> 7
```

### Run

```bash
cargo run --bin w2_05_caesar_decode_fn
```

---

## 06 — `distance_squared_fn`

### Concepts

```text
function
i32
geometry
distance squared
```

### Goal

Create a function that receives two points and returns the squared distance between them.

You are not calculating the real distance yet.

Formula:

```text
dx = x2 - x1
dy = y2 - y1
distance_squared = dx * dx + dy * dy
```

### Restrictions

```text
Function name: distance_squared
Use i32.
Return i32.
No sqrt.
No f32.
No f64.
Do not print inside the function.
```

### Suggested signature

```rust
fn distance_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i32
```

### Run

```bash
cargo run --bin w2_06_distance_squared_fn
```

---

## 07 — `closest_point_fn`

### Concepts

```text
function composition
distance squared
comparison
classification
```

### Goal

Given two points, decide which one is closer to the origin.

Use your distance-squared logic.

Example:

```text
point A = (3, 4)   -> 25
point B = (10, 1)  -> 101
A is closer
```

### Restrictions

```text
Use one function named distance_from_origin_squared.
Use one function named closest_point.
Do not use sqrt.
Do not use floats.
Return &'static str from closest_point.
Handle tie.
```

Possible outputs:

```text
"A"
"B"
"Tie"
```

### Run

```bash
cargo run --bin w2_07_closest_point_fn
```

---

## 08 — `array_transform_while`

### Concepts

```text
array
while
mut index
function
u8
```

### Goal

Transform every value in a five-element numeric array using Caesar encoding.

Input:

```text
[7, 4, 11, 11, 14]
```

Key:

```text
3
```

Output:

```text
[10, 7, 14, 14, 17]
```

### Restrictions

```text
Use [u8; 5].
Use while.
Use mut index.
Use a second array for the encoded result.
Do not modify the source array.
Use a function named caesar_encode.
No for loop.
No iterators.
No Vec.
No String.
No char.
```

### Run

```bash
cargo run --bin w2_08_array_transform_while
```

---

## 09 — `array_sum_while`

### Concepts

```text
array
while
accumulator
usize
u32
```

### Goal

Sum all values in a fixed array.

Example:

```text
numbers = [10, 20, 30, 40, 50]
sum = 150
```

### Restrictions

```text
Use [u8; 5] for the array.
Use u32 for the sum.
Use usize for the index.
Use while.
Do not use for.
Do not use iterators.
Do not use .sum().
```

### Reason

This exercise trains a classic accumulator pattern:

```text
start with zero
walk through the array
add each value
stop at len()
```

That pattern comes back later in checksums, statistics, parsing, compression, and cryptography.

### Run

```bash
cargo run --bin w2_09_array_sum_while
```

---

## 10 — `boundary_case_table`

### Concepts

```text
manual verification
boundary cases
function reuse
```

### Goal

Create a file that tests several values manually by printing function results.

This is not automated testing yet. It is a **manual test table**.

### Restrictions

```text
Use at least one function from earlier exercises.
No arrays in the first version.
No loops in the first version.
Print several boundary cases manually.
```

### Suggested target

Use `classify_grade` and `is_valid_grade`.

Print results for:

```text
59
60
69
70
79
80
89
90
100
101
255
```

### Example behavior

```text
grade 59 -> valid: true, classification: F
grade 60 -> valid: true, classification: D
grade 100 -> valid: true, classification: A
grade 101 -> valid: false
```

### Run

```bash
cargo run --bin w2_10_boundary_case_table
```

---

## 3. Workflow recomendado

Faça um exercício por vez.

```bash
code src/bin/w2_01_abs_difference_fn.rs
cargo run --bin w2_01_abs_difference_fn
cargo fmt
cargo check --bins
git add .
git commit -m "Solve w2_01 abs difference function"
```

Depois repete para o próximo.

---

## 4. O que eu quero ver no seu código da Week 2

Quero ver menos disto:

```rust
println!("{}", x * x + y * y);
```

E mais disto:

```rust
let result = some_calculation(...);
println!("{result}");
```

E principalmente:

```rust
fn some_calculation(...) -> ... {
    ...
}
```

A mudança mental é esta:

```text
Week 1:
Can I calculate this?

Week 2:
Can I name this calculation and reuse it safely?
```

---

## 5. Regra dura da Week 2

Não avance para `String` ainda.

A tentação vai aparecer na Caesar cipher. Segura. Primeiro fazemos:

```text
number -> number
array of numbers -> array of numbers
function -> reusable transformation
manual test table -> confidence
```

Depois, na Week 3, aí sim a gente começa a encostar em:

```text
char
ASCII
byte
String vs &str
```

Mas só depois que `u8`, arrays, loops e funções estiverem sólidos.
__________
`cargo check --bins`:

warning: function `main` is never used
  --> src/bin/w2_04_caesar_encode_fn.rs:11:4
   |
11 | fn main() {
   |    ^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `rust-number-lab` (bin "w2_08_array_transform_while") generated 1 warning
warning: function `caesar_encode` is never used
 --> src/bin/w2_04_caesar_encode_fn.rs:7:8
  |
7 | pub fn caesar_encode(letter: u8, key: u8) -> u8 {
  |        ^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `main` is never used
  --> src/bin/w2_04_caesar_encode_fn.rs:11:4
   |
11 | fn main() {
   |    ^^^^

warning: `rust-number-lab` (bin "w2_05_caesar_decode_fn") generated 2 warnings
warning: function `main` is never used
  --> src/bin/w2_02_grade_classifier_fn.rs:17:4
   |
17 | fn main() {
   |    ^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `rust-number-lab` (bin "w2_03_valid_grade_domain") generated 1 warning
warning: function `main` is never used
 --> src/bin/w2_01_abs_difference_fn.rs:5:4
  |
5 | fn main() {
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `rust-number-lab` (bin "w2_06_distance_squared_fn") generated 1 warning
warning: function `main` is never used
  --> src/bin/w2_06_distance_squared_fn.rs:10:4
   |
10 | fn main() {
   |    ^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `main` is never used
 --> src/bin/w2_01_abs_difference_fn.rs:5:4
  |
5 | fn main() {
  |    ^^^^

warning: `rust-number-lab` (bin "w2_07_closest_point_fn") generated 2 warnings
warning: function `main` is never used
 --> src/bin/w2_03_valid_grade_domain.rs:8:4
  |
8 | fn main() {
  |    ^^^^

warning: function `classify_grade` is never used
 --> src/bin/w2_02_grade_classifier_fn.rs:1:8
  |
1 | pub fn classify_grade(n: u8) -> &'static str {
  |        ^^^^^^^^^^^^^^

warning: function `main` is never used
  --> src/bin/w2_02_grade_classifier_fn.rs:17:4
   |
17 | fn main() {
   |    ^^^^

warning: `rust-number-lab` (bin "w2_10_boundary_case_table") generated 4 warnings (1 duplicate)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
