# rust-bubble-sort

Bubble sort algorithm written in [rust](https://www.rust-lang.org/)

## Description

Bubble sort, sometimes referred to as sinking sort, is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted.

* Worst complexity: n^2
* Average complexity: n^2
* Best complexity: n
* Space complexity: 1
* Method: Exchanging
* Stable: Yes
* Class: Comparison sort

## Setup

Make sure you have [rust](https://www.rust-lang.org/) installed on your machine by following the [getting started guide](https://www.rust-lang.org/learn/get-started)

## Instructions

* Clone this repository `git clone git@github.com:thomaschaplin/rust-bubble-sort.git`
* Change directory `cd rust-bubble-sort`
* Build the application `cargo build`
* Run the application `cargo run`
* Test the application `cargo test`

#### Example output of run:

```
Starting a bubble sort on [5, 4, 3, 2, 1]
Bubble sort completed, here's the result: [1, 2, 3, 4, 5]
```

#### Example output of test:

```
running 1 test
test tests::assert_unsorted_vector_is_sorted_correctly ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
