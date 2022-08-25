use priority_queue::PriorityQueue;
use std::any::Any;
struct CMap {
    vboxes: PriorityQueue<dyn Any, dyn Any>,
}

impl CMap {
    pub fn palette(&self) -> String {
        // contents contains color
        // contents = {
        //     color:
        // }
        return self.vboxes.map(|x| x["color"]);
    }
}
