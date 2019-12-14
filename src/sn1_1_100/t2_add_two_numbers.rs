///Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

/// 自写
/// 步骤：·转为数字->求和->转为链表
/// 此方案只可解决数字大小不超过u128的最大值的情况
/// 注意：即使u128也不够题目要求，还是会溢出导致结果错误。
/// 本题考查的就是链表保存的没有长度限制的超长数字求和的问题,所以
pub fn add_two_numbers1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //1 获取l1和l2的数字
    let (mut a, mut l1_num, mut l2_num) = (1u128, 0u128, 0u128);
    let (mut l1, mut l2) = (l1, l2);
    while let Some(ref l) = l1 {
        l1_num += (l.val as u128 * a);
        a *= 10;
        l1 = l1.unwrap().next;
    }
    a = 1;
    while let Some(ref l) = l2 {
        l2_num += (l.val as u128 * a);
        a *= 10;
        l2 = l2.unwrap().next;
    }
    println!("l1_num:{} l2_num:{}", l1_num, l2_num);

    //2 生成新的数字链表
    let mut sum = l1_num + l2_num;
    if sum < 10 {
        return Some(Box::new(ListNode::new(sum as i32)));
    }
    let num = sum % 10;
    let mut result = ListNode::new(num as i32);
    sum /= 10;
    let mut result_vec = vec![result];
    while sum > 0 {
        println!("sum {}", sum);
        let num = sum % 10;
        println!("num {}", num);
        let next = ListNode::new(num as i32);
        result_vec.push(next);
        sum /= 10;
    }
    let mut temp = None;
    //从尾向头的顺序创造链表，后一个节点的所有权总是属于前一个结点。
    for mut i in result_vec.into_iter().rev() {
        if temp == None {
            temp = Some(Box::new(i));
            continue;
        }
        i.next = temp;//转移所有权
        temp = Some(Box::new(i));
    }
    temp
}

/// 官网题解
/// https://leetcode-cn.com/problems/add-two-numbers/solution/rust-4ms-22mb-by-downtimecc/
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut head = Some(Box::new(ListNode::new(0)));
    //mut tail表示tail这个变量可变  &mut head表示head的可变引用
    let mut tail = &mut head;//tail永远指向最后一个节点的可变借用
    let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);//overflow 溢出进位
    loop {
        let lhs = match l1 {
            Some(node) => {
                l1 = node.next;
                node.val
            }
            None => {
                l1_end = true;
                0
            }
        };
        let rhs = match l2 {
            Some(node) => {
                l2 = node.next;
                node.val
            }
            None => {
                l2_end = true;
                0
            }
        };
        if l1_end && l2_end && !overflow {
            break head.unwrap().next;// break可以用于终止循环、终止到指定标签、终止并返回其后的值作为循环表达式的值
        }
        let sum = lhs + rhs + if overflow { 1 } else { 0 };
        let sum = if sum >= 10 {
            overflow = true;
            sum - 10
        } else {
            overflow = false;
            sum
        };
        //从头向尾的顺序创造链表，后一个节点的所有权总是属于前一个结点。
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
        tail = &mut tail.as_mut().unwrap().next;
    }
}

#[test]
fn test1() {
    let mut x = add_two_numbers1(Some(Box::new(ListNode::new(8))), Some(Box::new(ListNode::new(8))));
    while let Some(s) = x {
        println!("{}", s.val);
        x = s.next;
    }
}

#[test]
fn test() {
    let mut x = add_two_numbers(Some(Box::new(ListNode::new(8))), Some(Box::new(ListNode::new(8))));
    while let Some(s) = x {
        println!("{}", s.val);
        x = s.next;
    }
}