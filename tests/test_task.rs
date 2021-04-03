use std::collections::BinaryHeap;
use xplr::*;

#[test]
fn test_task_priority() {
    let task1 = app::Task::new(2, app::MsgIn::External(app::ExternalMsg::Refresh), None);
    let task2 = app::Task::new(2, app::MsgIn::External(app::ExternalMsg::Refresh), None);
    let task3 = app::Task::new(1, app::MsgIn::External(app::ExternalMsg::Refresh), None);
    let task4 = app::Task::new(1, app::MsgIn::External(app::ExternalMsg::Refresh), None);
    let task5 = app::Task::new(3, app::MsgIn::External(app::ExternalMsg::Refresh), None);
    let task6 = app::Task::new(3, app::MsgIn::External(app::ExternalMsg::Refresh), None);

    let mut heap = BinaryHeap::new();

    heap.push(task1.clone());
    heap.push(task2.clone());
    heap.push(task3.clone());
    heap.push(task4.clone());
    heap.push(task5.clone());
    heap.push(task6.clone());

    assert_eq!(heap.pop(), Some(task3));
    assert_eq!(heap.pop(), Some(task4));
    assert_eq!(heap.pop(), Some(task1));
    assert_eq!(heap.pop(), Some(task2));
    assert_eq!(heap.pop(), Some(task5));
    assert_eq!(heap.pop(), Some(task6));
    assert_eq!(heap.pop(), None);
}
