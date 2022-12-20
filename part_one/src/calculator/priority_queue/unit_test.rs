#[cfg(test)]
mod tests{
    use super::super::*;
    use rand::Rng;

    #[test]
    fn test_binary_heap_priority_queue(){
        let len = 10000000; 
        let mut rng = rand::thread_rng();
        let mut queue = BinaryHeapPriorityQueue::<isize>::init(len);
        assert!(queue.is_empty());
        for k in 0..len{
            queue.insert(rng.gen::<isize>());
        }
        assert_eq!(queue.len(), len);
        assert_eq!(queue.heap.peek().unwrap().clone(), queue.heap.pop().unwrap());
    }

    #[test]
    #[should_panic]
    fn test_min_priority_queue(){
        let len = 1000000; 
        let mut rng = rand::thread_rng();
        let mut queue = PriorityQueue::<isize>::init(2, PriorityQueueKind::Min);
        assert!(queue.is_empty());
        queue.insert(rng.gen::<isize>());
        assert_eq!(queue.len(), 1);
        queue.insert(rng.gen::<isize>());
        assert_eq!(queue.vec.len(), 6);
        for k in 0..len-2{
            queue.insert(rng.gen::<isize>());
        }
        queue.delete();
        assert_eq!(queue.len(), len);
        assert_eq!(queue.extremum(), queue.vec[1]);
        for k in 1..(queue.len()-1)/2{
            assert!(
                queue.vec[k] <= queue.vec[2*k] && 
                queue.vec[k] <= queue.vec[2*k+1]
            );
        }
        let queue = PriorityQueue::<String>::init(0, PriorityQueueKind::Min);
    }

    #[test]
    #[should_panic]
    fn test_max_priority_queue(){
        let len = 1000000; 
        let mut rng = rand::thread_rng();
        let mut queue = PriorityQueue::<isize>::init(2, PriorityQueueKind::Max);
        assert!(queue.is_empty());
        queue.insert(rng.gen::<isize>());
        assert_eq!(queue.len(), 1);
        queue.insert(rng.gen::<isize>());
        assert_eq!(queue.vec.len(), 6);
        for k in 0..len-2{
            queue.insert(rng.gen::<isize>());
        }
        queue.delete();
        assert_eq!(queue.len(), len);
        assert_eq!(queue.extremum(), queue.vec[1]);
        for k in 1..(queue.len()-1)/2{
            assert!(
                queue.vec[k] >= queue.vec[2*k] && 
                queue.vec[k] >= queue.vec[2*k+1]
            );
        }
        let queue = PriorityQueue::<String>::init(0, PriorityQueueKind::Max);
    }

    #[test]
    #[should_panic]
    fn test_unordered_vec_min_priority_queue(){
        let mut queue = UnorderedVecPriorityQueue::<usize>::init(2, PriorityQueueKind::Min);
        assert!(queue.is_empty());
        assert!(queue.extremum.is_none());
        queue.insert(2);
        assert_eq!(queue.len(), 1);
        queue.insert(10);
        assert_eq!(queue.vec.len(), 4);
        queue.insert(1);
        queue.insert(1);
        queue.delete();
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.extremum(), Some(1));
        let queue = UnorderedVecPriorityQueue::<String>::init(0, PriorityQueueKind::Min);
    }

    #[test]
    #[should_panic]
    fn test_unordered_vec_max_priority_queue(){
        let mut queue = UnorderedVecPriorityQueue::<usize>::init(2, PriorityQueueKind::Max);
        assert!(queue.is_empty());
        assert!(queue.extremum.is_none());
        queue.insert(2);
        assert_eq!(queue.len(), 1);
        queue.insert(10);
        queue.insert(1);
        queue.delete();
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.extremum(), Some(2));
        let queue = UnorderedVecPriorityQueue::<String>::init(0, PriorityQueueKind::Max);
    }
}