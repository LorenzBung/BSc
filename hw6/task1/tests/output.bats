#!/usr/bin/env bats


@test "task1: Check that we have a debug output" {
    run stat "$BATS_TEST_DIRNAME/../target/debug/task1"
    [ "$status" -eq 0 ]
}

# Check lines of output


# wc output with white spaces is trimmed by xargs
@test "task1: Output with to no paras must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 1 | wc -l | xargs"
    [ "$output" = "1" ]

}

# wc output with white spaces is trimmed by xargs
@test "task1: Output with one para must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' -1 | wc -l | xargs"
    [ "$output" = "1" ]

}


# wc output with white spaces is trimmed by xargs
@test "task1: Output with correct para must be exact 3 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 2 3 4 5 6 7 8 9 | wc -l | xargs"
    [ "$output" = "3" ]
}


# Check results

@test "task1: 1 -2 3 -4 5" {
    run "$BATS_TEST_DIRNAME/../target/debug/task1" 1 -2 3 -4 5
    [[ "${lines[0]}" =~ "sending to childs: 1 -2 3 -4 5" ]]
    [[ "${lines[1]}" =~ "Sum = 3" ]]
    [[ "${lines[2]}" =~ "Mul = 120" ]]
}

@test "task1: 1 -2 3 -4 5 100 -400 5332 3290 -22 -4646 -1 -1" {
    run "$BATS_TEST_DIRNAME/../target/debug/task1" 1 -2 3 -4 5 100 -400 5332 3290 -22 -4646 -1 -1
    [[ "${lines[0]}" =~ "sending to childs: 1 -2 3 -4 5 100 -400 5332 3290 -22 -4646 -1 -1" ]]
    [[ "${lines[1]}" =~ "Sum = 3655" ]]
    [[ "${lines[2]}" =~ "Mul = -8606551312128000000" ]]
}

@test "task1: 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20" {
    run "$BATS_TEST_DIRNAME/../target/debug/task1" 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20
    [[ "${lines[0]}" =~ "sending to childs: 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20" ]]
    [[ "${lines[1]}" =~ "Sum = 210" ]]
    [[ "${lines[2]}" =~ "Mul = 2432902008176640000" ]]
}

# Status checks
@test "task1: Output with wrong args (1) does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' a b c "
    [ "$status" = 1 ]
}

# Status checks
@test "task1: Output with wrong args (2) does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 1 - 2 "
    [ "$status" = 1 ]
}

@test "task1: Overflow Output does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 10000 10000 10000 10000 10000"
    [ "$status" = 1 ]
}
