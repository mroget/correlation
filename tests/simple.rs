use correlation::*;

#[test]
#[should_panic]
fn pearson_empty() {
    pearsonr(&vec![], &vec![]);
}

#[test]
#[should_panic]
fn pearson_different_len() {
    pearsonr(&vec![1.,2.,3.], &vec![1., 2.]);
}

#[test]
fn pearson_perfect() {
    let result = pearsonr(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    assert!((result - 1.).abs() < 1e-7);
}



#[test]
#[should_panic]
fn spearman_empty() {
    spearmanr(&vec![], &vec![]);
}

#[test]
#[should_panic]
fn spearman_different_len() {
    spearmanr(&vec![1.,2.,3.], &vec![1., 2.]);
}

#[test]
fn spearman_perfect() {
    let result = spearmanr(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    assert!((result - 1.).abs() < 1e-7);
}


#[test]
#[should_panic]
fn kendall_empty() {
    kendalltau(&vec![], &vec![]);
}

#[test]
#[should_panic]
fn kendall_different_len() {
    kendalltau(&vec![1.,2.,3.], &vec![1., 2.]);
}

#[test]
fn kendall_perfect() {
    let result = kendalltau(&vec![1.,2.,3.], &vec![4., 5., 6.]);
    assert!((result - 1.).abs() < 1e-7);
}