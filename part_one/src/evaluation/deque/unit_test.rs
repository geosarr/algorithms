#[cfg(test)]
mod tests{
    use super::super::*;

    #[test]
    fn test_linked_list_deque_new(){
        let deque = LinkedListDeque::<usize>::new();
        assert!(deque.is_empty());
        assert_eq!(deque.size(), 0);
    }

    #[test]
    fn test_linked_list_deque_init(){
        let string = "test".to_string();
        let mut deque = LinkedListDeque::init(string.clone());
        assert_eq!(1, deque.size());
        assert_eq!(Some(string), deque.remove_first());
        assert_eq!(0, deque.size());
    }

    #[test]
    fn test_linked_list_deque_mutations(){
        let num1 = -1isize;
        let num2 = -2isize;
        let num3 = -3isize;
        let num4 = -4isize;
        let mut deque = LinkedListDeque::new();
        deque.add_first(num1);
        deque.add_last(num2);
        deque.add_first(num3);
        deque.add_last(num4);
        assert_eq!(Some(num3), deque.remove_first()); 
        assert_eq!(Some(num4), deque.remove_last());
        assert_eq!(Some(num2), deque.remove_last());
        assert_eq!(Some(num1), deque.remove_first());
    }
}