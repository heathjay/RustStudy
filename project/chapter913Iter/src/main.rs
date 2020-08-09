fn main() {
    //该map方法使用闭包来调用每个元素以生成新的迭代器，这里的闭包创建了一个新的迭代器，对其ector中的每个元素都被加上1
    let v1 : Vec<i32> = vec![1,2,3];
    let v2 : Vec<_> = v1.iter().map(|x| x+ 1).collect();
    assert_eq!(v2, vec![2,3,4]);
    
}
