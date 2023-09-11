#[test]
fn iterator_demonstration() {
  let v1 = vec![1,2,3];
  let mut v1_iter = v1.iter();
  
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
  
}

#[test]
fn iterator_sum() {
  let v2 = vec![1,2,3];
  let mut v2_iter = v2.iter();
  let sum_result:i32 = v2_iter.sum();
  assert_eq!(sum_result, 6);
}