#[cfg(test)]
mod tests{
    use super::super::*;

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