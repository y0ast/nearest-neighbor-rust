#![feature(portable_simd)]
#![feature(array_chunks)]

mod heap;
mod io;

use heap::Result;
use rayon::prelude::*;
use std::collections::BinaryHeap;
use std::simd::*;
use std::time::Instant;

fn l2(a: &[f32], b: &[f32]) -> f32 {
    let block_a = a.array_chunks::<8>().map(|&a_i| Simd::from_array(a_i));
    let block_b = b.array_chunks::<8>().map(|&b_i| Simd::from_array(b_i));

    let total = block_a
        .zip(block_b)
        .fold(Simd::splat(0.0), |acc, (a_i, b_i)| {
            let diff = a_i - b_i;

            diff.mul_add(diff, acc)
        });

    total.reduce_sum()
}

fn linear_search(q: &[f32], base: &[Vec<f32>], k: usize) -> Vec<Result> {
    let mut heap: BinaryHeap<_> = base
        .iter()
        .take(k)
        .enumerate()
        .map(|(id, row)| Result {
            id: id as i32,
            dist: l2(q, row),
        })
        .collect();

    let mut max = heap.pop().unwrap();
    for (id, row) in base.iter().enumerate().skip(k) {
        let dist = l2(q, row);
        if dist < max.dist {
            heap.push(Result {
                dist,
                id: id as i32,
            });
            max = heap.pop().unwrap();
        }
    }
    heap.push(max);
    heap.into_vec()
}

fn recall(results: &[Vec<Result>], gt: &[Vec<i32>]) -> f32 {
    let correct: i32 = results
        .iter()
        .zip(gt.iter())
        .map(|(result, label)| result.iter().any(|r| r.id == label[0]))
        .map(|x| x as i32)
        .sum();

    (correct as f32) / (results.len() as f32)
}

fn main() {
    // siftsmall
    println!("Running on siftsmall");
    let base = io::read_vecs::<f32>("data/siftsmall/siftsmall_base.fvecs");
    let query = io::read_vecs::<f32>("data/siftsmall/siftsmall_query.fvecs");
    let gt = io::read_vecs::<i32>("data/siftsmall/siftsmall_groundtruth.ivecs");

    // sift
    // println!("Running on sift");
    // let base = io::read_vecs::<f32>("data/sift/sift_base.fvecs");
    // let query = io::read_vecs::<f32>("data/sift/sift_query.fvecs");
    // let gt = io::read_vecs::<i32>("data/sift/sift_groundtruth.ivecs");

    let k = 5;
    let r = 10;
    let mut times = Vec::new();
    let mut recalls = Vec::new();

    for _ in 0..r {
        let now = Instant::now();
        let result = query
            .par_iter()
            .map(|q| linear_search(q, &base, k))
            .collect::<Vec<Vec<Result>>>();

        times.push(now.elapsed().as_secs_f32());
        recalls.push(recall(&result, &gt));
    }

    let avg_recall = recalls.iter().sum::<f32>() / recalls.len() as f32;
    let min_time = times.iter().cloned().reduce(f32::min).unwrap();
    let max_time = times.iter().cloned().reduce(f32::max).unwrap();
    let avg_time = times.iter().sum::<f32>() / times.len() as f32;
    let avg_time_per_query_ms = avg_time / query.len() as f32 * 1000.0;

    println!("===========================================");
    println!("Config - k: {k}");
    println!("Runs: {r}, Avg Recall: {avg_recall}");
    println!("Best: {min_time:.2}s, Worst: {max_time:.2}s, Avg: {avg_time:.2}s");
    println!("Avg per query: {avg_time_per_query_ms:.2}ms");
    println!("All times: {times:?}");
    println!("===========================================");
}
