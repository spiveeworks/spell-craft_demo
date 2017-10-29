use std::cmp;
use std::collections::binary_heap;

use units;
use entity_heap;

pub trait Event {
    fn invoke(
        self: Self,
        space: &mut entity_heap::EntityHeap,
        time: &mut EventQueue,
    );
}

// polymorphise Event monomorphisms,
//   since by-value makes more sense to write,
//   and potentially allows implementors to use their own code
//   outside of event contexts
trait PolyEvent: Event {
    fn invoke_box(
        self: Box<Self>,
        space: &mut entity_heap::EntityHeap,
        time: &mut EventQueue,
    );
}

// note the resemlence to FnBox
impl<T> PolyEvent for T
    where T: Event
{
    fn invoke_box(
        self: Box<Self>,
        space: &mut entity_heap::EntityHeap,
        time: &mut EventQueue,
    ) {
        self.invoke(space, time);
    }
}


struct QueueElement {
    execute_time: units::Time,
    call_back: Box<PolyEvent>,
}

impl PartialEq for QueueElement {
    fn eq(
        self: &QueueElement,
        other: &QueueElement
    ) -> bool {
        self.execute_time == other.execute_time
    }
}

impl Eq for QueueElement {
}

impl PartialOrd for QueueElement {
    fn partial_cmp(
        self: &QueueElement,
        other: &QueueElement
    ) -> Option<cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for QueueElement {
    fn cmp(
        self: &QueueElement,
        other: &QueueElement
    ) -> cmp::Ordering {
        use std::cmp::Ordering::*;
        match Ord::cmp(&self.execute_time, &other.execute_time) {
            Less => Greater,  // lower time = higher priority
            Equal => Equal,
            Greater => Less,
        }
    }
}

pub struct EventQueue {
    current_time: units::Time,
    queue: binary_heap::BinaryHeap<QueueElement>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            current_time: 0,
            queue: binary_heap::BinaryHeap::new(),
        }
    }

    pub fn now(&self) -> units::Time {
        self.current_time
    }

    pub fn next(&self) -> Option<units::Time> {
        self.queue
            .peek()
            .map(|qe| qe.execute_time)
    }

    pub fn invoke_next(&mut self, space: &mut entity_heap::EntityHeap) {
        let element =
            if let Some(next) = self.queue.peek_mut() {
                if next.execute_time > self.current_time {
                    self.current_time = next.execute_time;
                }
                binary_heap::PeekMut::pop(next)
            } else {
                return;
            };
        element.call_back
               .invoke_box(space, self);
    }

    pub fn simulate(
        &mut self,
        space: &mut entity_heap::EntityHeap,
        until: units::Time
    ) {
        while let Some(next_time) = self.next() {
            if next_time <= until {
                self.invoke_next(space);
            } else {
                break;
            }
        }
        self.current_time = until;
    }

    pub fn enqueue<E>(&mut self, event: E, delay: units::Duration)
        where E: 'static + Event
    {
        let element = QueueElement {
            execute_time: self.current_time + delay,
            call_back: Box::new(event),
        };
        self.queue.push(element);
    }
}

