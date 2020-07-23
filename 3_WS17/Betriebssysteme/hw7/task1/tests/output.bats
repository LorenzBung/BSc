#!/usr/bin/env bats

load test_helper


@test "task1: Check that we have a debug output" {
    run stat "$BATS_TEST_DIRNAME/../target/debug/task1"
    [ "$status" -eq 0 ]
}

# Check lines of output

# wc output with white spaces is trimmed by xargs
@test "task1: Error Output must at least 8 Lines long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 2>&1 | wc -l | xargs"
    assert_range 8 6 10   

}

# wc output with white correct input is trimmed by xargs
@test "task1: Normal Output must be at least 4 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 42 123 | wc -l | xargs"
    assert_range 4 4 6
}

# wc output with white correct input is trimmed by xargs
@test "task1: Help Output must be at least 18 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' -- --help | wc -l | xargs"
    assert_range 18 18 20
}

# timeout loop
@test "task1: Output with wrong pattern" {
    run timeout 2 bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 42 j "
    no_timeout_fail

}


# Status checks
@test "task1: Output with wrong input paras does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' x y z "
    assert_fail
}

@test "task1: Output with wrong PARAM does not crash" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' a a"
    assert_fail
}

#@test "task1: Output with wrong pattern does not crash" {
#    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 2 a "
#    assert_fail
#}





