
use std::cmp::Ordering;

fn sum(l : &Vec<f64>) -> f64 {
    l.iter().fold(0., |acc, val| acc+val)
}


fn mean(l : &Vec<f64>) -> f64 {
    if l.len() == 0 {
        panic!("Cannot calculate the mean of an empty Vec !");
    }
    sum(l)/(l.len() as f64)
}

fn constant(l : &Vec<f64>) -> bool {
    if l.len() == 0 {
        return true;
    }
    let k = l[0];
    for i in 1..l.len() {
        if k != l[i] {
            return false;
        }
    }
    true
}


fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    if a.is_nan() && b.is_nan() {
        return Ordering::Equal;
    }
    if a.is_nan() {
        return Ordering::Greater;
    }
    if b.is_nan() {
        return Ordering::Less;
    }
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    return Ordering::Equal;
} 

#[derive(Debug, PartialEq, Clone, Copy)]
enum Concordance {
    Discordant,
    Concordant,
    Equal,
}

fn concordance(x1 : f64, y1 : f64, x2 : f64, y2 : f64) -> Concordance {
    if x1==x2 && y1==y2 {
        Concordance::Equal
    }
    else if (x1 < x2 && y1 < y2) || (x1 > x2 && y1 > y2) {
        Concordance::Concordant
    }
    else {
        Concordance::Discordant
    }
}


fn rank(l : &Vec<f64>) -> Vec<f64> {
    let n = l.len();
    if n == 0 {
        return Vec::new();
    }
    let mut l2 = (0..n).map(|i| (l[i], i)).collect::<Vec<(f64, usize)>>();
    l2.sort_by(|a, b| cmp_f64(&a.0, &b.0));

    let mut ranking = (0..n).map(|i| (l2[i].0, l2[i].1, i as f64)).collect::<Vec<(f64, usize, f64)>>();
    let mut k = 1;
    let mut h = 1;
    let mut sum = ranking[0].2;
    while k < ranking.len() {
        if ranking[k-1].0 == ranking[k].0 {
            h = h+1;
            sum = sum + ranking[k].2;
        }
        else {
            for i in (k-h)..k {
                ranking[i].2 = sum / (h as f64);
            }
            h = 1;
            sum = ranking[k].2;
        }
        k = k+1;
    }
    for i in (k-h)..k {
        ranking[i].2 = sum / (h as f64);
    }


    ranking.sort_by_key(|x| x.1);
    ranking.into_iter().map(|x| x.2).collect()
}



pub fn pearsonr(x : &Vec<f64>, y : &Vec<f64>) -> f64 {
    //! This function calculate the Pearson correlation coefficient between x and y (<https://en.wikipedia.org/wiki/Pearson_correlation_coefficient>).
    //! </br></br><b>Example</b>
    //! ```
    //! let result = pearsonr(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    //! assert!((result - 1.).abs() < 1e-7);
    //! ```
    assert!(x.len() == y.len());
    let n = x.len();

    let xm = mean(x);
    let ym = mean(y);

    let s1 = sum(&(0..n).map(|i| (x[i]-xm)*(y[i]-ym)).collect());
    let s2 = sum(&(0..n).map(|i| (x[i]-xm).powf(2.)).collect());
    let s3 = sum(&(0..n).map(|i| (y[i]-ym).powf(2.)).collect());

    s1 / ( s2.sqrt() * s3.sqrt() )
}


pub fn spearmanr(x : &Vec<f64>, y : &Vec<f64>) -> f64 {
    //! This function calculate the Spearman's rank correlation coefficient between x and y (<https://en.wikipedia.org/wiki/Spearman's_rank_correlation_coefficient>).
    //! </br></br><b>Example</b>
    //! ```
    //! let result = spearmanr(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    //! assert!((result - 1.).abs() < 1e-7);
    //! ```
    pearsonr(&rank(x), &rank(y))
}

pub fn kendalltau(x : &Vec<f64>, y : &Vec<f64>) -> f64 {
    //! This function calculate the Kendall rank correlation coefficient between x and y (<https://en.wikipedia.org/wiki/Kendall_rank_correlation_coefficient#Tau-a>).
    //! </br>In the presence of ties, the Tau-a variant is calculated.
    //! </br></br><b>Example</b>
    //! ```
    //! let result = kendalltau(&vec![1.,2.,3.], &vec![1., 2., 3.]);
    //! assert!((result - 1.).abs() < 1e-7);
    //! ```
    assert!(x.len() == y.len());
    assert!(x.len() != 0);
    if constant(x) || constant(y) {
        return f64::NAN;
    }
    let n = x.len();
    let mut discordant = 0;
    let mut m = 0;
    for i in 0..n {
        for j in (i+1)..n {
            if concordance(x[i],y[i],x[j],y[j]) == Concordance::Discordant {
                discordant +=1;
            }
            m+=1;
        }
    }

    1. - ((2 * discordant) as f64 / (m as f64))
}


#[cfg(test)]
mod tests_sum {
    use super::*;

    #[test]
    fn sum_empty() {
        let result = sum(&vec![]);
        assert_eq!(result, 0.);
    }

    #[test]
    fn sum_float() {
        let result = sum(&vec![2.3, 1.6, 2.9, -4.8]);
        assert_eq!(result, 2.0);
    }

    #[test]
    #[should_panic]
    fn mean_empty() {
        let result = mean(&vec![]);
        assert_eq!(result, 0.);
    }

    #[test]
    fn mean_float() {
        let result = mean(&vec![2.3, 1.6, 2.9, -4.8]);
        assert_eq!(result, 0.5);
    }

}

#[cfg(test)]
mod tests_float{
    use super::*;

    #[test]
    fn greater() {
        let result = cmp_f64(&0.2, &-0.1);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn lesser() {
        let result = cmp_f64(&0.1, &0.11);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn equal() {
        let result = cmp_f64(&(1./3.), &(1./3.));
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn nan1() {
        let result = cmp_f64(&f64::NAN, &(1./3.));
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn nan2() {
        let result = cmp_f64(&(1./3.), &f64::NAN);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn nan3() {
        let result = cmp_f64(&f64::NAN, &f64::NAN);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn precision() {
        let result = cmp_f64(&(2.3-1.1), &(1.2));
        assert!(result != Ordering::Equal);
    }
}

#[cfg(test)]
mod tests_constant {
    

    #[test]
    fn simple() {
        let result = super::constant(&vec![3.4,5.1,2.6,7.3]);
        assert_eq!(result, false);
    }

    #[test]
    fn empty() {
        let result = super::constant(&vec![]);
        assert_eq!(result, true);
    }

    #[test]
    fn double() {
        let result = super::constant(&vec![2.3, 1.2, 1.2, 3.2]);
        assert_eq!(result, false);
    }

    #[test]
    fn constant() {
        let result = super::constant(&vec![1., 1., 1., 1.]);
        assert_eq!(result, true);
    }
}

#[cfg(test)]
mod tests_rank {
    use super::*;

    #[test]
    fn simple() {
        let result = rank(&vec![3.4,5.1,2.6,7.3]);
        assert_eq!(result, vec![1.,2.,0.,3.]);
    }

    #[test]
    fn empty() {
        let result = rank(&vec![]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn double() {
        let result = rank(&vec![2.3, 1.2, 1.2, 3.2]);
        assert_eq!(result, vec![2.0,0.5,0.5,3.0]);
    }

    #[test]
    fn constant() {
        let result = rank(&vec![1., 1., 1., 1.]);
        assert_eq!(result, vec![1.5, 1.5, 1.5, 1.5]);
    }
}