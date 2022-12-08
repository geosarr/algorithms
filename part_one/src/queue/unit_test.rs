#[cfg(test)]
mod tests{
    use super::super::super::*;

    #[test]
    fn test_linked_list_queue_new(){
        let queue = LinkedListQueue::<usize>::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_linked_list_queue_init(){
        let string = "test".to_string();
        let mut queue = LinkedListQueue::init(string.clone());
        assert_eq!(Some(string), queue.dequeue());
    }

    #[test]
    fn test_linked_list_queue_dequeue(){
        let num = -1isize;
        let mut queue = LinkedListQueue::init(num);
        assert_eq!(Some(num), queue.dequeue()); 
        assert_eq!(None, queue.dequeue());
    }

    #[test]
    fn test_linked_list_queue_enqueue(){
        let string1 = "test".to_string();
        let string2 = string1.clone(); 
        let mut queue = LinkedListQueue::new();
        queue.enqueue(string1.clone());
        queue.enqueue(string2.clone());
        assert_eq!(Some(string1.clone()), queue.dequeue());   
        assert_eq!(Some(string2.clone()), queue.dequeue());      
    }
}