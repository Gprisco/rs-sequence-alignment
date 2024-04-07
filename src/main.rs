use rs_sequence_alignment::align_sequences;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <sequence1> <sequence2>", args[0]);
        return;
    }

    let seq1 = &args[1];
    let seq2 = &args[2];
    let (aligned_seq1, aligned_seq2) = align_sequences(seq1, seq2);

    println!("{}", aligned_seq1);
    println!("{}", aligned_seq2);
}
