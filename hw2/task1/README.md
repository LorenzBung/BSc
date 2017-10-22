# Homework hw2 task1

## prepare your task

Run the cargo command to prepare your task1/ directory as a library

## task

Calculate the Hamming difference between two DNA strands by implementing the
library function *hamming_distance(s1: &str, s2: &str) -> Result<usize,
String>*.

A mutation is simply a mistake that occurs during the creation or copying of a
nucleic acid, in particular DNA. Because nucleic acids are vital to cellular
functions, mutations tend to cause a ripple effect throughout the cell. Although
mutations are technically mistakes, a very rare mutation may equip the cell with
a beneficial attribute. In fact, the macro effects of evolution are attributable
by the accumulated result of beneficial microscopic mutations over many
generations.

The simplest and most common type of nucleic acid mutation is a point mutation,
which replaces one base with another at a single nucleotide.

By counting the number of differences between two homologous DNA strands taken
from different genomes with a common ancestor, we get a measure of the minimum
number of point mutations that could have occurred on the evolutionary path
between the two strands.

This is called the 'Hamming distance'.

It is found by comparing two DNA strands and counting how many of the
nucleotides are different from their equivalent in the other string.

    GAGCCTACTAACGGGAT
    CATCGTAATGACGGCCT
    ^ ^ ^  ^ ^    ^^

The Hamming distance between these two DNA strands is 7.
