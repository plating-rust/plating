/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

/*mod lib {
    use std::{collections::VecDeque, marker::PhantomData, pin::Pin, task::{Context, Poll, RawWaker, Waker}};

    pub use tuple_list::{tuple_list as event_handler_list, TupleList as EventHandlerList};
    use plating::PlatingResult;
    use thiserror::Error;

    //////////////////////////
    // Waker
    //////////////////////////
    unsafe fn queue_waker_clone(data: *const ()) -> std::task::RawWaker {
        std::task::RawWaker::new(data, &QUEU_WAKER_VTABLE)
    }

    unsafe fn wake(_data: *const ()) {
        //push
    }

    unsafe fn wake_by_ref(_data: *const ()) {
        //push
    }

    unsafe fn drop(_data: *const ()) {
        //push
    }

    const QUEU_WAKER_VTABLE: std::task::RawWakerVTable = std::task::RawWakerVTable::new(queue_waker_clone, wake, wake_by_ref, drop);

    const fn queue_raw_waker() -> RawWaker {
        std::task::RawWaker::new(std::ptr::null(), &QUEU_WAKER_VTABLE)
    }

    #[inline]
    fn queue_waker() -> Waker {
        unsafe { Waker::from_raw(queue_raw_waker()) }
    }


    pub struct KeyEvent {
        d1: u8,
    }
    #[derive(Error, Debug)]
    pub enum EventHandleError {
        #[error("Can only handle one event at a time")]
        OnlyOneEvent,
        #[error("No waker available")]
        NoWaker,
    }

    pub struct EventStream<T> {
        //queue of events
        waker: Option<std::task::Waker>,
        event: Option<T>,
        closed: bool,
    }
    impl<T> Unpin for EventStream<T> {}

    impl<T> EventStream<T> {
        pub fn new() -> Self {
            Self {
                waker: None,
                event: None,
                closed: false
            }
        }
        pub(crate) fn handle(&mut self, event: T) -> PlatingResult<()> {
            if let Some(_) = &self.event {
                Err(EventHandleError::OnlyOneEvent.into())
            } else {
                self.event = Some(event);

                if let Some(w) = &self.waker {
                    w.wake_by_ref();
                    Ok(())
                } else {
                    Err(EventHandleError::NoWaker.into())
                }
            }
        }
    }

    impl<T> std::stream::Stream for EventStream<T> {
        type Item = T;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if self.closed {
                self.waker = None;
                Poll::Ready(None)
            } else {
                if let Some(waker) = &self.waker {
                    // Check if the stored waker matches the current task's waker.
                    // This is necessary as the `Delay` future instance may move to
                    // a different task between calls to `poll`. If this happens, the
                    // waker contained by the given `Context` will differ and we
                    // must update our stored waker to reflect this change.
                    if !waker.will_wake(cx.waker()) {
                        self.waker = Some(cx.waker().clone());
                    }
                } else {
                    self.waker = Some(cx.waker().clone());
                }

                if let Some(ev) = self.event.take() {
                    Poll::Ready(Some(ev))
                } else {
                    Poll::Pending
                }

            }
        }

    }

    struct LoopData {
        stream: EventStream<KeyEvent>,
        event: KeyEvent,
    }
    pub struct Loop {
        events: VecDeque<LoopData>,
        //next_events: VecDeque<>;

        tasks: VecDeque<Pin<Box<dyn std::future::Future<Output = ()>>>>,
    }
    impl Loop {
        fn tick(&self) {
            unimplemented!{}
            /*let mut event_stream = EventStream::<Event>::new();
            let event = KeyEvent{ d1: 8};

            event_stream.handle(event);

            loop {

                while let Some(mut task) = self.tasks.pop_front() {
                    if task.as_mut().poll(&mut cx).is_pending() {
                        self.tasks.push_back(task);
                    }
                }

                if self.events.is_empty() {
                    break;
                }
            }*/
        }
    }

    pub struct CocoaKeyEventData {
        d1: u8,
        d2: bool,
    }

    pub trait CocoaEvent {
        fn d1(&self) -> u8;
        fn d2(&self) -> bool;
    }

    impl CocoaEvent for CocoaKeyEventData {
        fn d1(&self) -> u8 {
            self.d1
        }
        fn d2(&self) -> bool {
            self.d2
        }
    }

    pub trait WidgetEvent {
        fn d2(&self) -> u32;
    }

    impl WidgetEvent for CocoaKeyEventData {
        fn d2(&self) -> u32 {
            self.d2 as u32
        }
    }

    pub struct CocoaWidgetStruct<EventHandler>
    where
        EventHandler: EventHandlerList,
    {
        _eh: PhantomData<EventHandler>
    }

    pub trait CocoaWidget {
        fn new<T: FnOnce()>() -> Self;
    }

    impl<EventHandler> CocoaWidget for CocoaWidgetStruct<EventHandler>
    where
        EventHandler: EventHandlerList,
    {
        fn new<T: FnOnce()>() -> Self {

            CocoaWidgetStruct {
                _eh: PhantomData::default(),
            }
        }
    }
}*/

pub fn main() -> plating::PlatingResult<()> {
    Ok(())
}
