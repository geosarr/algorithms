#[cfg(test)]
mod tests{
    use super::super::super::*;

    fn unbox<T>(value: Box<T>) -> T {
        *value
    }

    #[test]
    fn test_linked_list_queue_init(){
        let string = "test".to_string();
        let queue = LinkedListQueue::init(string.clone());
        assert_eq!(string, unbox(queue.first.unwrap()).item);
    }

    #[test]
    #[should_panic]
    fn test_linked_list_queue_dequeue(){
        let string = "test".to_string();
        let mut queue = LinkedListQueue::init(string.clone());
        assert_eq!(Some(string.clone()), queue.dequeue()); 
        queue.dequeue();
    }

    // #[test]
    // fn test_linked_list_queue_enqueue(){
    //     let string = "test".to_string();
    //     let mut queue = LinkedListQueue::new();
    //     queue.enqueue(string.clone());
    //     assert_eq!(Some(string.clone()), queue.dequeue());        
    // }

    // #[test]
    // fn test_linked_list_queue_is_empty(){
    //     let mut queue = LinkedListQueue::new();
    //     assert!(queue.is_empty());
    //     queue.enqueue("test".to_string());
    //     assert!(!queue.is_empty());
    // }


    // #[test]
    // #[should_panic]
    // fn test_vec_queue_init(){
    //     let n = 50;
    //     let queue = Vecqueue::<String>::init(50);
    //     let none: Option<String> = None;
    //     assert_eq!(queue.vec.len(), n);
    //     assert_eq!(false, queue.vec.iter().any(|e| *e != none));
    //     Vecqueue::<isize>::init(0);
    // }

    // #[test]
    // #[should_panic]
    // fn test_vec_dequeue(){
    //     let string = "test".to_string();
    //     let mut queue = LinkedListQueue::init(string.clone());
    //     assert_eq!(string.clone(), queue.dequeue()); 
    //     queue.dequeue();
    // }

    // #[test]
    // fn test_vec_enqueue(){
    //     let string = "test".to_string();
    //     let mut queue = Vecqueue::<String>::new();
    //     queue.enqueue(string.clone());
    //     assert_eq!(string.clone(), queue.dequeue().unwrap());        
    // }

    // #[test]
    // fn test_vec_is_empty(){
    //     let mut queue = Vecqueue::<usize>::new();
    //     assert!(queue.is_empty());
    //     queue.enqueue(1);
    //     assert!(!queue.is_empty());
    // }

}