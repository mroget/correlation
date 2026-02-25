use correlation::pearsonr;

fn main() {
    let result = pearsonr(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    let expected = 1.;
    assert!((result - expected).abs() <= 1e-7);
}