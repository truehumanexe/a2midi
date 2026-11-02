
use std::cmp::Ordering;

pub fn argmax_pair<T>(arr: &[T]) -> Option<(usize, T)>
where
    T: PartialOrd + Copy,
{
    arr.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, &value)| (index, value))
}

pub fn argmax<T>(arr: &[T]) -> Option<usize>
where
    T: PartialOrd + Copy,
{
    argmax_pair(arr).map(|(index, _)| index)
}



pub fn argmin_pair<T>(arr: &[T]) -> Option<(usize, T)>
where
    T: PartialOrd + Copy,
{
    arr.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, &value)| (index, value))
}


pub fn argmin<T>(arr: &[T]) -> Option<usize>
where
    T: PartialOrd + Copy,
{
    argmin_pair(arr).map(|(index, _)| index)
}