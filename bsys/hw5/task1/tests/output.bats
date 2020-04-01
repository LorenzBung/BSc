#!/usr/bin/env bats


@test "task1: Check that we have a debug output" {
    run stat "$BATS_TEST_DIRNAME/../target/debug/task1"
    [ "$status" -eq 0 ]
}

# Check lines of output

# wc output with white spaces is trimmed by xargs
@test "task1: Output with Zombie must at least 4 Lines long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' | wc -l | xargs"
    [ "$output" -gt 4 ]

}

# wc output with white spaces is trimmed by xargs
@test "task1: Output with to many paras must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 2 3 4 | wc -l | xargs"
    [ "$output" = "1" ]

}


# wc output with white spaces is trimmed by xargs
@test "task1: Output with wrong para must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' y | wc -l | xargs"
    [ "$output" = "1" ]
}

# wc output with white spaces is trimmed by xargs
@test "task1: Output with wrong para must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' -1 | wc -l | xargs"
    [ "$output" = "1" ]
}

# wc output with white spaces is trimmed by xargs
@test "task1: Output with para 0 must be exact 0 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 0 | wc -l | xargs"
    [ "$output" = "0" ]
}

# wc output with white spaces is trimmed by xargs
@test "task1: Output with para 256 must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 256 | wc -l | xargs"
    [ "$output" = "1" ]
}

# wc output with white spaces is trimmed by xargs
@test "task1: Output with para 1 must be exact 4 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 1 | wc -l | xargs"
    [ "$output" = "4" ]
}

# wc output with white spaces is trimmed by xargs
@test "task1: Output with para 16 must be exact 34 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 16 | wc -l | xargs"
    [ "$output" = "34" ]
}

# wc output with white spaces is trimmed by xargs
@test "task1: Output with para 255 must be exact 512 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 255 | wc -l | xargs"
    [ "$output" = "512" ]
}

# Status checks
@test "task1: Output with wrong CHILD_NUMBERS does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 0 "
    [ "$status" = 1 ]
}

@test "task1: Output with wrong CHILD_NUMBERS does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 256 "
    [ "$status" = 1 ]
}

@test "task1: Output with wrong PARAM does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' a "
    [ "$status" = 1 ]
}

@test "task1: Output with to many para does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 2 3 4 "
    [ "$status" = 1 ]
}

@test "task1: Output with standard CHILD_NUMBERS exits with 0" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 4 "
    [ "$status" = 0 ]
}

@test "task1: Output with MIN CHILD_NUMBERS exits with 0" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 1 "
    [ "$status" = 0 ]
}

@test "task1: Output with MAX CHILD_NUMBERS exits with 0" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 255 "
    [ "$status" = 0 ]
}


