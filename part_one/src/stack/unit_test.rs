#[cfg(test)]
mod tests{
    use super::super::super::*;

    fn unbox<T>(value: Box<T>) -> T {
        *value
    }

    #[test]
    fn test_linked_list_stack_init(){
        let string = "test".to_string();
        let stack = LinkedListStack::init(string.clone());
        assert_eq!(string, unbox(stack.first.unwrap()).item);
    }

    #[test]
    #[should_panic]
    fn test_linked_list_stack_pop(){
        let string = "test".to_string();
        let mut stack = LinkedListStack::init(string.clone());
        assert_eq!(string.clone(), stack.pop()); 
        stack.pop();
    }

    #[test]
    fn test_linked_list_stack_push(){
        let string = "test".to_string();
        let mut stack = LinkedListStack::new();
        stack.push(string.clone());
        assert_eq!(string.clone(), stack.pop());        
    }

    #[test]
    fn test_linked_list_stack_is_empty(){
        let mut stack = LinkedListStack::new();
        assert!(stack.is_empty());
        stack.push("test".to_string());
        assert!(!stack.is_empty());
    }


    #[test]
    #[should_panic]
    fn test_vec_stack_init(){
        let n = 50;
        let stack = VecStack::<String>::init(50);
        let none: Option<String> = None;
        assert_eq!(stack.vec.len(), n);
        assert_eq!(false, stack.vec.iter().any(|e| *e != none));
        VecStack::<isize>::init(0);
    }

    #[test]
    #[should_panic]
    fn test_vec_pop(){
        let string = "test".to_string();
        let mut stack = LinkedListStack::init(string.clone());
        assert_eq!(string.clone(), stack.pop()); 
        stack.pop();
    }

    #[test]
    fn test_vec_push(){
        let string = "test".to_string();
        let mut stack = VecStack::<String>::new();
        stack.push(string.clone());
        assert_eq!(string.clone(), stack.pop().unwrap());        
    }

    #[test]
    fn test_vec_is_empty(){
        let mut stack = VecStack::<usize>::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
    }

}