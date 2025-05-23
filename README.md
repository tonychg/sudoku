# Sudoku

Simple Sudoku generator and solver. I develop this tool to learn Rust.

## Build from source

```bash
cargo build --release
```

## Usage

* Simple generation

```bash
sudoku generate
11336784070861000209
+-------+-------+-------+
| 6 5 3 | 9 2 8 | 7 1 4 |
| 2 9 1 | 3 7 4 | 5 6 8 |
| 4 8 7 | 5 6 1 | 2 9 3 |
+-------+-------+-------+
| 9 1 4 | 6 3 7 | 8 5 2 |
| 7 2 5 | 8 4 9 | 6 3 1 |
| 3 6 8 | 1 5 2 | 4 7 9 |
+-------+-------+-------+
| 8 3 2 | 7 1 6 | 9 4 5 |
| 1 7 9 | 4 8 5 | 3 2 6 |
| 5 4 6 | 2 9 3 | 1 8 7 |
+-------+-------+-------+

+-------+-------+-------+
|       | 9     |     4 |
| 2 9   | 3     |   6 8 |
|       | 5 6   | 2 9 3 |
+-------+-------+-------+
|     4 |       |       |
|   2   | 8 4 9 |       |
|   6   |       |   7   |
+-------+-------+-------+
|   3   | 7     |       |
|       |   8 5 |       |
|     6 |   9   |   8   |
+-------+-------+-------+
```

* Set count of starting numbers and export to raw string

```bash
sudoku generate -n 28 -r
9:13774580503900678126:413689572826457391579321846145976283798132654362845917231798465957264138684513729
9:13774580503900678126:010000502006057000579000800000906000098100600300800907200008065007000030080003000
```

* Solve a grid from stdin

```bash
echo -n "9:7074261304826435246:000100000456000007070060000000400962000210708062005000003000800000703210024001005" | sudoku solve -s
0
+-------+-------+-------+
| 2 3 8 | 1 5 7 | 6 4 9 |
| 4 5 6 | 3 2 9 | 1 8 7 |
| 9 7 1 | 8 6 4 | 5 2 3 |
+-------+-------+-------+
| 5 1 7 | 4 3 8 | 9 6 2 |
| 3 4 9 | 2 1 6 | 7 5 8 |
| 8 6 2 | 9 7 5 | 4 3 1 |
+-------+-------+-------+
| 1 9 3 | 5 4 2 | 8 7 6 |
| 6 8 5 | 7 9 3 | 2 1 4 |
| 7 2 4 | 6 8 1 | 3 9 5 |
+-------+-------+-------+
```

* Seed can be used to re-generate a grid

```bash
sudoku generate 7074261304826435246
7074261304826435246
+-------+-------+-------+
| 2 3 8 | 1 5 7 | 6 4 9 |
| 4 5 6 | 3 2 9 | 1 8 7 |
| 9 7 1 | 8 6 4 | 5 2 3 |
+-------+-------+-------+
| 5 1 7 | 4 3 8 | 9 6 2 |
| 3 4 9 | 2 1 6 | 7 5 8 |
| 8 6 2 | 9 7 5 | 4 3 1 |
+-------+-------+-------+
| 1 9 3 | 5 4 2 | 8 7 6 |
| 6 8 5 | 7 9 3 | 2 1 4 |
| 7 2 4 | 6 8 1 | 3 9 5 |
+-------+-------+-------+

+-------+-------+-------+
|       | 1     |       |
| 4 5 6 |       |     7 |
|   7   |   6   |       |
+-------+-------+-------+
|       | 4     | 9 6   |
|       | 2 1   | 7   8 |
|   6 2 |     5 |       |
+-------+-------+-------+
|     3 |       | 8     |
|       | 7   3 |   1   |
|   2 4 |     1 |     5 |
+-------+-------+-------+
```
