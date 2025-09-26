/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

enum StackMode {
    DATA_IN_Q1,
    DATA_IN_Q2 
}
pub struct myStack<T>
{
	//TODO
    mode:StackMode,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            mode:StackMode::DATA_IN_Q1,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        match self.mode {
            StackMode::DATA_IN_Q1 => self.q1.enqueue(elem),
            StackMode::DATA_IN_Q2 => self.q2.enqueue(elem)
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> 
    where 
    T:std::fmt::Display,
    {
        //TODO
        let pop_data = |from:&mut Queue<T>, to:&mut Queue<T>|{
            while from.size() > 1 {
                if let Ok(value) = from.dequeue() {
                    to.enqueue(value);
                } 
            }
            from.dequeue().map_err(|_|{"Stack is empty"})
        };
        
        let mut ret = Err("Stack is empty");
        match self.mode {
            StackMode::DATA_IN_Q1 => {
                ret = pop_data(&mut self.q1, &mut self.q2);
                self.mode= StackMode::DATA_IN_Q2;
            }
            StackMode::DATA_IN_Q2 => {
                ret = pop_data(&mut self.q2, &mut self.q1);
                self.mode= StackMode::DATA_IN_Q1;
            }
        }
        ret
    }
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}