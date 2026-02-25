# Correlation
A very simple crate that implements the calculation of correlation coefficients.

# Usage
This crate implements:
 - The Pearson correlation coefficient (<https://en.wikipedia.org/wiki/Pearson_correlation_coefficient>).
 - The Spearman's rank correlation coefficient (<https://en.wikipedia.org/wiki/Spearman's_rank_correlation_coefficient>).
 - The Kendall rank correlation coefficient (<https://en.wikipedia.org/wiki/Kendall_rank_correlation_coefficient#Tau-a>).

## Pearson
```[rust]
use correlation::pearsonr;

fn main() {
    let result = pearsonr(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    let expected = 1.;
    assert!((result - expected).abs() <= 1e-7);
}
```

## Spearman
```[rust]
use correlation::spearmanr;

fn main() {
    let result = spearmanr(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    let expected = 1.;
    assert!((result - expected).abs() <= 1e-7);
}
```

## Kendall (variant Tau-A)
```[rust]
use correlation::kendalltau;

fn main() {
    let result = kendalltau(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    let expected = 1.;
    assert!((result - expected).abs() <= 1e-7);
}
```

# Tests
These functions have been tested against their scipy.stats counterparts.

# TODO
 - Add Kendall Tau-B variant
 - Add Kendall Tau-C variant
