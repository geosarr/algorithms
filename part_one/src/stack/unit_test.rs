#[cfg(test)]
mod tests{
    use super::super::super::*;

    fn unbox<T>(value: Box<T>) -> T {
        *value
    }

    #[test]
    fn test_linked_list_stack_str_init(){
        let string = "test".to_string();
        let stack = LinkedListStackOfString::init(string.clone());
        assert_eq!(string, unbox(stack.first.unwrap()).item);
        assert_eq!(1, stack.len);
    }

    #[test]
    #[should_panic]
    fn test_linked_list_stack_str_pop(){
        let string = "test".to_string();
        let mut stack = LinkedListStackOfString::init(string.clone());
        assert_eq!(Some(string.clone()), stack.pop()); 
        assert_eq!(0, stack.len); 
        stack.pop();
    }

    #[test]
    fn test_linked_list_stack_str_push(){
        let string = "test".to_string();
        let mut stack = LinkedListStackOfString::new();
        assert_eq!(0, stack.len); 
        stack.push(string.clone());
        assert_eq!(1, stack.len); 
        assert_eq!(Some(string.clone()), stack.pop());        
    }

    #[test]
    fn test_linked_list_stack_str_is_empty(){
        let mut stack = LinkedListStackOfString::new();
        assert!(stack.is_empty());
        stack.push("test".to_string());
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_linked_list_stack_new(){
        let stack = LinkedListStack::<usize>::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_linked_list_stack_init(){
        let string = "test".to_string();
        let mut stack = LinkedListStack::init(string.clone());
        assert_eq!(Some(string), stack.pop());
    }

    #[test]
    fn test_linked_list_stack_pop(){
        let num = -1isize;
        let mut stack = LinkedListStack::init(num);
        assert_eq!(Some(num), stack.pop()); 
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_linked_list_stack_push(){
        let string1 = "test".to_string();
        let string2 = string1.clone(); 
        let mut stack = LinkedListStack::new();
        stack.push(string1.clone());
        stack.push(string2.clone());
        assert_eq!(Some(string1.clone()), stack.pop());   
        assert_eq!(Some(string2.clone()), stack.pop());      
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
        let mut stack = VecStack::<String>::init(1);
        stack.push(string.clone());
        assert_eq!(Some(string.clone()), stack.pop()); 
        stack.pop();
    }

    #[test]
    fn test_vec_push(){
        let string = "test".to_string();
        let mut stack = VecStack::<String>::new();
        stack.push(string.clone());
        assert_eq!(Some(string.clone()), stack.pop()); 
    }

    #[test]
    fn test_vec_is_empty(){
        let mut stack = VecStack::<usize>::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
    }

}